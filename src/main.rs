mod clidef;
mod licences;
mod pkgcache;
mod rfs;

#[allow(unused_imports)]
use askalono::TextData;
use clap::Error;
use rfs::RfsScan;

#[allow(unused_imports)]
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::exit,
    str::FromStr,
};

static VERSION: &str = "0.1";
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

    if !fs::metadata(&rootfs).is_ok() {
        return {
            println!("Path {} does not exist", rootfs);
            Ok(())
        };
    }

    // Fix default path
    if rootfs == "/" {
        rootfs = "".to_string();
    }

    let mut x = RfsScan::new(PathBuf::from_str(&rootfs).unwrap())?;
    for p in x.get_pkg_list() {
        println!("{}", p);
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
