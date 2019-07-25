use super::{
    controllable::{self, Controllable},
    controller::{Controller},
    pomodoro::Pomodoro,
    task::Task,
    timer::Timer,
    events::{Request, Response},
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
    let cont: Controller = Controller::control(tmr).unwrap();

    command_loop(cont, &mut pomo);
}

pub fn command_loop(c: Controller, p: &mut Pomodoro) -> io::Result<()> {
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

        let default_width = 8;

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
                    Some(task) => writeln!(handle, "{}", format_task(task, default_width))?,
                    None => writeln!(handle, "There is no current task!")?,
                }
            }
            "complete" => match p.complete_next() {
                Some(task) => writeln!(
                    handle,
                    "Just completed: {}",
                    format_task(task, default_width)
                )?,
                None => writeln!(handle, "No more tasks to complete!")?,
            },
            "pop" => match p.tasks.pop_front() {
                Some(task) => {
                    writeln!(handle, "Just popped: {}", format_task(&task, default_width))?
                }
                None => writeln!(handle, "No more tasks to pop!")?,
            },
            "tasks" => {
                format_tasks(p).iter().for_each(|line| println!("{}", line));
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

                        Response::Resetting => writeln!(handle, "The timer is resetting")?,

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

pub fn format_task(task: &Task, width: usize) -> String {
    let mut completion = " ";

    if task.is_complete() {
        completion = "✓";
    };

    format!("{1:0$} | [{2}]", width, task.title, completion)
}
pub fn format_tasks(p: &Pomodoro) -> Vec<String> {
    let task_width = p
        .tasks
        .iter()
        .map(|task| task.title.len())
        .max()
        .unwrap_or(0);
    let current_marker = "<<";
    let delimiter = "\n";
    let header = format!(
        "{1:^0$} | {2}{3}",
        task_width, "Tasks", "Complete?", delimiter
    );

    let tasks = p
        .tasks
        .iter()
        .map(|task| {
            let formatted = format_task(task, task_width);
            if let Some(current) = p.current() {
                //check pointer equality since == not implemented on Tasks
                if current as *const _ == task as *const _ {
                    format!("{}{}{}", formatted, current_marker, delimiter)
                } else {
                    format!("{}{}", formatted, delimiter)
                }
            } else {
                format!("{}{}", formatted, delimiter)
            }
        })
        .collect();

    vec![header, tasks]
}
