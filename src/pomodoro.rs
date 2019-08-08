use std::collections::VecDeque;

use super::task::Task;


/// Keeps track of the tasks to go through and the current position of the pomodoro.
pub struct Pomodoro {
    pub tasks: VecDeque<Task>,
    on_break: bool,
}

impl Pomodoro{
    pub fn new(tasks: VecDeque<Task>) -> Self{
        Self{
            tasks,
            on_break: false,
        }
    }
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
    /// Completes the next completable task on the queue.
    /// During breaks tasks cannot be completed. 
    pub fn complete_next(&mut self) -> Option<&Task>{
        if self.on_break {
            self.on_break = false;
            None
        }
        else{
            //should empty pomodoros go on_break?
            self.on_break = true;
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

}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn verify_all_are_done(){
        let tasks = VecDeque::from(vec![
            Task::new(String::from("task1")),
            Task::new(String::from("task2"))
        ]);
        let p = Pomodoro::new(tasks);
        assert_ne!(true, p.is_done());
    }
    #[test]
    fn complete_next_on_empty(){
        let tasks = VecDeque::from(vec![]);
        let mut p = Pomodoro::new(tasks);
        let completed_task = p.complete_next();
        //since task doesn't have equality implemented, 
        //turn Some task into something with equality
        let completed_num = match completed_task {
            Some(_) => Some(0),
            _       => None,
        };
        assert_eq!(completed_num, None);
    }
    #[test]
    fn on_break_after_task_is_completed(){
        let tasks = VecDeque::from(vec![
            Task::new(String::from("task1")),
            Task::new(String::from("task2"))
        ]);
        let mut p = Pomodoro::new(tasks);
        p.complete_next();
        assert!(p.on_break);
    }
    #[test]
    fn toggle_on_break_with_next_task(){
        let tasks = VecDeque::from(vec![
            Task::new(String::from("task1")),
            Task::new(String::from("task2"))
        ]);
        let mut p = Pomodoro::new(tasks);
        p.complete_next();//goes on break
        p.complete_next();//goes off break
        assert!(!p.on_break);
    }
}