mod access;
mod exitcode;

use std::env;
use std::process;

use anyhow::Result;
use clap::{crate_version, App, AppSettings, Arg};

use crate::exitcode::ExitCode;
use crate::access::get_json_keys;


fn build_app() -> App<'static, 'static> {
    let app = App::new("lsjson")
        .version(crate_version!())
        .usage("lsjson [FLAGS/OPTIONS] [<file-path>]")
        .setting(AppSettings::DeriveDisplayOrder)
        .arg(
            Arg::with_name("file-path")
                .help("the path to json file")
                .required(true)
        );

    app
}

fn run_app() -> Result<ExitCode> {
    let matches = build_app().get_matches_from(env::args_os());
    let path = matches
        .value_of("file-path")
        .expect("Failed to get path argument");

    if let Ok(keys) = get_json_keys(path) {
        for json_path in &keys {
            println!("{:#}", json_path);
        }
        Ok(ExitCode::Success)
    } else {
        Ok(ExitCode::Failure)
    }
}

fn main() {
    let result = run_app();
    match result {
        Ok(exitcode) => {
            process::exit(exitcode.into());
        }
        Err(err) => {
            eprintln!("Failed to get exitcode: {:#}", err);
            process::exit(ExitCode::Failure.into());
        }
    }
}
