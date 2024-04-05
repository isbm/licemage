/*
    Detect rootfs type (debian, fedora etc)
*/

use phf::phf_map;
use std::{fs, path::PathBuf};

pub static PKG_MGR_DEB: &'static str = "dpkg";
pub static PKG_MGR_RPM: &'static str = "rpm";
static PACKAGE_MANAGERS: phf::Map<&'static str, &'static str> = phf_map! {
    // Debian family
    "debian" => PKG_MGR_DEB,
    "raspbian" => PKG_MGR_DEB,
    "elementary" => PKG_MGR_DEB,
    "linuxmint" => PKG_MGR_DEB,
    "ubuntu" => PKG_MGR_DEB,
    "pop" => PKG_MGR_DEB,

    // RedHat family
    "centos" => PKG_MGR_RPM,
    "fedora" => PKG_MGR_RPM,
    "opensuse" => PKG_MGR_RPM,
    "opensuse-leap" => PKG_MGR_RPM,
    "rhel" => PKG_MGR_RPM,
    "rocky" => PKG_MGR_RPM,
    "scientific" => PKG_MGR_RPM,
    "sled" => PKG_MGR_RPM,
    "sles" => PKG_MGR_RPM,
    "sles_sap" => PKG_MGR_RPM,
};

pub struct RfsType {
    rfs_path: PathBuf,
    rfs_os_id: String,
}

impl RfsType {
    /// Create an instance of the RfsType
    pub fn new(p: PathBuf) -> Self {
        RfsType { rfs_path: p.join("/etc/os-release"), rfs_os_id: "".to_string() }.parse_osrls()
    }

    /// Parse os-release, which is at $rfs_path/etc/os-release
    fn parse_osrls(mut self) -> Self {
        if fs::metadata(self.rfs_path.as_path()).is_err() {
            return self;
        }

        if let Ok(osr) = rs_release::parse_os_release(&self.rfs_path) {
            for (k, v) in osr {
                if k.into_owned() == "ID" {
                    self.rfs_os_id = v;
                    return self;
                }
            }
        }

        return self;
    }

    /// Gets ID of the distribution
    pub fn get_os_id(&self) -> String {
        self.rfs_os_id.to_owned()
    }

    /// Returns a package manager ("dpkg" for Debian, "rpm" for Fedora etc)
    pub fn get_pkg_mgr(&self) -> String {
        if PACKAGE_MANAGERS.contains_key(&self.get_os_id()) {
            return PACKAGE_MANAGERS.get(&self.get_os_id()).unwrap().to_string();
        }

        "".to_string()
    }
}
