use super::rfsitf::RootFSItf;

pub struct RpmRootFsScan {}

impl RpmRootFsScan {}

impl RootFSItf for RpmRootFsScan {
    fn get_pkg_list(&self) -> Vec<String> {
        todo!("Not implemented")
    }

    #[allow(unused_variables)]
    fn get_pkg_license(&self, pkgname: String) -> crate::licences::PkgLicence {
        todo!("Not implemented")
    }
}
