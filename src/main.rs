use std::sync::mpsc::channel;
use std::time;
use std::thread;

mod cli;
mod controller;
mod timer;

use controller::Controllable;


/// TODO: Develop an input method for tasks
/// and a display method for pomodoros
/// TODO: Error handling
fn main() {
    //The main way to receive updates from pomodoros is through channels

    //cli::run(std::env::args());

    let cont : controller::Controller = timer::Timer::new(time::Duration::from_secs(8),String::from("timer")).controlled();


    cont.start();
    println!("Just the main thread stopping through");
    std::thread::sleep(time::Duration::from_secs(2));
    cont.stop();
    thread::sleep(time::Duration::from_secs(2));
    cont.unpause();
    println!("This is the main");

    thread::sleep(time::Duration::from_secs(7));


}
