pub mod fn_help;
pub mod fn_version;

use std::env;

pub struct Config {
    pub cmd: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {

        args.next(); // args[0]

        let cmd = match args.next() { // args[1]
            Some(arg) => arg,
            None => return Err("Didn't get a cmd string"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { cmd, case_sensitive })
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;


}