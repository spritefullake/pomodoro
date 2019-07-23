use std::time;


/// Manages the timekeeping of a pomodoro.
pub struct Timer {
    pub duration: time::Duration,
    // is this field necessary?
    pub name: String,
}

/// TODO: add channels that can send updates of the timer to the main thread
/// or try a shared memory solution
///
/// The only responsibility of the timer should be to keep track of the time
impl Timer {
    pub fn new(duration: time::Duration, name: String) -> Self{
        Timer {
            duration,
            name,
        }
    }

    pub fn decrement_seconds(&mut self, amount: u64) -> &mut Self {
        self.duration -= time::Duration::new(amount, 0);
        self
    }
}