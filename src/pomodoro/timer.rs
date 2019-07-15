use std::thread;
use std::time;
use std::sync::mpsc;

/// Manages the timekeeping of a pomodoro. 
pub struct Timer {
    duration: time::Duration,
}

/// TODO: add channels that can send updates of the timer to the main thread
/// or try a shared memory solution
impl Timer {
    pub fn new(duration: time::Duration) -> Self {
        Self{
            duration
        }
    }
    /// Begins the pomodoro timer with periodic updates sent every second. 
    /// Displaying responsibility falls on the receiver.
    /// There will be one pomodoro timer running at a time, so message passing makes sense.
    pub fn start(mut self, tx: mpsc::Sender<u64>) -> thread::JoinHandle<Self> {
        thread::spawn(move || {
            while self.duration.as_secs() > 0 {
                thread::sleep(time::Duration::from_secs(1));
                self.decrement_seconds(1);

                tx.send(self.duration.as_secs()).unwrap();
            };
            // When the timer ends:
            tx.send(0).unwrap();
            self
        })
    }
    fn decrement_seconds(&mut self, amount : u64) -> &mut Self{
        self.duration = 
        time::Duration::from_secs(self.duration.as_secs()  - amount);
        self
    }
}