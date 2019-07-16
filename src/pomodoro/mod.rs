pub mod timer;
pub mod task;
use std::time::Duration;

pub fn new(title: String, duration: Duration) -> (task::Task, timer::Timer){
    (task::Task::new(title), timer::Timer::new(duration))
}


/*

/// A Pomodoro consists of the task 
/// needed to be completed in that time
pub struct Pomodoro {
    task: Task,
    timer: Timer
}

impl Pomodoro{
    pub fn new(task: Task, duration: time::Duration) -> Self {
        Self {
            task,
            timer: Timer::new(duration),
        }
    }
}

*/

