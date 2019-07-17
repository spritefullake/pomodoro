use std::sync::mpsc::channel; 
use std::time;
   
mod cli;
mod timer;

use std::thread;

/// TODO: Develop an input method for tasks 
/// and a display method for pomodoros
/// TODO: Error handling
fn main() {
    //The main way to receive updates from pomodoros is through channels

    //cli::run(std::env::args());

    let (txTimer, rxTimer) = channel::<timer::Response>();
    let (txCont, rxCont) = channel::<timer::Request>();

    

    let t = timer::Timer{
        tx: txTimer,
        rx: rxCont,
        duration: time::Duration::from_secs(7)
    };

    let cont = timer::Controller{
        tx: txCont,
        rx: rxTimer,
        timer: t,
    };

    cont.watch(t);

    std::thread::sleep(time::Duration::from_secs(2));
    cont.stop();
    thread::sleep(time::Duration::from_secs(2));
    cont.unpause();

    thread::sleep(time::Duration::from_secs(7));


}
