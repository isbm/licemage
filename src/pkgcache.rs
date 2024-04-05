use self::{cacheitf::PackageCacheItf, debget::DebPackageCache};

mod cacheitf;
mod debget;

pub struct PackageCache {
    cache: Box<dyn PackageCacheItf>,
}

impl PackageCache {
    pub fn new() -> Self {
        PackageCache {
            cache: Box::new(DebPackageCache::new()),
        }
    }

    pub fn get_source(&self) {}
}
