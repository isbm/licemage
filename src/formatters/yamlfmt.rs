use crate::rfs::RfsScan;

use super::stdfmt::DataFormatter;

pub struct YAMLDataFormatter<'a> {
    rfs: &'a RfsScan,
}

impl YAMLDataFormatter<'_> {}

impl<'a> DataFormatter<'a> for YAMLDataFormatter<'_> {
    fn format(&self) -> String {
        let mut out = String::new();

        for p in self.rfs.get_pkg_list() {
            let pkl = self.rfs.get_pkg_license(p.to_owned());
            out.push_str(format!("{}:\n", p).as_str());
            if !pkl.get_id().is_empty() {
                out.push_str(format!("    - {}\n", pkl.get_id()).as_str());
            }
            for l in pkl.get_other() {
                if l.contains(' ') {
                    out.push_str(format!("    - \"{}\"\n", l).as_str());
                } else {
                    out.push_str(format!("    - {}\n", l).as_str());
                }
            }
        }

        out.push_str("\n");
        out
    }

    fn new(rfs: &'a RfsScan) -> Box<(dyn DataFormatter + 'a)> {
        Box::new(YAMLDataFormatter { rfs })
    }
}
