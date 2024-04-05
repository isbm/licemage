use crate::licences::{cpritf::PkgLicenceItf, debcpr::DebPkgLicense, PkgLicence};

use super::rfsitf::RootFSItf;
use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind},
    path::PathBuf,
};

static DPKG_DBF: &str = "/var/lib/dpkg/status";
static PKG_MARKER: &str = "Package:";

pub struct DebRootFsScan {
    rootfs: PathBuf,
}

impl DebRootFsScan {
    /// Create a new instance of DebRootFsScan
    pub fn new(mut p: PathBuf) -> Result<Self, Error> {
        p = p.join(DPKG_DBF);

        if !p.exists() {
            return Err(Error::new(ErrorKind::NotFound, format!("{:?} not found", p.as_os_str())));
        }

        Ok(DebRootFsScan { rootfs: p })
    }
}

impl RootFSItf for DebRootFsScan {
    /// Get all packages from the current status file
    fn get_pkg_list(&self) -> Vec<String> {
        let mut out: Vec<String> = Vec::default();

        if let Ok(f) = File::open(self.rootfs.as_os_str()) {
            let r = BufReader::new(f);
            for l in r.lines().map_while(Result::ok) {
                if l.starts_with(PKG_MARKER) {
                    out.push(l.replace(PKG_MARKER, "").trim().to_string());
                }
            }
        }

        out
    }

    /// Get license(s) of a package
    fn get_pkg_license(&self, pkgname: String) -> PkgLicence {
        PkgLicenceItf::into(DebPkgLicense::new(pkgname))
    }
}
