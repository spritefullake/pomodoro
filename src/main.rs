use std::thread;
use std::time;


/* a Pomodoro consists of the task 
needed to be completed in that time*/
struct Pomodoro {
    task: Task,
    duration: time::Duration,
}
/// TODO: add channels that can send updates of the timer to the main thread
/// or try a shared memory solution
impl Pomodoro {
    fn start(mut self) -> thread::JoinHandle<()> {
        println!("Starting pomodoro for {} seconds",self.duration.as_secs());
        thread::spawn(move || {
            while self.duration.as_secs() > 0 {
                thread::sleep(time::Duration::from_secs(1));
                self.decrement_seconds(1);

            }
        })
    }
    fn decrement_seconds(&mut self, amount : u64) -> &mut Self{
        self.duration = 
        time::Duration::from_secs(self.duration.as_secs()  - amount);
        self
    }
}

struct Task{
    title: String,
    notes: String,
    completed: bool,
}


fn main() {

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

    let thread1 = pomo1.start();

    thread1.join().expect("Thread error");
    
 
    println!("Now your pomodoro has ended!");


}
