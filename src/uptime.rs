/// System uptime information
///
/// See: kernel file fs/proc/uptime.c
use lazy_static::lazy_static;
use std::ffi::CString;
use std::num::ParseFloatError;
use std::str::FromStr;

/// This file contains two numbers (values in seconds): the uptime of the system (including time
/// spent in suspend) and the amount of time spent in the idle process.
const UPTIME: &'static str = "/proc/uptime";
const UPTIME_FMT: &'static str = "%lf %lf";
const UPTIME_FMT_NB_VAR: i32 = 2;

lazy_static! {
    static ref UPTIME_FMT_C: CString = CString::new(UPTIME_FMT).unwrap();
    static ref O_RDONLY: CString = CString::new("r").unwrap();
}

#[derive(Debug, Clone, Default)]
pub struct Uptime {
    /// System uptime, including idle (in seconds)
    pub uptime: f64,
    /// Time spent in idle task (in seconds)
    pub idle: f64,
}

impl Uptime {
    pub fn new() -> Self {
        Self::parse_uptime_file().unwrap()
    }

    fn parse_uptime_file() -> Result<Self, &'static str> {
        use libc::{c_double, c_int, fclose, fdopen, fscanf, FILE};
        use std::fs::File;
        use std::os::unix::io::IntoRawFd;

        let err: c_int;
        let mut uptime = Uptime::default();
        let f = File::open(UPTIME).unwrap();

        unsafe {
            let cf: *mut FILE = fdopen(f.into_raw_fd(), (*O_RDONLY).as_ptr());

            err = fscanf(
                cf,
                (*UPTIME_FMT_C).as_ptr(),
                &mut uptime.uptime as *mut c_double,
                &mut uptime.idle as *mut c_double,
            );

            fclose(cf);
        }

        if err != UPTIME_FMT_NB_VAR {
            return Err("Error during fscanf()");
        }

        Ok(uptime)
    }
}

impl FromStr for Uptime {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ups: Vec<&str> = s
            .trim_matches(|m| m == '\n' || m == '\r')
            .split_whitespace()
            .collect();

        let uptime = ups[0].parse::<f64>()?;
        let idle = ups[1].parse::<f64>()?;

        Ok(Uptime {
            uptime: uptime,
            idle: idle,
        })
    }
}

mod tests {
    use crate::uptime::*;

    #[test]
    fn test_local_uptime() {
        let uptime = Uptime::new();

        println!("local uptime: {:?}", uptime);
        assert!(uptime.uptime > 0.);
        assert!(uptime.idle > 0.);
    }

    #[test]
    fn test_str_parser0() {
        let uptime = Uptime::from_str("96445.86 402942.06\n").unwrap();

        assert_eq!(uptime.uptime, 96445.86 as f64);
        assert_eq!(uptime.idle, 402942.06 as f64);
    }
}
