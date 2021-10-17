//! Global kernel/system CPU statistics
//!
//! This implementation needs at least Linux-2.6.33
//!
//! See: fs/proc/stat.c

use std::num::ParseIntError;
pub use std::str::FromStr;

/// Struct used to store cpu stat information gathered
///
/// # Examples
///
/// ```text
/// cpu 10132153 290696 3084719 46828483 16683 0 25195 0 175628 0
/// cpu0 1393280 32966 572056 13343292 6130 0 17875 0 23933 0
/// ...
/// cpuN 205335 71 72949 5476469 1179 14642 4387 0 0 0
/// ```
#[derive(Debug, Clone, Default)]
pub struct CpuStat {
    /// CPU number, -1 for aggregate
    pub cpu_number: i32,
    /// Time spent in user mode
    pub user: u64,
    /// Time spent in user mode with low priority (nice)
    pub nice: u64,
    /// Time spent in system mode
    pub system: u64,
    /// Time spent in the idle task.  This value should be USER_HZ times the second entry in the
    /// /proc/uptime pseudo-file
    pub idle: u64,
    /// Time waiting for I/O to complete
    pub iowait: u64,
    /// Time servicing interrupts
    pub irq: u64,
    /// Time servicing softirqs
    pub softirq: u64,
    /// Stolen time, which is the time spent in other operating systems when running in
    /// a virtualized environment
    pub steal: u64,
    /// Time spent running a virtual CPU for guest operating systems under the control of the Linux kernel
    pub guest: u64,
    /// Time spent running a niced guest (virtual CPU for guest operating systems under the control
    /// of the Linux kernel)
    pub guest_nice: u64,
}

impl FromStr for CpuStat {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stats: Vec<&str> = s
            .trim_matches(|m| m == '\n' || m == '\r')
            .split_whitespace()
            .collect();

        debug_assert!(stats[0].starts_with("cpu"));

        // Check if CPU aggregate stats & get CPU numbers
        let cpu_number = if stats[0].len() == 3 {
            -1
        } else {
            stats[0][3..].parse::<i32>()?
        };

        Ok(CpuStat {
            cpu_number: cpu_number,
            user: stats[1].parse::<u64>()?,
            nice: stats[2].parse::<u64>()?,
            system: stats[3].parse::<u64>()?,
            idle: stats[4].parse::<u64>()?,
            iowait: stats[5].parse::<u64>()?,
            irq: stats[6].parse::<u64>()?,
            softirq: stats[7].parse::<u64>()?,
            steal: stats[8].parse::<u64>()?,
            guest: stats[9].parse::<u64>()?,
            guest_nice: stats[10].parse::<u64>()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_stat_str0() {
        let cpu_stat =
            CpuStat::from_str("cpu  10132153 290696 3084719 46828483 16683 42 25195 4242 175628 424242\n")
                .unwrap();

        assert_eq!(cpu_stat.cpu_number, -1);
        assert_eq!(cpu_stat.user, 10132153);
        assert_eq!(cpu_stat.nice, 290696);
        assert_eq!(cpu_stat.system, 3084719);
        assert_eq!(cpu_stat.idle, 46828483);
        assert_eq!(cpu_stat.iowait, 16683);
        assert_eq!(cpu_stat.irq, 42);
        assert_eq!(cpu_stat.softirq, 25195);
        assert_eq!(cpu_stat.steal, 4242);
        assert_eq!(cpu_stat.guest, 175628);
        assert_eq!(cpu_stat.guest_nice, 424242);
    }

    #[test]
    fn test_cpu_stat_str1() {
        let cpu_stat =
            CpuStat::from_str("cpu2 1393280 32966 572056 13343292 6130 0 17875 0 23933 0\n").unwrap();

        assert_eq!(cpu_stat.cpu_number, 2);
        assert_eq!(cpu_stat.user, 1393280);
        assert_eq!(cpu_stat.nice, 32966);
        assert_eq!(cpu_stat.system, 572056);
        assert_eq!(cpu_stat.idle, 13343292);
        assert_eq!(cpu_stat.iowait, 6130);
        assert_eq!(cpu_stat.irq, 0);
        assert_eq!(cpu_stat.softirq, 17875);
        assert_eq!(cpu_stat.steal, 0);
        assert_eq!(cpu_stat.guest, 23933);
        assert_eq!(cpu_stat.guest_nice, 0);
    }

    #[test]
    #[should_panic]
    fn test_cpu_stat_str2() {
        CpuStat::from_str("cpu2 1393280 32966 572056 13343292 6130 0 17875 0\n").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_cpu_stat_str3() {
        CpuStat::from_str("cpuN 1393280 32966 572056 13343292 6130 0 17875 0 23933 0\n").unwrap();
    }
}
