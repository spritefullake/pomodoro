use super::{
    mailbox::{Mailbox},
    pomodoro::Pomodoro,
    task::Task,
    timer::Timer,
    timer_state::{Event, State},
    input
};
use std::{
    collections::VecDeque,
    env,
    error::Error,
    fs,
    io::{self, ErrorKind, Write},
    process,
    time::{self, Duration},
    thread,
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

    let mut pomo = lines_to_pomodoro(contents);

    let c = input::begin(Timer::new(duration), &mut pomo);
    command_loop(c.unwrap(), &mut pomo);
}

pub fn lines_to_pomodoro(lines: String) -> Pomodoro {
    let make_task = |line| Task::new(String::from(line));

    let mut p = Pomodoro::new(VecDeque::new());

    lines
        .lines()
        .map(|line| {
            // Tasks will be kept on a queue and popped when completed
            let task = make_task(line);
            p.tasks.push_back(task);
        })
        .for_each(drop);
    
    p
}

pub fn command_loop(c: Mailbox<Event, State>, p: &mut Pomodoro) -> io::Result<()> {
    unimplemented!();      
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
