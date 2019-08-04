// An abstract, platform-agnostic, pomodoro-input module
use super::{
    controller::Controller,
    timer::Timer,
    pomodoro::Pomodoro,
    events::{Response}
};
use std::{
    time::Duration,
    thread,
    collections::VecDeque,
};

pub fn begin(duration : Duration){
    
    let t = Timer::new(duration, String::from("main"));
    let mut c = Controller::new(t).unwrap();
    let mut p = Pomodoro{ tasks: VecDeque::new()};


    c.start();

    for received in &c.control_rx {
        if let Response::Ending = received {
            p.complete_next();
            c.reset(duration);
        }
    };
}