#!/bin/bash
# KorrinOS Backup & Restore - Full system backup with encryption and scheduling

set -e

have() { command -v "$1" >/dev/null 2>&1; }

BACKUP_DIR="$HOME/.tinker/backups"
CONFIG_FILE="$BACKUP_DIR/config.conf"
LOG_FILE="$BACKUP_DIR/backup.log"
EXCLUDE_FILE="$BACKUP_DIR/exclude.conf"

mkdir -p "$BACKUP_DIR"

init() {
    [ ! -f "$CONFIG_FILE" ] && cat > "$CONFIG_FILE" << 'EOF'
# Backup Configuration
DEFAULT_DESTINATION=local
ENCRYPTION=true
COMPRESSION=zstd
COMPRESSION_LEVEL=3
MAX_BACKUPS=10
AUTO_CLEANUP=true
VERIFY_BACKUPS=true
NOTIFICATIONS=true
EXCLUDE_CACHE=true
EXCLUDE_TRASH=true
EOF

    [ ! -f "$EXCLUDE_FILE" ] && cat > "$EXCLUDE_FILE" << 'EOF'
# Backup Exclude Patterns
.cache
.thumbnails
.Trash
Trash
*.tmp
*.log
node_modules
__pycache__
.git
.vscode
*.iso
*.img
VirtualBox VMs
VMware
EOF

    [ ! -f "$LOG_FILE" ] && touch "$LOG_FILE"

    # Every statement above is a `test && action` pair, so once the files exist
    # the final test evaluates false and the function returned 1. Under `set -e`
    # that aborted the whole script on every run after the first -- the tool
    # created its config and then silently did nothing forever. Always succeed.
    return 0
}

# Populate the argv arrays for the backup pipeline.
# Previously this returned ONE STRING that the caller passed to `eval`, so the
# backup name (argv), the source directory, every line of the exclude file and
# the compression level from the config all became shell code. This now fills
# arrays, so each value stays exactly one argument.
build_tar_cmd() {
    local dest=$1
    local name=$2
    local src=${3:-$HOME}

    local -a excludes=()
    local line
    while IFS= read -r line; do
        [ -z "$line" ] && continue
        [[ "$line" =~ ^[[:space:]]*# ]] && continue
        excludes+=("--exclude=$line")
    done < "$EXCLUDE_FILE"

    local -a encrypt=()
    if grep -q "^ENCRYPTION=true" "$CONFIG_FILE"; then
        encrypt=(gpg --symmetric --cipher-algo AES256)
    fi

    local compress=()
    local comp level
    comp=$(grep "^COMPRESSION=" "$CONFIG_FILE" | cut -d= -f2)
    level=$(grep "^COMPRESSION_LEVEL=" "$CONFIG_FILE" | cut -d= -f2)
    # Only accept a plain integer level; anything else came from a config file
    # and must never be able to inject an extra argument.
    [[ "$level" =~ ^-?[0-9]+$ ]] || level=""

    case $comp in
        zstd) have zstd || return 1; [ -n "$level" ] && compress=(zstd "-$level") || compress=(zstd) ;;
        gzip) [ -n "$level" ] && compress=(gzip "-$level") || compress=(gzip) ;;
        xz)   have xz   || return 1; [ -n "$level" ] && compress=(xz "-$level")   || compress=(xz) ;;
        none|"") compress=(cat) ;;
        *) echo "Unknown COMPRESSION '$comp' in config" >&2; return 1 ;;
    esac

    TAR_ARGS=("$dest/$name")
    TAR_SRC="$src"
    TAR_EXCLUDES=("${excludes[@]}")
    TAR_COMPRESS=("${compress[@]}")
    TAR_ENCRYPT=("${encrypt[@]}")
}

# Create backup
create_backup() {
    local name=${1:-"backup-$(date +%Y%m%d-%H%M%S)"}
    local src=${2:-$HOME}
    local dest=$BACKUP_DIR

    # Same reason as restore_backup: the name reaches a command line.
    if [[ ! "$name" =~ ^[A-Za-z0-9._-]+$ ]]; then
        echo "Invalid backup name: $name"
        echo "Allowed: letters, digits, dot, underscore, hyphen only."
        return 1
    fi
    [ -d "$src" ] || { echo "Source directory not found: $src"; return 1; }

    echo "Creating backup: $name"
    echo "Source: $src"
    echo "Destination: $dest"
    echo ""

    local start=$(date +%s)
    local -a TAR_EXCLUDES=() TAR_COMPRESS=() TAR_ENCRYPT=()
    local TAR_SRC="" TAR_ARGS=()
    if ! build_tar_cmd "$dest" "$name.tar" "$src"; then
        echo "Could not build the backup pipeline (see error above)."
        return 1
    fi

    echo "Running backup..."
    local out="${TAR_ARGS[0]}"
    # pipefail so a failure anywhere in the pipeline (tar OR a compressor stage)
    # is reported. Without it `if <pipeline>` would only see the last stage's
    # status and a tar failure would still look like success.
    #
    # The stages MUST be separate pipeline elements. Appending the gpg args to
    # the compressor ("zstd -3 gpg --symmetric") makes zstd treat "gpg" as an
    # input FILENAME and it aborts with "Incorrect parameters".
    set -o pipefail
    local ok=0
    if [ "${#TAR_ENCRYPT[@]}" -gt 0 ]; then
        tar "${TAR_EXCLUDES[@]}" -C "$TAR_SRC" -cf - . 2>/dev/null \
          | "${TAR_COMPRESS[@]}" | "${TAR_ENCRYPT[@]}" > "$out" && ok=1 || true
    else
        tar "${TAR_EXCLUDES[@]}" -C "$TAR_SRC" -cf - . 2>/dev/null \
          | "${TAR_COMPRESS[@]}" > "$out" && ok=1 || true
    fi
    if [ "$ok" -eq 1 ]; then
        local end=$(date +%s)
        local duration=$((end - start))
        local size=$(du -h "$out" 2>/dev/null | awk '{print $1}' || echo "unknown")

        echo "Backup completed in ${duration}s"
        echo "Size: $size"

        # Verify backup
        if grep -q "^VERIFY_BACKUPS=true" "$CONFIG_FILE"; then
            echo "Verifying backup..."
            if tar -tf "$out" >/dev/null 2>&1; then
                echo "Verification: OK"
            else
                echo "Verification: FAILED"
            fi
        fi
        
        # Log
        echo "$(date +%s)|create|$name|$size|${duration}s|success" >> "$LOG_FILE"
        
        # Cleanup old backups
        if grep -q "AUTO_CLEANUP=true" "$CONFIG_FILE"; then
            cleanup_old
        fi
        
        grep -q "NOTIFICATIONS=true" "$CONFIG_FILE" && notify-send "Backup Complete" "$name ($size)" 2>/dev/null || true
    else
        echo "Backup failed!"
        echo "$(date +%s)|create|$name|0|0|failed" >> "$LOG_FILE"
        return 1
    fi
}

# Restore backup
restore_backup() {
    local name=$1
    local dest=${2:-$HOME}

    # Reject anything that is not a plain backup filename. The old code built
    #   eval "... < $BACKUP_DIR/$name | ... "
    # with $name taken straight from argv, so a name such as
    #   'x.tar; touch /tmp/pwn; #'
    # executed the injected command. Filenames are validated up front now and
    # the pipeline no longer goes through eval at all.
    if [[ ! "$name" =~ ^[A-Za-z0-9._-]+$ ]]; then
        echo "Invalid backup name: $name"
        echo "Allowed: letters, digits, dot, underscore, hyphen only."
        return 1
    fi
    case "$name" in
        *..*) echo "Invalid backup name (no '..'): $name"; return 1 ;;
    esac

    local backup_file="$BACKUP_DIR/$name"

    [ ! -f "$backup_file" ] && backup_file="$BACKUP_DIR/$name.tar"
    [ ! -f "$backup_file" ] && echo "Backup not found: $name" && return 1

    echo "Restoring backup: $name"
    echo "Destination: $dest"
    echo ""
    echo "WARNING: This will overwrite existing files!"
    read -p "Continue? (y/N) " -n 1 -r
    echo ""
    [[ ! $REPLY =~ ^[Yy]$ ]] && echo "Aborted" && return 1

    local start=$(date +%s)

    # Decrypt / decompress as separate, validated steps instead of one eval'd
    # string. Only the literal program names below are ever executed.
    local decrypt_prog=() decompress_prog=()
    if grep -q "^ENCRYPTION=true" "$CONFIG_FILE"; then
        have gpg || { echo "gpg not installed but ENCRYPTION=true"; return 1; }
        decrypt_prog=(gpg --decrypt)
    else
        decrypt_prog=(cat)
    fi

    local comp
    comp=$(grep "^COMPRESSION=" "$CONFIG_FILE" | cut -d= -f2)
    case $comp in
        zstd) have zstd || { echo "zstd not installed"; return 1; }; decompress_prog=(zstd -d) ;;
        gzip) decompress_prog=(gzip -d) ;;
        xz)   have xz || { echo "xz not installed"; return 1; }; decompress_prog=(xz -d) ;;
        none|"") decompress_prog=(cat) ;;
        *) echo "Unknown COMPRESSION '$comp' in config"; return 1 ;;
    esac

    echo "Restoring..."
    if "${decrypt_prog[@]}" < "$backup_file" | "${decompress_prog[@]}" | tar -C "$dest" -xf - ; then
        local end=$(date +%s)
        echo "Restore completed in $((end - start))s"
        echo "$(date +%s)|restore|$name|success" >> "$LOG_FILE"
    else
        echo "Restore failed!"
        echo "$(date +%s)|restore|$name|failed" >> "$LOG_FILE"
        return 1
    fi
}

# List backups
list_backups() {
    echo "Available Backups:"
    echo ""
    ls -lh "$BACKUP_DIR"/*.tar 2>/dev/null | while read line; do
        local name=$(echo "$line" | awk '{print $9}')
        local size=$(echo "$line" | awk '{print $5}')
        local date=$(echo "$line" | awk '{print $6, $7, $8}')
        local base=$(basename "$name" .tar)
        echo "  $base ($size) - $date"
    done || echo "  No backups found"
}

# Cleanup old backups
cleanup_old() {
    local max=$(grep "^MAX_BACKUPS=" "$CONFIG_FILE" | cut -d= -f2)
    max=${max:-10}
    
    local count=$(ls -1 "$BACKUP_DIR"/*.tar 2>/dev/null | wc -l)
    if [ $count -gt $max ]; then
        local to_remove=$((count - max))
        ls -t "$BACKUP_DIR"/*.tar | tail -$to_remove | while read f; do
            echo "Removing old backup: $(basename $f)"
            rm -f "$f"
        done
    fi
}

# Schedule backups
schedule() {
    local freq=${1:-daily}
    local time=${2:-02:00}
    local src=${3:-$HOME}
    
    echo "Scheduling backups: $freq at $time"
    
    cat > /etc/systemd/system/tinker-backup.service << EOF
[Unit]
Description=KorrinOS Backup
After=network-online.target

[Service]
Type=oneshot
ExecStart=/home/tinkerspace/linux-kernel/os/apps/system/backup-restore.sh create scheduled $src
Environment=HOME=/home/tinkerspace
EOF

    local timer_spec=""
    case $freq in
        hourly) timer_spec="OnCalendar=*:00" ;;
        daily) timer_spec="OnCalendar=*-*-* $time" ;;
        weekly) timer_spec="OnCalendar=Mon *-*-* $time" ;;
        monthly) timer_spec="OnCalendar=*-1 $time" ;;
    esac
    
    cat > /etc/systemd/system/tinker-backup.timer << EOF
[Unit]
Description=Run KorrinOS Backup $freq

[Timer]
$timer_spec
Persistent=true

[Install]
WantedBy=timers.target
EOF
    
    systemctl daemon-reload 2>/dev/null || true
    systemctl enable --now tinker-backup.timer 2>/dev/null || true
    
    echo "Backup timer created and enabled"
}

show_help() {
    echo "Usage: tinker-backup [command]"
    echo ""
    echo "Commands:"
    echo "  create [name] [src]   Create backup (default: timestamp, \$HOME)"
    echo "  restore <name> [dest] Restore backup (default: \$HOME)"
    echo "  list                  List available backups"
    echo "  cleanup               Remove old backups"
    echo "  schedule [freq] [time] [src]  Schedule automatic backups"
    echo "  verify <name>         Verify backup integrity"
    echo "  help                  Show this help"
    echo ""
    echo "Frequencies: hourly, daily, weekly, monthly"
}

init

case "$1" in
    create) create_backup "$2" "$3" ;;
    restore) restore_backup "$2" "$3" ;;
    list) list_backups ;;
    cleanup) cleanup_old ;;
    schedule) schedule "$2" "$3" "$4" ;;
    verify) tar -tf "$BACKUP_DIR/$2.tar" >/dev/null 2>&1 && echo "OK" || echo "FAILED" ;;
    *) show_help ;;
esac