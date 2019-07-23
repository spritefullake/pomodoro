use std::collections::VecDeque;

use super::task::Task;

/// Keeps track of the tasks to go through and the current position of the pomodoro.
pub struct Pomodoro {
    pub tasks: VecDeque<Task>,
}

impl Pomodoro{
    /// Returns the current task being worked on in the pomodoro; that is, the first non-complete task.
    pub fn current(&self) -> Option<&Task>{
        self.tasks.iter().skip_while(|task| task.is_complete()).nth(0)
    }
    // TODO: figure out if current_mut vs current breaks DRY
    fn current_mut(&mut self) -> Option<&mut Task>{
        self.tasks.iter_mut().skip_while(|task| task.is_complete()).nth(0)
    }
    /// Indicates if all of the tasks in the pomodoro have been completed.
    pub fn is_done(&self) -> bool {
        match self.tasks.iter().filter(|task| !task.is_complete()).next() {
            None => true,
            _    => false,
        }
    }
    /// Completes the next completable task on the queue 
    pub fn complete_next(&mut self) -> Option<&Task>{
        let current = self.current_mut();
        if let Some(task) = current {
            task.complete();
            Some(task)
        }
        else{
            None
        }
    }

}