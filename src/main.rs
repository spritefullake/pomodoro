use std::sync::mpsc; 
use std::time;
   
mod pomodoro;
mod cli;

/// TODO: Develop an input method for tasks 
/// and a display method for pomodoros
/// TODO: Error handling
fn main() {
    //The main way to receive updates from pomodoros
    let (tx, rx) = mpsc::channel::<u64>();

    let (task1, timer1) = pomodoro::new(String::from("Mowing the lawn"), time::Duration::from_secs(5));

    cli::run(cli::config::Config::new(std::env::args()).unwrap());

}
