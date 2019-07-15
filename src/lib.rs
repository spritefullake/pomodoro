use std::thread;
use std::time;
use std::sync::mpsc;

//// A Pomodoro consists of the task 
/// needed to be completed in that time
struct Pomodoro {
    task: Task,
    duration: time::Duration,
}

struct Task{
    title: String,
    notes: String,
    completed: bool,
}

/// TODO: add channels that can send updates of the timer to the main thread
/// or try a shared memory solution
impl Pomodoro {
    /// Begins the pomodoro timer with periodic updates sent every second. 
    /// Displaying responsibility falls on the receiver.
    /// There will be one pomodoro timer running at a time, so message passing makes sense.
    fn start(mut self, tx: mpsc::Sender<u64>) -> thread::JoinHandle<Self> {
        thread::spawn(move || {
            while self.duration.as_secs() > 0 {
                thread::sleep(time::Duration::from_secs(1));
                self.decrement_seconds(1);

                tx.send(self.duration.as_secs()).unwrap();
            };
            // When the timer ends:
            self.task.completed = true;
            self
        })
    }
    fn decrement_seconds(&mut self, amount : u64) -> &mut Self{
        self.duration = 
        time::Duration::from_secs(self.duration.as_secs()  - amount);
        self
    }
}