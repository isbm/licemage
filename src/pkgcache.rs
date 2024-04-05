use self::{cacheitf::PackageCacheItf, debget::DebPackageCache};

mod cacheitf;
mod debget;

#[allow(dead_code)]
pub struct PackageCache {
    cache: Box<dyn PackageCacheItf>,
}

#[allow(dead_code)]
impl PackageCache {
    pub fn new() -> Self {
        PackageCache { cache: Box::new(DebPackageCache::new()) }
    }

    pub fn get_source(&self) {}
}
