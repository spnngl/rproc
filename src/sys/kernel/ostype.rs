//! OS type information
//!
//! See: https://www.kernel.org/doc/html/latest/admin-guide/sysctl/kernel.html?highlight=osrelease#osrelease-ostype-version

use std::fmt;

const OSTYPE: &'static str = "/proc/sys/kernel/ostype";

pub struct OsType(String);

impl OsType {
    pub fn new(os_type: String) -> Self {
        OsType(os_type)
    }

    pub fn current() -> Result<Self, &'static str> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let f = File::open(OSTYPE).unwrap();
        let mut reader = BufReader::new(f);
        let mut os_type = String::new();

        if let Err(_) = reader.read_line(&mut os_type) {
            return Err("Error during BufReader::read_line()");
        }

        Ok(OsType::new(os_type))
    }
}

impl fmt::Display for OsType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current() {
        if let Ok(os_type) = OsType::current() {
            println!("current os_type = {}", os_type);
        } else {
            assert!(false, "Error during OsType::current()");
        }
    }
}
