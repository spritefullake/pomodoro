use std::sync::mpsc; 
use std::time;
   
mod cli;

/// TODO: Develop an input method for tasks 
/// and a display method for pomodoros
/// TODO: Error handling
fn main() {
    //The main way to receive updates from pomodoros
    let (tx, rx) = mpsc::channel::<u64>();

    cli::run(std::env::args());

}
