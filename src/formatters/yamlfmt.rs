use crate::rfs::RfsScan;

use super::stdfmt::DataFormatter;

pub struct YAMLDataFormatter<'a> {
    rfs: &'a RfsScan,
    skip_unknown: bool,
}

impl<'a> DataFormatter<'a> for YAMLDataFormatter<'_> {
    fn format(&self) -> String {
        let mut out = String::new();

        for p in self.rfs.get_pkg_list() {
            let pkl = self.rfs.get_pkg_license(p.to_owned());
            let is_known = !pkl.get_id().is_empty();
            if !is_known && self.skip_unknown {
                continue;
            }
            out.push_str(format!("\n{}:\n", p).as_str());

            if is_known {
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
        out = out.trim().to_owned();
        out.push('\n');
        out
    }

    fn new(rfs: &'a RfsScan, skip_unknown: bool) -> Box<(dyn DataFormatter + 'a)> {
        Box::new(YAMLDataFormatter { rfs, skip_unknown })
    }
}
