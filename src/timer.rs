use std::thread;
use std::time;
use std::sync::{Arc,mpsc};

/// Manages the timekeeping of a pomodoro. 
pub struct Timer {
    duration: time::Duration,
    tx: mpsc::Sender<Response>,
    rx: mpsc::Receiver<Request>,

}


/// TODO: add channels that can send updates of the timer to the main thread
/// or try a shared memory solution
impl Timer {
    /// Begins the pomodoro timer with periodic updates sent every second. 
    /// Displaying responsibility falls on the receiver.
    /// There will be one pomodoro timer running at a time, so message passing makes sense.
    fn start(mut self) -> thread::JoinHandle<Response> {
        thread::spawn(move || {
            unimplemented!();
        })
    }
    fn decrement_seconds(&mut self, amount : u64) -> &mut Self{
        self.duration -= time::Duration::new(amount,0);
        self
    }
}

/// The events emitted during the lifecyle of the timer.
/// Enums with data contain the remaining duration of the timer
pub enum Request{
    Start,
    Pause,
    Reset(time::Duration),
    End,
}
pub enum Response{
    Starting,
    Ticking(time::Duration),
    Pausing(time::Duration),
    Resetting,
    Ending,
}