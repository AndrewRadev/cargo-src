use cargo::core::Workspace as CargoWorkspace;
use cargo::ops::load_pkg_lockfile as load_cargo_lockfile;
use cargo::util::config::Config as CargoConfig;
use cargo::util::{hex, CargoResult};
use clap::{Arg, App, AppSettings, crate_version};

use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use std::process;

const DESCRIPTION: &'static str =
    "A third-party cargo extension that lists dependencies' source locations";

fn main() {
    let outer_matches = App::new("cargo-local")
        .about(DESCRIPTION)
        .version(&crate_version!()[..])
        // We have to lie about our binary name since this will be a third party
        // subcommand for cargo, this trick learned from cargo-outdated
        .bin_name("cargo")
        // We use a subcommand because parsed after `cargo` is sent to the third party plugin
        // which will be interpreted as a subcommand/positional arg by clap
        .subcommand(App::new("local").about(DESCRIPTION)
                    .arg(Arg::new("PACKAGE")
                         .help("Individual packages to show the source locations of")
                         .multiple_occurrences(true)
                         .required(false))
                    .arg(Arg::new("only-names")
                         .long("only-names")
                         .help("Only list package names")))
        .setting(AppSettings::SubcommandRequired)
        .get_matches();
    let arg_matches = outer_matches.subcommand_matches("local").unwrap();

    let src_dirs = match cargo_dirs() {
        Ok(Some(dirs)) => dirs,
        Ok(None) => {
            eprintln!("Error: Couldn't detect Cargo project in the current directory");
            process::exit(1);
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        },
    };

    if let Some(package_names) = arg_matches.values_of("PACKAGE") {
        for package_name in package_names {
            match src_dirs.get(package_name) {
                Some(dir) => {
                    if arg_matches.is_present("only-names") {
                        println!("{}", package_name)
                    } else {
                        println!("{}", dir.display())
                    }
                },
                None => {
                    eprintln!("Warning: Couldn't find local dir for package: {}", package_name);
                }
            }
        }
    } else {
        for (package_name, dir) in src_dirs {
            if arg_matches.is_present("only-names") {
                println!("{}", package_name);
            } else {
                println!("{}", dir.display());
            }
        }
    }
}

fn cargo_dirs() -> CargoResult<Option<HashMap<String, PathBuf>>> {
    // Load the current project's dependencies from its Cargo manifest
    let manifest_path     = "Cargo.toml";
    let manifest_path     = Path::new(&manifest_path);
    let manifest_path_buf = absolutize(manifest_path.to_path_buf());
    let manifest_path     = manifest_path_buf.as_path();

    let cargo_config = CargoConfig::default().expect("cargo_config");
    let workspace = CargoWorkspace::new(&manifest_path, &cargo_config)?;

    let resolved = match load_cargo_lockfile(&workspace)? {
        Some(r) => r,
        None => return Ok(None),
    };

    // Build registry_source_path the same way cargo's Config does as of
    // https://github.com/rust-lang/cargo/blob/176b5c17906cf43445888e83a4031e411f56e7dc/src/cargo/util/config.rs#L35-L80
    let cwd                  = env::current_dir()?;
    let cargo_home           = env::var_os("CARGO_HOME").map(|home| cwd.join(home));
    let user_home            = ::dirs::home_dir().map(|p| p.join(".cargo")).expect("user_home");
    let home_path            = cargo_home.unwrap_or(user_home);
    let registry_source_path = home_path.join("registry").join("src");

    let paths = resolved.iter().flat_map(|pkgid| {
        // Build src_path the same way cargo's RegistrySource does as of
        // https://github.com/rust-lang/cargo/blob/176b5c17906cf43445888e83a4031e411f56e7dc/src/cargo/sources/registry.rs#L232-L238
        let hash     = hex::short_hash(&pkgid.source_id());
        let ident    = pkgid.source_id().url().host()?.to_string();
        let part     = format!("{}-{}", ident, hash);
        let src_path = registry_source_path.join(&part);

        // Build the crate's unpacked destination directory the same way cargo's RegistrySource does as
        // of https://github.com/rust-lang/cargo/blob/176b5c17906cf43445888e83a4031e411f56e7dc/src/cargo/sources/registry.rs#L357-L358
        let dest = format!("{}-{}", pkgid.name(), pkgid.version());
        let full_path = src_path.join(&dest);

        if full_path.exists() {
            Some((pkgid.name().to_string(), full_path))
        } else {
            None
        }
    }).collect();

    Ok(Some(paths))
}

fn absolutize(pb: PathBuf) -> PathBuf {
    if pb.as_path().is_absolute() {
        pb
    } else {
        std::env::current_dir().expect("current_dir").join(&pb.as_path()).clone()
    }
}
