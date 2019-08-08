use super::{
    mailbox::{self,Mailbox},
    pomodoro::Pomodoro,
    task::Task,
    timer::Timer,
    timer_state::{Event, State},
    input,
    controllable::Controllable,
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
    let t = Timer::new(duration);

    //Timer loop and main loop mailbox setup
    let (controller_mail, timer_mail) = mailbox::new_pair::<Event, State>();
    let handle = timer_mail.activate(t).unwrap();
    controller_mail.start(handle.thread()).unwrap();

    //Complete a task everytime the timer for tasks ends. Start the break timer when the pomodoro is on break.
    for state in &controller_mail.rx{
        input::sync_pomodoro_with_state(&state, &mut pomo);
        let the_time = controller_mail.react(handle.thread(), &state).unwrap();
        println!("The time is {}",the_time.duration.as_secs());     
    }

    
    //command_loop(c.unwrap(), &mut pomo);
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
