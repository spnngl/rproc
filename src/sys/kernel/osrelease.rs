//! OS release information
//!
//! See: https://www.kernel.org/doc/html/latest/admin-guide/sysctl/kernel.html?highlight=osrelease#osrelease-ostype-version

use lazy_static::lazy_static;
use std::cmp::Ordering;
use std::ffi::CString;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

const OSRELEASE: &'static str = "/proc/sys/kernel/osrelease";
const OSRELEASE_FMT: &'static str = "%u.%u.%u";
const OSRELEASE_FMT_NB_VAR: i32 = 3;

lazy_static! {
    static ref OSRELEASE_FMT_C: CString = CString::new(OSRELEASE_FMT).unwrap();
    static ref O_RDONLY: CString = CString::new("r").unwrap();
}

#[derive(Debug, Clone, Default, Eq)]
pub struct OsRelease {
    pub version_code: u32,
    pub major: u8,
    pub minor: u8,
    pub patch: u16,
}

/// Compute kernel version code
///
/// Kernel-5.14.12 example code from /usr/include/version.h
/// ```c
/// #define KERNEL_VERSION(a,b,c) (((a) << 16) + ((b) << 8) + ((c) > 255 ? 255 : (c)))
/// ```
///
/// # Panics
///
/// Don't.
///
/// # Examples
///
/// ```
/// use rproc::sys::kernel::osrelease::kernel_version;
/// let kcode: u32 = kernel_version(3, 18, 0);
/// assert_eq!(kcode, 201216);
/// ```
///
/// ```
/// use rproc::sys::kernel::osrelease::kernel_version;
/// let kcode: u32 = kernel_version(5, 14, 12);
/// assert_eq!(kcode, 331276);
/// ```
///
/// ```
/// use rproc::sys::kernel::osrelease::kernel_version;
/// let kcode: u32 = kernel_version(4, 4, 288);
/// assert_eq!(kcode, 263423);
/// ```
#[inline]
pub fn kernel_version(major: u8, minor: u8, patch: u16) -> u32 {
    ((major as u32) << 16) | ((minor as u32) << 8) | (if patch > 255 { 255 } else { patch as u32 })
}

impl OsRelease {
    pub fn new(major: u8, minor: u8, patch: u16) -> Self {
        OsRelease {
            version_code: kernel_version(major, minor, patch),
            major: major,
            minor: minor,
            patch: patch,
        }
    }

    /// Read current osrelease
    ///
    /// # Panics
    ///
    /// Panic if we are unable to open /proc/sys/kernel/osrelease.
    pub fn current() -> Result<Self, &'static str> {
        use libc::{c_int, c_uint, fclose, fdopen, fscanf, FILE};
        use std::fs::File;
        use std::os::unix::io::IntoRawFd;

        let err: c_int;
        let (mut major, mut minor, mut patch): (c_uint, c_uint, c_uint) = (0, 0, 0);
        let f = File::open(OSRELEASE).unwrap();

        unsafe {
            let cf: *mut FILE = fdopen(f.into_raw_fd(), (*O_RDONLY).as_ptr());

            err = fscanf(
                cf,
                (*OSRELEASE_FMT_C).as_ptr(),
                &mut major,
                &mut minor,
                &mut patch,
            );

            fclose(cf);
        }

        if err != OSRELEASE_FMT_NB_VAR {
            return Err("Error during fscanf()");
        }

        Ok(OsRelease {
            version_code: kernel_version(major as u8, minor as u8, patch as u16),
            major: major as u8,
            minor: minor as u8,
            patch: patch as u16,
        })
    }
}

impl Ord for OsRelease {
    fn cmp(&self, other: &Self) -> Ordering {
        self.version_code.cmp(&other.version_code)
    }
}

impl PartialOrd for OsRelease {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for OsRelease {
    fn eq(&self, other: &Self) -> bool {
        self.version_code == other.version_code
    }
}

impl fmt::Display for OsRelease {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl FromStr for OsRelease {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s_trimmed: Vec<&str> = s.trim_matches(|m| m == '\n' || m == '\r').split("-").collect();
        let osrelease_v: Vec<&str> = s_trimmed[0].split(".").collect();

        let major = osrelease_v[0].parse::<u8>()?;
        let minor = osrelease_v[1].parse::<u8>()?;
        let patch = osrelease_v[2].parse::<u16>()?;

        Ok(OsRelease {
            version_code: kernel_version(major, minor, patch),
            major: major,
            minor: minor,
            patch: patch,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current() {
        if let Ok(os_release) = OsRelease::current() {
            println!("current os_release = {:?}", os_release);
            println!("current os_release string = {}", os_release);
            assert!(os_release.version_code > 0);
            assert!(os_release.major > 0);
            assert!(os_release.minor > 0);
            assert!(os_release.patch > 0);
        } else {
            assert!(false, "Error during OsRelease::current()");
        }
    }

    #[test]
    fn test_sup() {
        let one = OsRelease::new(1, 2, 3);
        let two = OsRelease::new(4, 5, 6);

        assert!(one < two);
    }

    #[test]
    fn test_eq0() {
        let one = OsRelease::new(1, 2, 3);
        let two = OsRelease::new(1, 2, 3);

        assert!(one == two);
    }

    #[test]
    fn test_eq1() {
        let one = OsRelease::new(1, 2, 255);
        let two = OsRelease::new(1, 2, 300);

        assert!(one == two);
    }

    #[test]
    fn test_str_parser0() {
        let osrelease = OsRelease::from_str("5.14.12-amd64").unwrap();

        assert_eq!(osrelease.version_code, 331276);
        assert_eq!(osrelease.major, 5);
        assert_eq!(osrelease.minor, 14);
        assert_eq!(osrelease.patch, 12);
    }

    #[test]
    fn test_str_parser1() {
        let osrelease = OsRelease::from_str("5.14.12").unwrap();

        assert_eq!(osrelease.version_code, 331276);
        assert_eq!(osrelease.major, 5);
        assert_eq!(osrelease.minor, 14);
        assert_eq!(osrelease.patch, 12);
    }
}
