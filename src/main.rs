use clap::Parser;
use itertools::Itertools as _;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CrateVersion {
    num: String,
    rust_version: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Crate {
    versions: Vec<CrateVersion>,
}

#[derive(Parser)]
struct Cli {
    crate_name: String,
}

fn main() {
    let args = Cli::parse();
    let crate_name = args.crate_name;

    let client = reqwest::blocking::Client::new();
    let crate_info: Crate = client
        .get(format!("https://crates.io/api/v1/crates/{crate_name}"))
        .header(
            "user-agent",
            "msrv-bump (https://github.com/tyilo/msrv-bump)",
        )
        .send()
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .unwrap();

    for (version, prev_version) in crate_info.versions.iter().tuple_windows() {
        if version.rust_version != prev_version.rust_version {
            println!(
                "{} -> {}: {:?} -> {:?}",
                prev_version.num, version.num, prev_version.rust_version, version.rust_version
            );
        }
    }
}
