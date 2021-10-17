//! Global simple kernel/system statistics from /proc/stat

use std::num::ParseIntError;
pub use std::str::FromStr;

#[derive(Debug, Clone, Default)]
pub struct SimpleU64Stat(u64);

/// boot time, in seconds since the Epoch, 1970-01-01 00:00:00 +0000 (UTC)
pub type Btime = SimpleU64Stat;
/// This struct contains the number of context switches that the system underwent
pub type Ctxt = SimpleU64Stat;
/// Number of forks since boot
pub type Processes = SimpleU64Stat;
/// Number of processes in runnable state
pub type ProcsRunning = SimpleU64Stat;
/// Number of processes blocked waiting for I/O to complete
pub type ProcsBlocked = SimpleU64Stat;

impl FromStr for SimpleU64Stat {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stats: Vec<&str> = s
            .trim_matches(|m| m == '\n' || m == '\r')
            .split_whitespace()
            .collect();

        Ok(SimpleU64Stat(stats[1].parse::<u64>()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ctxt_parser0() {
        let ctxt = Ctxt::from_str("ctxt 115315\n").unwrap();

        assert_eq!(ctxt.0, 115315);
    }

    #[test]
    #[should_panic]
    fn test_ctxt_parser1() {
        Ctxt::from_str("ctxt 115315.0\n").unwrap();
    }
}
