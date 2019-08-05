/* The finite state machine for the timer */

use super::{
    event::Event,
};
use crate::fsm::{Stateful};

#[derive(Debug, Clone)]
pub enum State{
    Idle,
    Running,
    Error,
}


impl Stateful<Event> for State {
    fn init() -> Self {
        State::Idle
    }
    
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