use std::{
    fs,
    time::{self,Duration},
    error::Error,
    env,
    collections::VecDeque,
    io::{self, Write},
};
use super::{
    controller::{Response, Request, Controller},
    controllable::{self,Controllable},
    timer::{Timer},
    task::{Task},
    pomodoro::{Pomodoro}
};
//use super::tasks;

/* General flow will be: 
1. Obtain the task titles
2. Convert these into tasks 
3. Store these tasks somewhere
4. Begin a timer
5. Print out each timer tick with its associated task?
6. When the timer runs out, remove the task */

pub mod config;

/// TODO: Coordinate the inputs and pomodoro integration
pub fn run(args: env::Args) -> () {
    let config = config::Config::new(args).unwrap();
    let filename = config.filename;
    let duration = Duration::from_secs(config.seconds);

    // handle opening error and create an empty, appendable file for that
    let contents = fs::read_to_string(filename).expect("File read/open error!");

    let make_task = |line| Task::new(String::from(line));

    let mut pomo = Pomodoro {
        tasks: VecDeque::new(),
    };

    contents.lines().map(|line| {
        // Tasks will be kept on a queue and popped when completed
        let task = make_task(line);

        println!("Adding {}",&task);
        pomo.tasks.push_back(task);


    }).for_each(drop);

   
    
    let tmr = Timer::new(duration,String::from("timer"));
    let cont : Controller = tmr.controlled().unwrap();

    let mut command = String::new();
    let prompt = ">>>>  ";

    loop {
        //pair print!s with stdout().flush() to ensure the prompt shows before reading in the buffer
        print!("\n{}", &prompt);
        io::stdout().flush().expect("Failed to flush stdout");
    
        io::stdin().read_line(&mut command).expect("Failed to read line");

        let trimmed = command.trim();

        // TODO: create mapping of commands to using the pomodoro / timer / controller api
        match trimmed{
            "start" => {
                println!("Starting the timer");
                cont.start().map(|res| println!("The response is {:?}",res)).expect("Starting Error!");
            },
            "pause" => {
                println!("pausing the thread")
            },
            "current" => {
                let current =  pomo.current();
                match current {
                    Some(task) => println!("The current task is:\n {}", task),
                    None       => println!("There is no task currently!"),
                }

            },
            "tasks" => {
                &pomo.tasks.iter().map(|task| println!("{}",task)).for_each(drop);
            },
            "timer" => {
                let result = cont.info();
                match result.unwrap() {
                    Response::Ticking(duration) => println!("The timer has {} seconds remaining",duration.as_secs()),
                    _                           => println!("No tick!") 
                }
                
            },
            _ => println!("'{}' is not a valid command!", trimmed)
        }
        command.clear();
    }

}