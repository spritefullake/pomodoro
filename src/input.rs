use std::env;
use std::process;
use std::time::Duration;



pub struct Config {
    pub filename: String,
    pub seconds: u64,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str>{
        //skip first default program name arg
        args.next();

        let filename = match args.next() {
            Some(arg) => arg,
            None      => return Err("Did not receive a file name"),
        };

        let seconds = match args.next(){
            Some(arg) => arg.parse::<u64>()
            .expect("Please enter a non-negative number"),

            None      => return Err("Did not receive a specified duration"),
        };

        Ok(Config { filename,  seconds})
    }
}