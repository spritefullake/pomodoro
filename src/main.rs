use std::sync::mpsc;
use std::time;

use pomodoro_timer::{Task,Pomodoro};

/// TODO: Develop an input method for tasks 
/// and a display method for pomodoros
/// TODO: Error handling
fn main() {
    //The main way to receive updates from pomodoros
    let (tx, rx) = mpsc::channel();

    let task1 = Task{
        title: String::from("Mowing the lawn"),
        notes: String::from("Watch out for bugs!"),
        completed: false
    };

    let pomo1 = Pomodoro{
        task: task1,
        duration: time::Duration::from_secs(5),
    };

    println!("Let us do this pomodoro!");

    let thread1 = pomo1.start(tx);

    
    for received in rx {
        println!("Received {}",received);
    }
 
    println!("Now your pomodoro has ended!");

    println!("Completed? {}",thread1.join().unwrap().task.completed);


}
