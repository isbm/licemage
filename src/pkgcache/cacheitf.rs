use std::io::Error;

pub trait PackageCacheItf {
    fn get_source(&self, pkgname: String) -> Result<(), Error>;
}
