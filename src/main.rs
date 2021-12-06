#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::{env, process};

use ru_storeman::{
    Config,
    fn_help::help,
    fn_start::start,
    fn_version::version,
    signal::notify_channel,
};

#[allow(unused_variables)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", process::id());
    let args = env::args();

    let config = Config::new(args).unwrap_or_else(|err| {
        log::error!(" parsing arguments: {}", err);
        process::exit(-1);
    });

    let help_fn = || {
        let help_str = help();
        eprintln!("{}", help_str)
    };

    match config.cmd.to_lowercase().as_str() {
        "start" => {
            let rx = notify_channel();
            start(rx)
        }
        "run" => {}
        "help" => {
            help_fn()
        }
        "version" => {
            println!("{}", version())
        }
        _ => {
            help_fn()
        }
    }
    Ok(())
}
