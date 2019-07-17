use std::sync::mpsc; 
use std::time;
   
mod cli;
mod timer;

/// TODO: Develop an input method for tasks 
/// and a display method for pomodoros
/// TODO: Error handling
fn main() {
    //The main way to receive updates from pomodoros is through channels

    cli::run(std::env::args());

}
