extern crate log;

use std::{env, process};

use ru_storeman::Config;
use ru_storeman::{fn_help::help, fn_version::version, fn_start::start, signal::notify_channel};

#[allow(unused_variables)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args();

    let config = Config::new(args).unwrap_or_else(|err| {
        log::error!(" parsing arguments: {}", err);
        process::exit(-1);
    });

    let help_fn = ||{
        let help_str = help();
        eprintln!("{}", help_str)
    };

    match config.cmd.to_lowercase().as_str() {
        "start" => {
            let rx = notify_channel();
            start(rx)
        },
        "run" => {
        },
        "help" => {
            help_fn()
        },
        "version" => {
            println!("{}", version())
        },
        _ => {
            help_fn()
        },
    }
    Ok(())
}
