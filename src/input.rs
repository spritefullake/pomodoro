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
    sync::{Arc, Mutex},
};

pub fn begin(duration : Duration, p : &mut Arc<Mutex<Pomodoro>>){
    
    let t = Timer::new(duration, String::from("main"));
    let mut c = Controller::new(t).unwrap();


    c.start();


    for received in &c.control_rx {
        if let Response::Ending = received {
            p.lock().unwrap().complete_next();
            c.reset(duration);
        }
    };

}