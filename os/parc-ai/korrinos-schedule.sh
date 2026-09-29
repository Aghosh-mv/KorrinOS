#!/usr/bin/env bash
# korrinos-schedule.sh — Task Scheduler & Cron Manager
# Schedule tasks, manage cron jobs, reminders, recurring tasks

set -euo pipefail

SCHEDULE_DIR="${HOME}/.config/korrinos/schedule"
SCHEDULE_CONFIG="$SCHEDULE_DIR/config.json"
SCHEDULE_LOG="$SCHEDULE_DIR/schedule.log"
SCHEDULE_TASKS="$SCHEDULE_DIR/tasks.json"
mkdir -p "$SCHEDULE_DIR"

# Default config
init_schedule() {
  if [ ! -f "$SCHEDULE_CONFIG" ]; then
    cat > "$SCHEDULE_CONFIG" << 'DEFAULTS'
{
  "default_shell": "/bin/bash",
  "notify_on_complete": true,
  "log_output": true,
  "max_retries": 3,
  "retry_delay_seconds": 60
}
DEFAULTS
    echo "Schedule config initialized"
  fi
  
  [ -f "$SCHEDULE_TASKS" ] || echo '[]' > "$SCHEDULE_TASKS"
}

# Add task
cmd_add() {
  local name="$1"
  local command="$2"
  local schedule="${3:-once}"
  local time="${4:-now}"
  
  local id
  id="task_$(date +%s)_$$"
  local created
  created=$(date -Iseconds)
  
  export ID="${id:-}" NAME="${name:-}" COMMAND="${command:-}" \
         SCHEDULE="${schedule:-}" TIME="${time:-}" CREATED="${created:-}" \
         NOW_ISO="$(date -Iseconds)"
  python3 -c "
import json, os
with open('$SCHEDULE_TASKS') as f:
    tasks = json.load(f)

task = {
    'id': os.environ['ID'],
    'name': os.environ['NAME'],
    'command': os.environ['COMMAND'],
    'schedule': os.environ['SCHEDULE'],
    'time': os.environ['TIME'],
    'created': os.environ['CREATED'],
    'enabled': True,
    'last_run': None,
    'next_run': None,
    'run_count': 0,
    'status': 'pending'
}

# Calculate next run time
from datetime import datetime, timedelta
now = datetime.now()
if os.environ['SCHEDULE'] == 'once':
    task['next_run'] = os.environ['TIME']
elif os.environ['SCHEDULE'] == 'hourly':
    task['next_run'] = (now + timedelta(hours=1)).isoformat()
elif os.environ['SCHEDULE'] == 'daily':
    task['next_run'] = (now + timedelta(days=1)).isoformat()
elif os.environ['SCHEDULE'] == 'weekly':
    task['next_run'] = (now + timedelta(weeks=1)).isoformat()
elif os.environ['SCHEDULE'] == 'monthly':
    task['next_run'] = (now + timedelta(days=30)).isoformat()

tasks.append(task)

with open('$SCHEDULE_TASKS', 'w') as f:
    json.dump(tasks, f, indent=2)

print(f'Added task: {\"$name\"} (ID: {\"$id\"})')
print(f'Schedule: {\"$schedule\"} | Next: {task[\"next_run\"]}')
"
  
  # Add to cron if recurring
  if [ "$schedule" != "once" ]; then
    local cron_line=""
    case "$schedule" in
      hourly)  cron_line="0 * * * * $command" ;;
      daily)   cron_line="0 9 * * * $command" ;;
      weekly)  cron_line="0 9 * * 1 $command" ;;
      monthly) cron_line="0 9 1 * * $command" ;;
    esac
    
    if [ -n "$cron_line" ]; then
      (crontab -l 2>/dev/null; echo "$cron_line") | crontab - 2>/dev/null
      echo "Added to crontab: ${cron_line}"
    fi
  fi
}

# List tasks
cmd_list() {
  echo "=== KorrinOS Task Scheduler ==="
  echo ""
  
  python3 -c "
import json, os
with open('$SCHEDULE_TASKS') as f:
    tasks = json.load(f)

if not tasks:
    print('  No scheduled tasks')
else:
    for t in tasks:
        # next_run is None until the task has been scheduled, so it must be
        # coerced before slicing. The old code did t.get('next_run','N/A')[:19]
        # which raised TypeError on None, and because stderr was discarded the
        # 'list' command printed nothing at all.
        nr = t.get('next_run') or 'N/A'
        if not isinstance(nr, str):
            nr = str(nr)
        nr = nr[:19]
        # the enabled marker used to be '' in BOTH branches, so a disabled task
        # looked identical to an enabled one
        status = '[on] ' if t.get('enabled', True) else '[off]'
        name = t.get('name', 'unnamed')
        schedule = t.get('schedule', 'once')
        runs = t.get('run_count', 0) or 0
        print(f'  {status}{name}')
        print(f'    Schedule: {schedule} | Runs: {runs}')
        print(f'    Next: {nr}')
        print(f'    Command: {(t.get(\"command\") or \"\")[:60]}')
        print()
" 2>/dev/null
}

# Run task
cmd_run() {
  local task_id="$1"
  
  export TASK_ID="${task_id:-}" NOW_ISO="$(date -Iseconds)"
  python3 -c "
import json, os
with open('$SCHEDULE_TASKS') as f:
    tasks = json.load(f)

for t in tasks:
    if t['id'] == os.environ['TASK_ID']:
        print(f'Running: {t[\"name\"]}')
        print(f'Command: {t[\"command\"]}')
        t['last_run'] = '$(date -Iseconds)'
        t['run_count'] = t.get('run_count', 0) + 1
        t['status'] = 'running'
        break

with open('$SCHEDULE_TASKS', 'w') as f:
    json.dump(tasks, f, indent=2)
" 2>/dev/null
  
  # Execute the command
  local command
  command=$(python3 -c "
import json, os
with open('$SCHEDULE_TASKS') as f:
    tasks = json.load(f)
for t in tasks:
    if t['id'] == os.environ['TASK_ID']:
        print(t['command'])
        break
" 2>/dev/null)
  
  if [ -n "$command" ]; then
    echo "Executing: ${command}"
    eval "$command" 2>&1 | tee -a "$SCHEDULE_LOG"
    
    # Update status
    python3 -c "
import json, os
with open('$SCHEDULE_TASKS') as f:
    tasks = json.load(f)
for t in tasks:
    if t['id'] == os.environ['TASK_ID']:
        t['status'] = 'completed'
        break
with open('$SCHEDULE_TASKS', 'w') as f:
    json.dump(tasks, f, indent=2)
" 2>/dev/null
    
    echo ""
    echo " Task completed"
  fi
}

# Delete task
cmd_delete() {
  local task_id="$1"
  
  export TASK_ID="${task_id:-}"
  python3 -c "
import json, os
with open('$SCHEDULE_TASKS') as f:
    tasks = json.load(f)

tasks = [t for t in tasks if t['id'] != os.environ['TASK_ID']]

with open('$SCHEDULE_TASKS', 'w') as f:
    json.dump(tasks, f, indent=2)

print(f'Deleted task: $task_id')
"
}

# Enable/disable task
cmd_toggle() {
  local task_id="$1"
  
  export TASK_ID="${task_id:-}"
  python3 -c "
import json, os
with open('$SCHEDULE_TASKS') as f:
    tasks = json.load(f)

for t in tasks:
    if t['id'] == os.environ['TASK_ID']:
        t['enabled'] = not t.get('enabled', True)
        status = 'enabled' if t['enabled'] else 'disabled'
        print(f'Task {t[\"name\"]}: {status}')
        break

with open('$SCHEDULE_TASKS', 'w') as f:
    json.dump(tasks, f, indent=2)
"
}

# Quick reminder
cmd_remind() {
  local message="$1"
  local time="$2"
  
  local id
  id="remind_$(date +%s)_$$"
  
  export TASK_ID="${id:-}" MESSAGE="${message:-}" TIME="${time:-}" \
         NOW_ISO="$(date -Iseconds)"
  python3 -c "
import json, os
from datetime import datetime, timedelta

with open('$SCHEDULE_TASKS') as f:
    tasks = json.load(f)

# Parse time offset
time_str = os.environ['TIME']
if time_str.endswith('m'):
    minutes = int(time_str[:-1])
    next_run = (datetime.now() + timedelta(minutes=minutes)).isoformat()
elif time_str.endswith('h'):
    hours = int(time_str[:-1])
    next_run = (datetime.now() + timedelta(hours=hours)).isoformat()
elif time_str.endswith('d'):
    days = int(time_str[:-1])
    next_run = (datetime.now() + timedelta(days=days)).isoformat()
else:
    next_run = time_str

task = {
    'id': os.environ['ID'],
    'name': 'Reminder: $message',
    'command': 'echo \"Reminder: $message\" | xmessage -file -',
    'schedule': 'once',
    'time': next_run,
    'created': '$(date -Iseconds)',
    'enabled': True,
    'last_run': None,
    'next_run': next_run,
    'run_count': 0,
    'status': 'pending'
}

tasks.append(task)

with open('$SCHEDULE_TASKS', 'w') as f:
    json.dump(tasks, f, indent=2)

print(f'Reminder set: $message')
print(f'Time: {next_run}')
"
}

# Show schedule log
cmd_log() {
  echo "=== Schedule Log ==="
  echo ""
  
  if [ -f "$SCHEDULE_LOG" ]; then
    tail -30 "$SCHEDULE_LOG" | sed 's/^/  /'
  else
    echo "  No schedule log"
  fi
}

# Check cron jobs
cmd_cron() {
  echo "=== Current Cron Jobs ==="
  echo ""
  crontab -l 2>/dev/null | sed 's/^/  /' || echo "  No cron jobs"
}

case "${1:-help}" in
  init)          init_schedule ;;
  add)           shift; cmd_add "$@" ;;
  list)          cmd_list ;;
  run)           shift; cmd_run "$@" ;;
  delete)        shift; cmd_delete "$@" ;;
  toggle)        shift; cmd_toggle "$@" ;;
  remind)        shift; cmd_remind "$@" ;;
  log)           cmd_log ;;
  cron)          cmd_cron ;;
  *)
    echo "KorrinOS Task Scheduler & Cron Manager"
    echo "Usage: korrinos-schedule.sh <command>"
    echo ""
    echo "Commands:"
    echo "  init              Initialize scheduler config"
    echo "  add <name> <cmd> [schedule] [time]  Add task"
    echo "                     Schedule: once|hourly|daily|weekly|monthly"
    echo "  list              List all scheduled tasks"
    echo "  run <task-id>     Run a task immediately"
    echo "  delete <task-id>  Delete a task"
    echo "  toggle <task-id>  Enable/disable a task"
    echo "  remind <msg> <time>  Set a reminder (e.g., 30m, 2h, 1d)"
    echo "  log               Show schedule log"
    echo "  cron              Show current cron jobs"
    ;;
esac
