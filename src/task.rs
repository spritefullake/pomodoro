/// Represents an item that is desired to be completed
/// at some point in the future.
#[derive(Debug)]
pub struct Task{
    pub title: String,
    completed: bool,
}

impl Task{
    pub fn new(title: String) -> Task {
        Task{
            title,
            completed: false
        }
    }
    /// Mark a task as completed.
    pub fn complete(&mut self){
        self.completed = true;
    }
    pub fn is_complete(&self) -> bool{
        self.completed == true
    }
}
