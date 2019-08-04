/* The finite state machine for the timer */
use super::{
    trigger::Trigger,
    timer::Timer,
};

use std::{
    fmt::{Debug},
    clone::Clone
};

#[derive(Debug, Clone)]
enum Event{
    Start,
    Stop,
    Tick,
}

impl<Timer> Trigger<Timer> for Event {
    fn trigger(&self, t: Timer) -> Timer {
        match self {
            //precondition: the duration is greater than or equal to the
            //amount that will be subtracted from it
            //TODO: finish trigger impl and refactor
            _ => t,
        }
    }
}

#[derive(Debug, Clone)]
enum State{
    Idle,
    Running,
    Error,
}

/* the state machine only transitions to the next state;
whatever action is intended by the event is not
the responsibility of the state machine */
impl State {
    //consumption by default
    fn next(self, e: Event) -> Self {
        match self {
            State::Idle => match e {
                Event::Start => State::Running,
                _ => State::Error,
            },
            State::Running => match e {
                Event::Stop => State::Idle,
                Event::Tick => State::Running,
                _ => State::Error,
            },
            _ => self
        }
    }
}