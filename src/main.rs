use std::sync::mpsc::channel;
use std::time;
use std::thread;

mod cli;
mod controller;
mod controllable;
mod timer;
use controllable::{Controllable};


/// TODO: Develop an input method for tasks
/// and a display method for pomodoros
/// TODO: Error handling
fn main() {
    //The main way to receive updates from pomodoros is through channels

    //cli::run(std::env::args());

    let cont : controller::Controller = timer::Timer::new(time::Duration::from_secs(8),String::from("timer")).controlled().unwrap();



}
