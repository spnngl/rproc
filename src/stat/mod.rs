//! Global kernel/system statistics
//!
//! See: fs/proc/stat.c

mod cpu;
use cpu::*;

mod simple_stat;
use simple_stat::*;

mod softirq;
use softirq::*;

mod pageswap;
use pageswap::*;

const STAT: &'static str = "/proc/stat";

#[derive(Debug, Clone, Default)]
pub struct Stat {
    pub cpus: Vec<CpuStat>,
    pub ctxt: Ctxt,
    pub btime: Btime,
    pub processes: Processes,
    pub procs_running: ProcsRunning,
    pub procs_blocked: ProcsBlocked,
    pub softirqs: Softirqs,
    pub page: Page,
    pub swap: Swap,
}

impl Stat {
    pub fn new() -> Self {
        Self::parse_stat_file().unwrap()
    }

    fn parse_stat_file() -> Result<Self, &'static str> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let mut stats = Stat::default();

        let f = File::open(STAT).unwrap();
        let lines = BufReader::new(f).lines();

        for line in lines {
            if let Ok(l) = line {
                // TODO static hashmap/array/LUT ?
                let stat_type = l.split_whitespace().next().unwrap();

                match stat_type {
                    "btime" => stats.btime = Btime::from_str(&l).unwrap(),
                    "ctxt" => stats.ctxt = Ctxt::from_str(&l).unwrap(),
                    "processes" => stats.processes = Processes::from_str(&l).unwrap(),
                    "procs_blocked" => stats.procs_blocked = ProcsBlocked::from_str(&l).unwrap(),
                    "procs_running" => stats.procs_running = ProcsRunning::from_str(&l).unwrap(),
                    "softirq" => stats.softirqs = Softirqs::from_str(&l).unwrap(),
                    "page" => stats.page = Page::from_str(&l).unwrap(),
                    "swap" => stats.swap = Swap::from_str(&l).unwrap(),
                    _ if stat_type.starts_with("cpu") => stats.cpus.push(CpuStat::from_str(&l).unwrap()),
                    _ => eprintln!("{} section not supported", stat_type),
                }
            }
        }

        Ok(stats)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_stat() {
        let stats = Stat::new();
        println!("current /proc/stat: {:?}", stats);
    }
}
