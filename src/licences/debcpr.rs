use super::cpritf::PkgLicenceItf;
use crate::licences::PkgLicence;
use regex::Regex;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufRead, BufReader},
    path::PathBuf,
};

/// Template where to find a debian copyright format for a specific package
static PKG_COPYRIGHT_TPL: &str = "/usr/share/doc/{pkg}/copyright";
static LICENCE_MARKER: &str = "License:";

pub struct DebPkgLicense {
    pth: PathBuf,
    first_licence: String,
    licenses: HashMap<String, bool>,
}

impl DebPkgLicense {
    pub fn new(pkgname: String) -> Self {
        DebPkgLicense {
            pth: PathBuf::from(&PKG_COPYRIGHT_TPL.replace("{pkg}", &pkgname)),
            licenses: HashMap::default(),
            first_licence: "".to_string(),
        }
        .parse_copyright()
    }

    fn parse_copyright(mut self) -> Self {
        if fs::metadata(&self.pth).is_err() {
            return self;
        }
        let f = File::open(self.pth.as_path().to_str().unwrap());
        if f.is_err() {
            // Log or something?
            return self;
        }

        let r = Regex::new(r" (?i)or | and ").unwrap();
        for l in BufReader::new(f.unwrap()).lines().map(|f| f.unwrap_or("".to_string())) {
            if l.starts_with(LICENCE_MARKER) {
                for l_title in r
                    .replace_all(&l.replace(LICENCE_MARKER, ""), |_: &regex::Captures| ", ")
                    .to_string()
                    .replace(['(', ')'], "")
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect::<Vec<String>>()
                {
                    if self.first_licence.is_empty() {
                        self.first_licence = l_title;
                    } else {
                        self.licenses.insert(l_title, false);
                    }
                }
            }
        }

        self
    }
}

impl PkgLicenceItf for DebPkgLicense {
    fn into(self) -> PkgLicence {
        let mut pkl = PkgLicence::new(self.first_licence);
        let mut liclist = self.licenses.into_keys().collect::<Vec<String>>();
        liclist.sort();
        for l in liclist {
            pkl.add(l.to_string());
        }

        pkl
    }
}
