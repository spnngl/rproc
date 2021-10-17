//! Pressure stall information
//!
//! Added to linux with commit eb414681d5a07, available on kernel >= 4.20
//!
//! See: https://lwn.net/Articles/759781/
//! See: https://www.kernel.org/doc/html/latest/accounting/psi.html
//! See: https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/kernel/sched/psi.c

use crate::sys::kernel::osrelease;
/// Current kernel version (lazy_static)
use crate::sys::kernel::KERNEL_VERSION;
use lazy_static::lazy_static;
use std::ffi::CString;

const PRESSURE_CPU: &'static str = "/proc/pressure/cpu";
const PRESSURE_MEM: &'static str = "/proc/pressure/memory";
const PRESSURE_IO: &'static str = "/proc/pressure/io";

const PRESSURE_FMT: &'static str =
    "some avg10=%f avg60=%f avg300=%f total=%llu full avg10=%f avg60=%f avg300=%f total=%llu";
const PRESSURE_FMT_NB_VAR: i32 = 8;

// CPU psi does not have "full" line for kernel < 5.13, added with commit e7fcd76228233
// See: https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/kernel/sched/psi.c
const PRESSURE_FMT_CPU_OLD: &'static str = "some avg10=%f avg60=%f avg300=%f total=%llu";
const PRESSURE_FMT_CPU_OLD_NB_VAR: i32 = 4;

lazy_static! {
    static ref PRESSURE_FMT_C: CString = CString::new(PRESSURE_FMT).unwrap();
    static ref PRESSURE_FMT_CPU_OLD_C: CString = CString::new(PRESSURE_FMT_CPU_OLD).unwrap();
    static ref O_RDONLY: CString = CString::new("r").unwrap();
    static ref KERNEL_5_13_VERSION_CODE: u32 = osrelease::kernel_version(5, 13, 0);
}

pub enum Pressure {
    Cpu,
    Mem,
    Io,
}

#[derive(Debug, Clone, Default)]
pub struct PressureAvg {
    pub avg10: f32,
    pub avg60: f32,
    pub avg300: f32,
    pub total: u64,
}

#[derive(Debug, Clone, Default)]
pub struct PressureStore {
    pub some: PressureAvg,
    pub full: PressureAvg,
}

impl PressureStore {
    pub fn new(t: Pressure) -> Option<Self> {
        match t {
            Pressure::Cpu => {
                if KERNEL_VERSION.version_code >= *KERNEL_5_13_VERSION_CODE {
                    Self::parse_pressure_file(PRESSURE_CPU, &PRESSURE_FMT_C, PRESSURE_FMT_NB_VAR)
                } else {
                    Self::parse_pressure_file(
                        PRESSURE_CPU,
                        &PRESSURE_FMT_CPU_OLD_C,
                        PRESSURE_FMT_CPU_OLD_NB_VAR,
                    )
                }
            }
            Pressure::Mem => Self::parse_pressure_file(PRESSURE_MEM, &PRESSURE_FMT_C, PRESSURE_FMT_NB_VAR),
            Pressure::Io => Self::parse_pressure_file(PRESSURE_IO, &PRESSURE_FMT_C, PRESSURE_FMT_NB_VAR),
        }
    }

    fn parse_pressure_file(path: &str, fmt: &CString, nb_var: i32) -> Option<Self> {
        use libc::{c_float, c_int, c_ulonglong, fclose, fdopen, fscanf, FILE};
        use std::fs::File;
        use std::os::unix::io::IntoRawFd;

        let err: c_int;
        let mut pstore = PressureStore::default();

        // On linux those files are optional, do not consider their absence as an error.
        let res = File::open(path);
        let f = match res {
            Ok(f) => f,
            Err(_) => {
                return None;
            }
        };

        unsafe {
            let cf: *mut FILE = fdopen(f.into_raw_fd(), (*O_RDONLY).as_ptr());

            err = fscanf(
                cf,
                (*fmt).as_ptr(),
                &mut pstore.some.avg10 as *mut c_float,
                &mut pstore.some.avg60 as *mut c_float,
                &mut pstore.some.avg300 as *mut c_float,
                &mut pstore.some.total as *mut c_ulonglong,
                &mut pstore.full.avg10 as *mut c_float,
                &mut pstore.full.avg60 as *mut c_float,
                &mut pstore.full.avg300 as *mut c_float,
                &mut pstore.full.total as *mut c_ulonglong,
            );

            fclose(cf);
        }

        if err != nb_var {
            // TODO add strerror (unsafe :| )
            eprintln!("Error during fscanf()");
            return None;
        }

        Some(pstore)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[inline(always)]
    fn percent_is_valid(v: f32) -> bool {
        v >= 0. && v <= 100.
    }

    #[test]
    fn test_local_cpu_pressure() {
        if let Some(pressure) = PressureStore::new(Pressure::Cpu) {
            println!("cpu pressure: {:?}", pressure);
            assert!(percent_is_valid(pressure.some.avg10));
            assert!(percent_is_valid(pressure.some.avg60));
            assert!(percent_is_valid(pressure.some.avg300));
            assert!(percent_is_valid(pressure.full.avg10));
            assert!(percent_is_valid(pressure.full.avg60));
            assert!(percent_is_valid(pressure.full.avg300));
        }
    }

    #[test]
    fn test_local_mem_pressure() {
        if let Some(pressure) = PressureStore::new(Pressure::Mem) {
            println!("memory pressure: {:?}", pressure);
            assert!(percent_is_valid(pressure.some.avg10));
            assert!(percent_is_valid(pressure.some.avg60));
            assert!(percent_is_valid(pressure.some.avg300));
            assert!(percent_is_valid(pressure.full.avg10));
            assert!(percent_is_valid(pressure.full.avg60));
            assert!(percent_is_valid(pressure.full.avg300));
        }
    }

    #[test]
    fn test_local_io_pressure() {
        if let Some(pressure) = PressureStore::new(Pressure::Io) {
            println!("io pressure: {:?}", pressure);
            assert!(percent_is_valid(pressure.some.avg10));
            assert!(percent_is_valid(pressure.some.avg60));
            assert!(percent_is_valid(pressure.some.avg300));
            assert!(percent_is_valid(pressure.full.avg10));
            assert!(percent_is_valid(pressure.full.avg60));
            assert!(percent_is_valid(pressure.full.avg300));
        }
    }
}
