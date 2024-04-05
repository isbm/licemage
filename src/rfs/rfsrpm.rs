use super::rfsitf::RootFSItf;

pub struct RpmRootFsScan {}

impl RpmRootFsScan {}

impl RootFSItf for RpmRootFsScan {
    fn get_pkg_list(&self) -> Vec<String> {
        todo!("Go ahead, implement this for RPM and send us PR! :-)")
    }
}
