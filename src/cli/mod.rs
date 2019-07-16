use super::pomodoro;
use std::fs;
use std::time::Duration;
use std::error::Error;

pub mod config;
/// TODO: Coordinate the inputs and pomodoro integration
pub fn run(config: config::Config) -> () {
    let filename = config.filename;
    let duration = Duration::from_secs(config.seconds);

    let contents = fs::read_to_string(filename).expect("File read/open error!");
    // time::Duration implements copy!
    contents.lines().map(|line| pomodoro::new(String::from(line),duration)).map(|(task,timer)| {
        println!("The task is: {}",&task);
    });

}