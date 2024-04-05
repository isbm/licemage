use crate::licences::PkgLicence;

pub trait RootFSItf {
    /// Gets a list of packages on that rootfs.
    /// It will basically read the corresponding package database.
    fn get_pkg_list(&self) -> Vec<String>;

    fn get_pkg_license(&self, pkgname: String) -> PkgLicence;
}
