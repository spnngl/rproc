//! OS type information
//!
//! See: https://www.kernel.org/doc/html/latest/admin-guide/sysctl/kernel.html?highlight=osrelease#osrelease-ostype-version

use std::fmt;

const OSVERSION: &'static str = "/proc/sys/kernel/version";

/// Current OS version
///
/// Contains build number and date of build
///
/// # Examples
///
/// ```text
/// #91-Ubuntu SMP Thu Jul 15 19:09:17 UTC 2021
/// ```
///
/// ```text
/// #52 SMP Fri Oct 15 16:23:14 CEST 2021
/// ```
///
/// ```text
/// #5 Wed Feb 25 21:49:24 MET 1998
/// ```
pub struct OsVersion(String);

impl OsVersion {
    pub fn new(os_version: String) -> Self {
        OsVersion(os_version)
    }

    pub fn current() -> Result<Self, &'static str> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let f = File::open(OSVERSION).unwrap();
        let mut reader = BufReader::new(f);
        let mut os_version = String::new();

        if let Err(_) = reader.read_line(&mut os_version) {
            return Err("Error during BufReader::read_line()");
        }

        Ok(OsVersion::new(os_version))
    }
}

impl fmt::Display for OsVersion {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current() {
        if let Ok(os_version) = OsVersion::current() {
            println!("current os_version = {}", os_version);
        } else {
            assert!(false, "Error during OsVersion::current()");
        }
    }
}
