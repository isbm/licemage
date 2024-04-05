use crate::rfs::rfsrpm::RpmRootFsScan;

use self::{rfsdeb::DebRootFsScan, rfsitf::RootFSItf, rfstype::RfsType};
use std::{io::Error, path::PathBuf};

mod rfsdeb;
mod rfsitf;
mod rfsrpm;
mod rfstype;

pub struct RfsScan {
    pkgscan: Box<dyn RootFSItf>,
}

impl RfsScan {
    pub fn new(p: PathBuf) -> Result<Self, Error> {
        if RfsType::new(p.clone()).get_pkg_mgr() == rfstype::PKG_MGR_DEB {
            return Ok(RfsScan {
                pkgscan: Box::new(DebRootFsScan::new(p)?),
            });
        } else {
            return Ok(RfsScan {
                pkgscan: Box::new(RpmRootFsScan {}),
            });
        }
    }

    pub fn get_pkg_list(&self) -> Vec<String> {
        self.pkgscan.get_pkg_list()
    }
}
