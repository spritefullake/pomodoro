mod input;
mod pomodoro;
use std::fs;
use std::time::Duration;
use std::error::Error;

/// TODO: Coordinate the inputs and pomodoro integration
pub fn run(config: input::Config) -> Result<(), Box<dyn Error>> {
    let filename = config.filename;
    let duration = Duration::from_secs(config.seconds);

    let contents = fs::read_to_string(filename).unwrap_or_else(|error| {
        String::from("Cannot read the file into a string!")
    });
    
    unimplemented!()

}