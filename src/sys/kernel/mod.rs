use lazy_static::lazy_static;

pub mod osrelease;

lazy_static! {
    pub static ref KERNEL_VERSION: osrelease::OsRelease = osrelease::OsRelease::current().unwrap();
}

pub mod ostype;
pub mod version;
