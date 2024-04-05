use clap::{Arg, Command};
use colored::Colorize;

pub fn cli(version: &'static str) -> Command {
    Command::new("licemage")
        .version(version)
        .about("licemage - image license scanner")
        .arg(Arg::new("path").short('p').long("path").required(true).help("Path to a mounted rootfs"))
        .arg(Arg::new("claimed").short('c').long("claimed").action(clap::ArgAction::SetTrue).help(format!(
            "Extract only published licences, provided by a package metadata.\n{}",
            "WARNING: package metadata might be inaccurate!\n".bright_yellow()
        )))
        .arg(
            Arg::new("known-only")
                .short('s')
                .long("known-only")
                .action(clap::ArgAction::SetTrue)
                .help("Suppress unknown licences"),
        )
        .arg(
            Arg::new("examine")
                .short('e')
                .long("examine")
                .action(clap::ArgAction::SetTrue)
                .help("Extract sources for all installed packages and find their licenses"),
        )
        .arg(Arg::new("temp").short('t').long("temp").help("Specify other location for caching temporary data"))
}
