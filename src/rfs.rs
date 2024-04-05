use self::{rfsdeb::DebRootFsScan, rfsitf::RootFSItf, rfstype::RfsType};
use crate::{licences::PkgLicence, rfs::rfsrpm::RpmRootFsScan};
use std::{io::Error, path::PathBuf};

mod rfsdeb;
mod rfsitf;
mod rfsrpm;
mod rfstype;

/// Root filesystem scanner
///
/// Incapsulates different scanner implementations
/// for different distributions, ecosystems etc.
pub struct RfsScan {
    pkgscan: Box<dyn RootFSItf>,
}

impl RfsScan {
    /// Create root filesystem scanner
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

    /// Get a package list of the root filesystem
    pub fn get_pkg_list(&self) -> Vec<String> {
        self.pkgscan.get_pkg_list()
    }

    /// Get claimed licence for a particular package on the root filesystem
    pub fn get_pkg_license(&self, pkgname: String) -> PkgLicence {
        self.pkgscan.get_pkg_license(pkgname)
    }
}
