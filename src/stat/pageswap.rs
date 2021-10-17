//! Global kernel/system page/swap statistics from /proc/stat

use std::num::ParseIntError;
pub use std::str::FromStr;

#[derive(Debug, Clone, Default)]
pub struct DoubleU64Stat {
    pub ins: u64,
    pub out: u64,
}

pub type Page = DoubleU64Stat;
pub type Swap = DoubleU64Stat;

impl FromStr for DoubleU64Stat {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let double: Vec<&str> = s
            .trim_matches(|m| m == '\n' || m == '\r')
            .split_whitespace()
            .collect();

        Ok(DoubleU64Stat {
            ins: double[1].parse::<u64>()?,
            out: double[2].parse::<u64>()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pageswap_stat_str0() {
        let page = Page::from_str("page 5741 1808\n").unwrap();

        assert_eq!(page.ins, 5741);
        assert_eq!(page.out, 1808);
    }

    #[test]
    fn test_pageswap_stat_str1() {
        let swap = Swap::from_str("swap 1 2\n").unwrap();

        assert_eq!(swap.ins, 1);
        assert_eq!(swap.out, 2);
    }
}
