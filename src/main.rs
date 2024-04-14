mod clidef;
mod formatters;
mod licences;
mod pkgcache;
mod rfs;

#[allow(unused_imports)]
use askalono::TextData;
use clap::Error;
use colored::Colorize;
use rfs::RfsScan;

#[allow(unused_imports)]
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::exit,
    str::FromStr,
};

static VERSION: &str = "0.2";
#[allow(dead_code)]
fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let mut cli = clidef::cli(VERSION);
    if args.len() < 2 {
        return {
            cli.print_help().unwrap();
            Ok(())
        };
    }

    let params = cli.to_owned().get_matches();
    let mut rootfs = params.get_one::<String>("path").unwrap().to_owned(); // Won't be an Option, required.

    if fs::metadata(&rootfs).is_err() {
        return {
            println!("Path {} does not exist", rootfs);
            Ok(())
        };
    }

    // Fix default path
    if rootfs == "/" {
        rootfs = "".to_string();
    }

    // Display claimed licences
    if params.get_flag("claimed") {
        println!("Package                        | Licence                             | Other...");
        println!("-------------------------------+-------------------------------------+---------------");
        let rfs = RfsScan::new(PathBuf::from_str(&rootfs).unwrap())?;
        for mut p in rfs.get_pkg_list() {
            let pkl = rfs.get_pkg_license(p.to_owned());
            let primary = pkl.get_id();
            if p.len() > 30 {
                p = p[..30].to_string();
            }
            if params.get_flag("known-only") && !primary.is_empty() || !params.get_flag("known-only") {
                println!(
                    "{:<30} | {:<35} | {}",
                    p.bright_white(),
                    if primary.is_empty() { "Unknown".bright_red() } else { primary.bright_cyan() },
                    pkl.get_other().join(", ").cyan()
                );
            }
        }
    }

    /*
    let mut s = askalono::Store::new();
    if let Err(error) = s.load_spdx(Path::new("dataset"), false) {
        println!("Unable to load dataset: {:?}", error);
        exit(1);
    }

    let fstrp = fs::read_to_string(&args[1]);
    if let Err(error) = fstrp {
        println!("Unable to read license file: {:?}", error);
        exit(1);
    }

    let t = TextData::new(&fstrp.unwrap());
    let m = s.analyze(&t);

    println!("{:?}", m);
    */

    Ok(())
}
