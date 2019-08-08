use std::time;


/// Manages the timekeeping of a pomodoro.
#[derive(Debug, Clone)]
pub struct Timer {
    pub duration: time::Duration,
}

/// The only responsibility of the timer should be to keep track of the time
impl Timer {
    pub fn new(duration: time::Duration) -> Self{
        Timer {
            duration,
        }
    }

    pub fn decrement_seconds(&mut self, amount: u64) -> &mut Self {
        self.duration -= time::Duration::new(amount, 0);
        self
    }

    pub fn set(&mut self, duration: time::Duration){
        self.duration = duration;
    }
}