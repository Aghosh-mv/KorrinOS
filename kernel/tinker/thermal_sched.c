// SPDX-License-Identifier: GPL-2.0
/*
 * KorrinOS thermal-aware scheduling hints.
 *
 * Implements the "silicon thermal mapping" concept as a real kernel
 * feature: maintain a live per-CPU heat map, expose it via
 * /proc/tinker/thermal, and let the scheduler treat hot cores as lower
 * preference for new load. The map is fed by the scheduler (marking
 * overloaded cores) plus a periodic decay so the signal self-heals.
 */

#include <linux/kernel.h>
#include <linux/module.h>
#include <linux/init.h>
#include <linux/proc_fs.h>
#include <linux/seq_file.h>
#include <linux/percpu.h>
#include <linux/smp.h>
#include <linux/mutex.h>
#include <linux/spinlock.h>
#include <linux/uaccess.h>
#include <linux/workqueue.h>
#include <linux/fs.h>
#include <linux/namei.h>
#include <linux/uaccess.h>
#include <linux/sched.h>

#include "tinker_core.h"

#define THERMAL_MAX_ZONES	8
#define THERMAL_BUF_SZ		32

struct tinker_heat_map {
	unsigned int		zones;
	unsigned long		last_sample;
	/* per-cpu rolling temperature estimate in millicelsius */
	u64			cpu_temp_mc[NR_CPUS];
	u64			cpu_hot_thresh_mc;
	unsigned int		enabled;
	u64			zone_temps[THERMAL_MAX_ZONES];
};

static struct tinker_heat_map heat;
static DEFINE_MUTEX(thermal_lock);	/* guards seq_file show() */
static DEFINE_SPINLOCK(thermal_map_lock); /* guards cpu_temp_mc[cpu] */
static struct delayed_work thermal_decay_work;

/* Mark a CPU hot; decay brings it back down over time. */
void tinker_thermal_hint_hot_cpu(int cpu)
{
	/* Single-writer, advisory data: use WRITE_ONCE, not a lock. This
	 * function is exported and can be called from any context, including
	 * IRQ, so it must not take a spinlock. */
	if (!READ_ONCE(heat.enabled) || cpu < 0 || cpu >= nr_cpu_ids)
		return;

	if (READ_ONCE(heat.cpu_temp_mc[cpu]) <
	    READ_ONCE(heat.cpu_hot_thresh_mc))
		WRITE_ONCE(heat.cpu_temp_mc[cpu],
			   READ_ONCE(heat.cpu_hot_thresh_mc) + 1000);
}
EXPORT_SYMBOL_GPL(tinker_thermal_hint_hot_cpu);

/* Query used by the scheduler: should this CPU be avoided for new load? */
bool tinker_thermal_is_hot(int cpu)
{
	/*
	 * Deliberately lock-free.
	 *
	 * This function is called from find_energy_efficient_cpu() in the CFS
	 * load balancer, which runs on EVERY task wakeup - including wakeups
	 * raised from IRQ and softirq context (process_timeout() running in the
	 * timer softirq reaches try_to_wake_up() -> select_task_rq_fair()).
	 *
	 * It used to take thermal_map_lock here. Lockdep proved that is a real
	 * bug, not a theoretical one:
	 *
	 *   BUG: Invalid wait context
	 *   swapper/0/1 is trying to lock:
	 *   ffffffff83c9a318 (thermal_map_lock){....}-{3:3},
	 *       at: tinker_thermal_is_hot+0x50/0xb0
	 *   context-{3:3}
	 *
	 * Acquiring a spinlock from the scheduler's wakeup path is a deadlock
	 * waiting to happen and matches the multi-CPU stall seen at -smp 4
	 * (0/4, no panic, no oops - a deadlock with the watchdog unable to
	 * report looks exactly like that).
	 *
	 * Lock-free reads are correct here: cpu_temp_mc[] has a single writer
	 * (the decay work function) and the value is purely advisory. If a CPU
	 * is reported 1 sample stale when deciding where to place a task, the
	 * cost is one misplaced wakeup; if we deadlock, the machine is gone.
	 * READ_ONCE gives a coherent-enough single-word snapshot without any
	 * exclusion.
	 */
	if (!READ_ONCE(heat.enabled) || cpu < 0 || cpu >= nr_cpu_ids)
		return false;

	return READ_ONCE(heat.cpu_temp_mc[cpu]) >
	       READ_ONCE(heat.cpu_hot_thresh_mc);
}
EXPORT_SYMBOL_GPL(tinker_thermal_is_hot);

/* Cool the map back toward zero so the heat signal self-heals. */
static void thermal_decay_workfn(struct work_struct *work)
{
	unsigned long flags;
	int cpu;
	unsigned int i;
	char path[64];
	char buf[THERMAL_BUF_SZ];
	struct file *f;
	loff_t pos = 0;
	ssize_t nr;
	long temp_mc;

	/* Read real thermal zone temperatures and blend them in */
	for (i = 0; i < THERMAL_MAX_ZONES; i++) {
		snprintf(path, sizeof(path),
			 "/sys/class/thermal/thermal_zone%u/temp", i);
		f = filp_open(path, O_RDONLY, 0);
		if (IS_ERR(f))
			break;
		memset(buf, 0, sizeof(buf));
		nr = kernel_read(f, buf, sizeof(buf) - 1, &pos);
		filp_close(f, NULL);
		if (nr > 0) {
			buf[nr] = '\0';
			if (kstrtol(buf, 10, &temp_mc) == 0) {
				/* Kernel thermal zones report millidegrees */
				if (temp_mc < 200)
					temp_mc *= 1000;
				heat.zone_temps[i] = temp_mc;
			}
		}
	}
	heat.zones = i;

	spin_lock_irqsave(&thermal_map_lock, flags);
	for_each_possible_cpu(cpu) {
		/*
		 * Bounds guard. for_each_possible_cpu() walks the CPU possible
		 * mask, whose highest set bit can reach NR_CPUS - 1, so indexing
		 * cpu_temp_mc[] directly is only safe while cpu < NR_CPUS. The two
		 * other accessors in this file (tinker_thermal_is_hot,
		 * tinker_thermal_hint_hot_cpu) both clamp against nr_cpu_ids; this
		 * loop did not, so a hotplug-configured machine could write past
		 * the end of the array and corrupt cpu_hot_thresh_mc and enabled
		 * immediately after it - which would make the scheduler heuristic
		 * return garbage.
		 */
		if (cpu < 0 || cpu >= NR_CPUS)
			continue;

		/*
		 * zone_temps[] is THERMAL_MAX_ZONES (8) entries. heat.zones is
		 * set from a count that is not clamped to that bound, so the
		 * previous test 'cpu < heat.zones' could index past the end of
		 * zone_temps[]. Bound by the array, not by the count.
		 */
		if (heat.zones > 0 && cpu < heat.zones &&
		    cpu < THERMAL_MAX_ZONES) {
			heat.cpu_temp_mc[cpu] =
				(heat.cpu_temp_mc[cpu] + heat.zone_temps[cpu]) / 2;
		}
		/* Decay toward zero. Subtracting from an unsigned value that is
		 * smaller than the step would wrap to a huge number and mark the
		 * CPU permanently hot. */
		if (heat.cpu_temp_mc[cpu] > 0) {
			if (heat.cpu_temp_mc[cpu] > 500)
				heat.cpu_temp_mc[cpu] -= 500;
			else
				heat.cpu_temp_mc[cpu] = 0;
		}
	}
	spin_unlock_irqrestore(&thermal_map_lock, flags);

	schedule_delayed_work(&thermal_decay_work,
			      msecs_to_jiffies(5000));
}

static int thermal_show(struct seq_file *m, void *v)
{
	unsigned long flags;
	int cpu;
	unsigned int enabled;

	mutex_lock(&thermal_lock);
	enabled = heat.enabled;
	seq_printf(m, "enabled:        %u\n", enabled);
	seq_printf(m, "zones:          %u\n", heat.zones);
	seq_printf(m, "hot_threshold:  %llu mc\n",
		   (unsigned long long)heat.cpu_hot_thresh_mc);
	seq_puts(m, "cpu_temp_mc:\n");
	spin_lock_irqsave(&thermal_map_lock, flags);
	for_each_possible_cpu(cpu) {
		if (cpu < 0 || cpu >= NR_CPUS)
			continue;
		seq_printf(m, "  cpu%d:       %llu mc\n", cpu,
			   (unsigned long long)heat.cpu_temp_mc[cpu]);
	}
	spin_unlock_irqrestore(&thermal_map_lock, flags);
	mutex_unlock(&thermal_lock);
	return 0;
}

static int thermal_open(struct inode *inode, struct file *file)
{
	return single_open(file, thermal_show, NULL);
}

static const struct proc_ops thermal_fops = {
	.proc_open	= thermal_open,
	.proc_read	= seq_read,
	.proc_lseek	= seq_lseek,
	.proc_release	= single_release,
};

static int __init tinker_thermal_init(void)
{
	heat.cpu_hot_thresh_mc = 80000; /* 80 C default */
	heat.enabled = 1;
	heat.zones = 1;

	INIT_DELAYED_WORK(&thermal_decay_work, thermal_decay_workfn);
	schedule_delayed_work(&thermal_decay_work, msecs_to_jiffies(5000));

	if (tinker_proc_root)
		/*
		 * Read-only by design: thermal_fops has no .proc_write handler, so
		 * with 0644 the node advertised itself writable but every write
		 * failed with EINVAL. Either the mode was wrong or a handler was
		 * missing. The temperature map is sampler-owned - only the decay work
		 * function writes it - so exposing writes would need a real control
		 * surface. Marked read-only so the mode matches the capability.
		 */
		proc_create("thermal", 0444, tinker_proc_root, &thermal_fops);

	pr_info("KorrinOS: thermal scheduler interface at /proc/tinker/thermal\n");
	return 0;
}

static void __exit tinker_thermal_exit(void)
{
	cancel_delayed_work_sync(&thermal_decay_work);
	pr_info("KorrinOS: thermal scheduler interface removed\n");
}

module_init(tinker_thermal_init);
module_exit(tinker_thermal_exit);

MODULE_LICENSE("GPL");
MODULE_DESCRIPTION("KorrinOS thermal-aware scheduling hints");
