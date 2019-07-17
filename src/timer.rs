use std::sync::{mpsc, Arc};
use std::thread;
use std::time;


/// Manages the timekeeping of a pomodoro.
pub struct Timer {
    pub duration: time::Duration,
    pub tx: mpsc::Sender<Response>,
    pub rx: mpsc::Receiver<Request>,
}

pub struct Controller {
    pub tx: mpsc::Sender<Request>,
    pub rx: mpsc::Receiver<Response>,
}
impl Controller {

    pub fn start(&self) {
        self.tx.send(Request::Start);
    }
    pub fn stop(&self){
        self.tx.send(Request::Pause);
    }

    pub fn unpause(&self){
        for received in &self.rx {
            if let  Response::Pausing(thread) = received {
                thread.unpark();
            }
        };
    }

    pub fn watch(&self, timer: Timer) {
        self.start();
        timer.start();
    }
}

/// TODO: add channels that can send updates of the timer to the main thread
/// or try a shared memory solution
impl Timer {
    /// Begins the pomodoro timer with periodic updates sent every second.
    /// Displaying responsibility falls on the receiver.
    /// There will be one pomodoro timer running at a time, so message passing makes sense.
    fn start(mut self) -> thread::JoinHandle<Self> {
        thread::spawn(move || {
            self.tx.send(Response::Starting);
            while self.duration.as_secs() > 0 {
                
                if let Request::Pause = self.rx.recv_timeout(time::Duration::from_nanos(1000)).unwrap_or_else(|err| Request::Start) {
                    self.tx.send(Response::Pausing(thread::current()));
                    thread::park();
                }

                self.decrement_seconds(1);
                self.tx.send(Response::Ticking(self.duration));
                thread::sleep(time::Duration::new(1, 0));
            }

            self
        })
    }

    fn decrement_seconds(&mut self, amount: u64) -> &mut Self {
        self.duration -= time::Duration::new(amount, 0);
        self
    }
}

/// The events emitted during the lifecyle of the timer.
/// Enums with data contain the remaining duration of the timer
pub enum Request {
    Start,
    Pause,
    Reset(time::Duration),
    End,
}
pub enum Response {
    Starting,
    Ticking(time::Duration),
    Pausing(thread::Thread),
    Resetting,
    Ending,
}
