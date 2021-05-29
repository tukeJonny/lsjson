mod access;
mod exitcode;

use std::env;
use std::process;

use anyhow::Result;
use clap::{crate_version, App, AppSettings, Arg};

use crate::access::get_json_keys;
use crate::exitcode::ExitCode;

fn build_app() -> App<'static, 'static> {
    let app = App::new("lsjson")
        .version(crate_version!())
        .usage("lsjson [FLAGS/OPTIONS] [<file-path>]")
        .setting(AppSettings::DeriveDisplayOrder)
        .arg(
            Arg::with_name("json-path")
                .long("json-path")
                .short("j")
                .takes_value(true)
                .value_name("JSONPATH")
                .help("jsonpath to dig in json"),
        )
        .arg(
            Arg::with_name("file-path")
                .help("the path to json file")
                .required(true),
        );

    app
}

fn run_app() -> Result<ExitCode> {
    let matches = build_app().get_matches_from(env::args_os());
    let path = matches
        .value_of("file-path")
        .expect("Failed to get path argument");

    match get_json_keys(path) {
        Ok(result) => {
            let json_keys = match matches.value_of("json-path") {
                Some(json_path) => {
                    result.iter()
                          .filter(|key| key.starts_with(json_path))
                          .cloned()
                          .collect::<Vec<String>>()
                },
                None => result
            };

            for json_path in &json_keys {
                println!("{:#}", json_path);
            }
            Ok(ExitCode::Success)
        }
        Err(e) => {
            eprintln!("Failed to get json keys: {:#}", e);
            Ok(ExitCode::Failure)
        }
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
