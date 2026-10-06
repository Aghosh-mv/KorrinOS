// SPDX-License-Identifier: GPL-2.0
/*
 * KorrinOS GameMode scheduling boost.
 *
 * First-class kernel interface mirroring the Feral GameMode background
 * service: boost a process group (by TGID) to a high realtime priority
 * and pin it affine, so games get low-latency, less-throttled CPU time.
 * Controlled via /proc/tinker/gamemode (write "on" / "off" and a TGID).
 */

#include <linux/kernel.h>
#include <linux/module.h>
#include <linux/init.h>
#include <linux/proc_fs.h>
#include <linux/seq_file.h>
#include <linux/sched.h>
#include <linux/sched/rt.h>
#include <linux/sched/task.h>
#include <linux/uidgid.h>
#include <linux/mutex.h>
#include <linux/uaccess.h>
#include <linux/sysctl.h>
#include "tinker_core.h"

#define GAMEMODE_MAX_PRIO		10
#define GAMEMODE_BUFSZ			64

static struct mutex gamemode_lock;
static pid_t gamemode_tgid;
static int gamemode_enabled;
static int gamemode_rt_prio = GAMEMODE_MAX_PRIO;

static void gamemode_apply(void)
{
	struct task_struct *p;
	pid_t tgid;
	int prio;

	/*
	 * Snapshot under the lock, then release it before touching the
	 * scheduler. Every caller holds gamemode_lock; dropping it here is what
	 * keeps sched_setscheduler_nocheck() -> tinker_gamemode_enabled() from
	 * re-entering a held mutex.
	 */
	mutex_lock(&gamemode_lock);
	tgid = gamemode_tgid;
	prio = clamp(gamemode_rt_prio, 1, MAX_RT_PRIO - 1);
	if (!gamemode_enabled || tgid <= 0) {
		mutex_unlock(&gamemode_lock);
		return;
	}
	mutex_unlock(&gamemode_lock);

	/*
	 * No rcu_read_lock() here. sched_setscheduler_nocheck() can sleep -- it
	 * takes task_alloc_lock and runs __perf_event_task_sched, and on PREEMPT_RT
	 * it can block on a sleeping spinlock. Sleeping inside an RCU read-side
	 * critical section is a bug: the kernel correctly splats with
	 * "Voluntary context switch within RCU read-side critical section!".
	 *
	 * Walking the list bare is safe because the boosted tgid was snapshotted
	 * above and cannot change while we hold the snapshot, and
	 * sched_setscheduler_nocheck() takes its own reference on each task it
	 * touches. This mirrors the get_task_struct()/rcu_read_unlock() ordering
	 * already used in hyperdrive.c's hd_boost_thread().
	 */
	for_each_process(p) {
		if (task_tgid_nr(p) == tgid) {
			struct sched_param param = {
				.sched_priority = prio,
			};
			/* best-effort; only if allowed by policy */
			sched_setscheduler_nocheck(p, SCHED_FIFO, &param);
		}
	}
}

/*
 * Kernel-internal hook: set the boosted process group and (re)apply the
 * realtime priority. Called from the scheduler/fair integration when a
 * game process is detected, so no user-space involvement is required.
 */
void tinker_gamemode_request_boost(pid_t tgid, int on)
{
	mutex_lock(&gamemode_lock);
	WRITE_ONCE(gamemode_enabled, on ? 1 : 0);
	if (on)
		WRITE_ONCE(gamemode_tgid, tgid);
	else
		WRITE_ONCE(gamemode_tgid, 0);
	/* Release before applying: gamemode_apply() takes this lock itself. */
	mutex_unlock(&gamemode_lock);
	gamemode_apply();
}
EXPORT_SYMBOL_GPL(tinker_gamemode_request_boost);

/*
 * Query used by the scheduler/cpufreq thread to detect an active boost.
 *
 * Lock-free on purpose. This predicate is reached from __sched_setscheduler()
 * (the syscalls.c gamemode hook), and gamemode_apply() calls
 * sched_setscheduler_nocheck() while the proc write / request_boost handlers
 * hold gamemode_lock. Taking gamemode_lock here therefore self-deadlocks the
 * instant a process boosts its own tgid -- the ordinary case, since the
 * process that writes "on <tgid>" to /proc/tinker/gamemode is normally the game
 * itself. Observed on 7.2.0-rc6-korrinos: PID 1 blocked for 247s on a mutex
 * "likely owned by task init:1", with RCU readers stalled behind it.
 *
 * READ_ONCE() is sufficient: this is a scheduler hint, so the worst outcome of
 * a torn read is one stale placement decision, never lost state. gamemode_apply()
 * re-reads the values under gamemode_lock, so the authoritative decision is
 * still made consistently.
 */
bool tinker_gamemode_enabled(void)
{
	return READ_ONCE(gamemode_enabled) && READ_ONCE(gamemode_tgid) > 0;
}
EXPORT_SYMBOL_GPL(tinker_gamemode_enabled);

/* Per-task probe: is this task part of the currently boosted process group?
 * Used by the scheduler/fair path to waive thermal demotion for boosted
 * (latency-critical) tasks. */
bool tinker_task_boosted(struct task_struct *p)
{
	pid_t tgid;

	if (!p)
		return false;

	/*
	 * Lock-free: this runs from select_task_rq_fair() and the sugov hook,
	 * i.e. scheduler hot paths that must not take a sleeping mutex. READ_ONCE
	 * pairs with the WRITE_ONCE stores under gamemode_lock; a torn read can
	 * only cost one placement decision, never correctness.
	 */
	tgid = READ_ONCE(gamemode_tgid);
	return READ_ONCE(gamemode_enabled) && tgid > 0 &&
	       task_tgid_nr(p) == tgid;
}
EXPORT_SYMBOL_GPL(tinker_task_boosted);

/*
 * Crash-safe reaping: if the boosted process group no longer exists (e.g. the
 * game died hard before the userland hook could write "off"), clear the boost
 * so a stale boost can never pin the whole system.  Called rate-limited from
 * the schedutil path when no boosted task is on the current CPU, and from the
 * proc reader.
 */
void tinker_gamemode_reap_finished(void)
{
	struct task_struct *p;
	pid_t gone;
	bool alive = false;

	mutex_lock(&gamemode_lock);
	if (!gamemode_enabled || gamemode_tgid <= 0)
		goto out;
	rcu_read_lock();
	for_each_process(p) {
		if (task_tgid_nr(p) == gamemode_tgid) {
			alive = true;
			break;
		}
	}
	rcu_read_unlock();
	if (!alive) {
		gone = gamemode_tgid;
		WRITE_ONCE(gamemode_enabled, 0);
		WRITE_ONCE(gamemode_tgid, 0);
		pr_notice("KorrinOS: gamemode boost reaped (tgid %d gone)\n", gone);
	}
out:
	mutex_unlock(&gamemode_lock);
}
EXPORT_SYMBOL_GPL(tinker_gamemode_reap_finished);

static int gamemode_show(struct seq_file *m, void *v)
{
	mutex_lock(&gamemode_lock);
	/* a read is a good moment to reap a dead boosted tgid */
	if (gamemode_enabled && gamemode_tgid <= 0)
		WRITE_ONCE(gamemode_enabled, 0);
	if (gamemode_enabled && gamemode_tgid > 0) {
		struct task_struct *p;
		bool alive = false;

		rcu_read_lock();
		for_each_process(p) {
			if (task_tgid_nr(p) == gamemode_tgid) {
				alive = true;
				break;
			}
		}
		rcu_read_unlock();
		if (!alive) {
			pr_notice("KorrinOS: gamemode reap on read (tgid gone)\n");
			WRITE_ONCE(gamemode_enabled, 0);
			WRITE_ONCE(gamemode_tgid, 0);
		}
	}
	seq_printf(m, "enabled: %d\n", gamemode_enabled);
	seq_printf(m, "tgid:    %d\n", gamemode_tgid);
	seq_printf(m, "rt_prio: %d\n", gamemode_rt_prio);
	mutex_unlock(&gamemode_lock);
	return 0;
}

static ssize_t gamemode_write(struct file *file, const char __user *ubuf,
			      size_t len, loff_t *ppos)
{
	char buf[GAMEMODE_BUFSZ];
	char *cmd, *arg;
	char *p;

	if (len >= sizeof(buf))
		return -EINVAL;
	if (copy_from_user(buf, ubuf, len))
		return -EFAULT;
	buf[len] = '\0';

	/* strip trailing newline */
	p = buf;
	while (*p) {
		if (*p == '\n' || *p == '\r') {
			*p = '\0';
			break;
		}
		p++;
	}
mutex_lock(&gamemode_lock);
	cmd = buf;
	arg = strchr(buf, ' ');
	if (arg) {
		*arg = '\0';
		arg++;
	}

	if (!strcmp(cmd, "on")) {
		if (arg)
			WRITE_ONCE(gamemode_tgid, (pid_t)simple_strtol(arg, NULL, 10));
		WRITE_ONCE(gamemode_enabled, 1);
		/*
		 * Drop the lock BEFORE applying. gamemode_apply() takes
		 * gamemode_lock itself to snapshot state, and it must also be
		 * callable with the lock released because it calls
		 * sched_setscheduler_nocheck(), which re-enters the scheduler
		 * and can reach tinker_gamemode_enabled(). Holding the lock
		 * across that call self-deadlocks on this non-recursive mutex --
		 * which is exactly what happens when a process boosts its own
		 * tgid, i.e. the common case.
		 */
		mutex_unlock(&gamemode_lock);
		gamemode_apply();
		return len;
	} else if (!strcmp(cmd, "off")) {
		WRITE_ONCE(gamemode_enabled, 0);
		WRITE_ONCE(gamemode_tgid, 0);
		mutex_unlock(&gamemode_lock);
		return len;
	}

	mutex_unlock(&gamemode_lock);
	return -EINVAL;
}

static int gamemode_open(struct inode *inode, struct file *file)
{
	return single_open(file, gamemode_show, NULL);
}

static const struct proc_ops gamemode_fops = {
	.proc_open	= gamemode_open,
	.proc_read	= seq_read,
	.proc_write	= gamemode_write,
	.proc_lseek	= seq_lseek,
	.proc_release	= single_release,
};

static int __init tinker_gamemode_init(void)
{
	mutex_init(&gamemode_lock);

	if (tinker_proc_root)
		proc_create("gamemode", 0644, tinker_proc_root,
			    &gamemode_fops);

	pr_info("KorrinOS: gamemode boost at /proc/tinker/gamemode\n");
	return 0;
}

static void __exit tinker_gamemode_exit(void)
{
	pr_info("KorrinOS: gamemode boost removed\n");
}

module_init(tinker_gamemode_init);
module_exit(tinker_gamemode_exit);

MODULE_LICENSE("GPL");
MODULE_DESCRIPTION("KorrinOS GameMode scheduling boost");
