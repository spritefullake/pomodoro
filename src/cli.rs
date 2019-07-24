use super::{
    controllable::{self, Controllable},
    controller::{Controller, Request, Response},
    pomodoro::Pomodoro,
    task::Task,
    timer::Timer,
};
use std::{
    collections::VecDeque,
    env,
    error::Error,
    fs,
    io::{self, ErrorKind, Write},
    process,
    time::{self, Duration},
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
    let contents = fs::read_to_string(filename).unwrap();

    let make_task = |line| Task::new(String::from(line));

    let mut pomo = Pomodoro {
        tasks: VecDeque::new(),
    };

    contents
        .lines()
        .map(|line| {
            // Tasks will be kept on a queue and popped when completed
            let task = make_task(line);

            println!("Adding {}", &task);
            pomo.tasks.push_back(task);
        })
        .for_each(drop);

    let tmr = Timer::new(duration, String::from("timer"));
    let cont: Controller = tmr.controlled().unwrap();

    command_loop(cont, pomo);
}

pub fn command_loop(c: Controller, p: Pomodoro) -> io::Result<()> {
    let mut command = String::new();
    let prompt = ">>>>  ";
    //pair with writeln! to avoid unnecessary flushing from println!
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    loop {
        //pair print!s with stdout().flush() to ensure the prompt shows before reading in the buffer
        print!("\n{}", &prompt);
        io::stdout().flush().expect("Failed to flush stdout");

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");
        let trimmed = command.trim();

        // TODO: create mapping of commands to using the pomodoro / timer / controller api
        // Implement add, help, commands
        match trimmed {
            "start" => {
                writeln!(handle, "Starting the timer")?;
                c.start()
                    .map(|res| writeln!(handle, "The response is {:?}", res))
                    .expect("Starting Error!")?;
            }
            "pause" => {
                writeln!(handle, "Pausing the thread")?;
            }
            "current" => {
                let current = p.current();
                match current {
                    Some(task) => writeln!(handle, "The current task is:\n {}", task)?,
                    None => writeln!(handle, "There is no task currently!")?,
                }
            }
            "tasks" => {
                let max_width = p.tasks.iter().map(|task| task.title.len()).max().unwrap_or(0);
                writeln!(handle, "{1:^0$} | {2}", max_width, "Tasks", "Complete?");
                
                p.tasks
                    .iter()
                    .map(|task| format_task(task, max_width))
                    .map(|task| writeln!(handle, "{}", task))
                    .for_each(drop);
            }
            "timer" => {
                let result = c.info();
                match result {
                    Ok(res) => match res {
                        Response::Ticking(duration) => writeln!(
                            handle,
                            "The timer has {} seconds remaining",
                            duration.as_secs()
                        )?,

                        _ => writeln!(handle, "No tick currently!")?,
                    },
                    _ => writeln!(handle, "No tick!")?,
                }
            }
            _ => writeln!(handle, "'{}' is not a valid command!", trimmed)?,
        }
        command.clear();
    }
}

pub fn format_task(task: &Task, width: usize) -> String{
    let mut completion = " ";
    if task.is_complete() {
        completion = "✓";
    };

    format!("{1:0$} | [{2}]",width,task.title,completion)
}