use crate::rfs::RfsScan;
use colored::Colorize;

use super::stdfmt::DataFormatter;

pub struct TextDataFormatter<'a> {
    rfs: &'a RfsScan,
    skip_unknown: bool,
}

impl<'a> TextDataFormatter<'a> {}

impl<'a> DataFormatter<'a> for TextDataFormatter<'a> {
    fn format(&self) -> String {
        let mut out = String::from("Package                        | Licences\n-------------------------------+---------\n");
        for mut p in self.rfs.get_pkg_list() {
            let pkl = self.rfs.get_pkg_license(p.to_owned());
            let prim = pkl.get_id();
            if prim.is_empty() && self.skip_unknown {
                continue;
            }

            if p.len() > 30 {
                p = p[..30].to_string();
            }
            out.push_str(&format!(
                "{:<30} | {}{}\n",
                p.bright_white(),
                if prim.is_empty() { "Unknown".bright_red() } else { prim.bright_cyan() },
                if !pkl.get_other().is_empty() { format!(", {}", pkl.get_other().join(", ")).cyan() } else { "".bright_black() }
            ));
        }

        out
    }

    fn new(rfs: &'a RfsScan, skip_unknown: bool) -> Box<(dyn DataFormatter + 'a)> {
        Box::new(TextDataFormatter { rfs, skip_unknown })
    }
}
