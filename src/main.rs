use std::sync::mpsc; 
use std::time;
   
mod pomodoro;

/// TODO: Develop an input method for tasks 
/// and a display method for pomodoros
/// TODO: Error handling
fn main() {
    //The main way to receive updates from pomodoros
    let (tx, rx) = mpsc::channel();

    let (task1, timer1) = pomodoro::new(String::from("Mowing the lawn"), time::Duration::from_secs(5));

    println!("Let us do this pomodoro!");

    let thread1 = timer1.start(tx);

    
    for received in rx {
        println!("Received {}",received);
    }
 
    println!("Now your pomodoro has ended!");


}
