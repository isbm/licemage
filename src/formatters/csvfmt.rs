use super::stdfmt::DataFormatter;
use crate::rfs::RfsScan;

pub struct CSVDataFormatter<'a> {
    rfs: &'a RfsScan,
    skip_unknown: bool,
}

impl<'a> CSVDataFormatter<'a> {
    fn vec2csv(&self, data: &Vec<String>) -> String {
        if data.is_empty() {
            return "".to_string();
        }

        let mut out = String::new();
        for e in data {
            if e.is_empty() {
                continue;
            }

            if e.contains(' ') {
                out.push_str(format!(",\"{}\"", e).as_str());
            } else {
                out.push_str(format!(",{}", e).as_str());
            }
        }

        out
    }
}

impl<'a> DataFormatter<'a> for CSVDataFormatter<'_> {
    fn format(&self) -> String {
        let mut out = String::from("Package,Licences\n");
        for pkn in self.rfs.get_pkg_list() {
            let pkl = self.rfs.get_pkg_license(pkn.to_owned());
            let is_known = !pkl.get_id().is_empty();
            if !is_known && self.skip_unknown {
                continue;
            }

            out.push_str(pkn.as_str());
            if is_known {
                out.push_str(format!(",{}", pkl.get_id()).as_str());
            }
            let otr = self.vec2csv(pkl.get_other());
            if !otr.is_empty() {
                out.push_str(&otr);
            }
            out.push('\n');
        }

        out
    }

    fn new(rfs: &'a RfsScan, skip_unknown: bool) -> Box<(dyn DataFormatter + 'a)> {
        Box::new(CSVDataFormatter { rfs, skip_unknown })
    }
}
