use super::cacheitf::PackageCacheItf;

pub struct DebPackageCache {}

impl DebPackageCache {
    pub fn new() -> Self {
        DebPackageCache {}
    }
}

impl PackageCacheItf for DebPackageCache {
    fn get_source(&self, pkgname: String) -> Result<(), std::io::Error> {
        todo!()
    }
}
