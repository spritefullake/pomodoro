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
    pub fn start(mut self, tx: mpsc::Sender<Event>) -> thread::JoinHandle<Event> {
        thread::spawn(move || {
            tx.send(Event::Start);

            while self.duration.as_secs() > 0 {
                thread::sleep(time::Duration::from_secs(1));
                self.decrement_seconds(1);

                // Every timer tick is sent
                tx.send(Event::Tick(self.duration)).unwrap();
            };
            // When the timer ends
            tx.send(Event::End).unwrap();
            Event::End
        })
    }
    fn decrement_seconds(&mut self, amount : u64) -> &mut Self{
        self.duration -= time::Duration::new(amount,0);
        self
    }
}

/// The events emitted during the lifecyle of the timer.
/// Enums with data contain the remaining duration of the timer
pub enum Event{
    Start,
    Tick(time::Duration),
    Pause(time::Duration),
    End,
    Reset(time::Duration),
}
