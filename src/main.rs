use std::{
    sync::mpsc::channel,
    time,
    thread,
};

mod cli;
mod pomodoro;
mod controller;
mod controllable;
mod timer_state;
mod task;
mod events;
mod input;
use controllable::{Controllable};
mod fsm;
mod lib;

/// TODO: Develop an input method for tasks
/// and a display method for pomodoros
/// TODO: Error handling
fn main() {
    //The main way to receive updates from pomodoros is through channels

    //cli::run(std::env::args());
    

}
