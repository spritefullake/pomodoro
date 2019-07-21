use std::fs;
use std::time::Duration;
use std::error::Error;
use std::env;

pub mod config;
/// TODO: Coordinate the inputs and pomodoro integration
pub fn run(args: env::Args) -> () {
    let config = config::Config::new(args).unwrap();
    let filename = config.filename;
    let duration = Duration::from_secs(config.seconds);

    let contents = fs::read_to_string(filename).expect("File read/open error!");
    // time::Duration implements copy!
    // iterators are lazy so currently won't run!
    contents.lines().map(|line| println!("The line is {}",line));

}