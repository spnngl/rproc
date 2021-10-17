//! Global kernel/system softirq statistics from /proc/stat
//!
//! See: /usr/include/linux/interrupt.h
//! See: kernel/softirq.c
//! See: https://0xax.gitbooks.io/linux-insides/content/Interrupts/linux-interrupts-9.html
/// See: https://www.kernel.org/doc/html/latest/admin-guide/kernel-per-CPU-kthreads.html
use std::num::ParseIntError;
pub use std::str::FromStr;

/// Stores the number of softirqs for all CPUs by type, there is 10 of them as of today.
#[derive(Debug, Clone, Default)]
pub struct Softirqs {
    /// Total of all softirqs
    pub all: u64,
    /// Number of high-priority tasklets and bottom halves
    pub hi: u64,
    /// Number of timer bottom half
    pub timer: u64,
    /// Number of packets transmission to network cards
    pub net_tx: u64,
    /// Number of packets reception from network cards
    pub net_rx: u64,
    /// Number of block layer softirq
    pub block: u64,
    /// Number of block IO poll softirq
    pub irq_poll: u64,
    /// Number of tasklets softirq
    pub tasklet: u64,
    /// Number of scheduler softirq
    pub sched: u64,
    /// Number of high-resolution timer softirq
    pub hrtimer: u64,
    /// Number of rcu softirq
    pub rcu: u64,
}

impl FromStr for Softirqs {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let softirqs: Vec<&str> = s
            .trim_matches(|m| m == '\n' || m == '\r')
            .split_whitespace()
            .collect();

        debug_assert!(softirqs[0].starts_with("softirq"));

        Ok(Softirqs {
            all: softirqs[1].parse::<u64>()?,
            hi: softirqs[2].parse::<u64>()?,
            timer: softirqs[3].parse::<u64>()?,
            net_tx: softirqs[4].parse::<u64>()?,
            net_rx: softirqs[5].parse::<u64>()?,
            block: softirqs[6].parse::<u64>()?,
            irq_poll: softirqs[7].parse::<u64>()?,
            tasklet: softirqs[8].parse::<u64>()?,
            sched: softirqs[9].parse::<u64>()?,
            hrtimer: softirqs[10].parse::<u64>()?,
            rcu: softirqs[11].parse::<u64>()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_softirq_stat_str0() {
        let softirqs = Softirqs::from_str(
            "softirq 229245889 94 60001584 13619 5175704 2471304 28 51212741 59130143 0 51240672\n",
        )
        .unwrap();

        assert_eq!(softirqs.all, 229245889);
        assert_eq!(softirqs.hi, 94);
        assert_eq!(softirqs.timer, 60001584);
        assert_eq!(softirqs.net_tx, 13619);
        assert_eq!(softirqs.net_rx, 5175704);
        assert_eq!(softirqs.block, 2471304);
        assert_eq!(softirqs.irq_poll, 28);
        assert_eq!(softirqs.tasklet, 51212741);
        assert_eq!(softirqs.sched, 59130143);
        assert_eq!(softirqs.hrtimer, 0);
        assert_eq!(softirqs.rcu, 51240672);
    }
}
