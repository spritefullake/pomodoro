/* The finite state machine for the timer */

pub mod event;

pub use event::Event;

use crate::{
    fsm::{Stateful, Trigger},
    timer::Timer,
};

///Carries a label for the current state with the data wrapped inside
#[derive(Debug, Clone)]
pub enum State{
    Idle(Timer),
    Running(Timer),
    Error,
}

impl State {
    pub fn init(t : Timer) -> Self {
        State::Idle(t)
    }
}

impl Stateful<Event> for State {
    fn next(self, e: Event) -> Self {
        match self{
            State::Idle(t) => match e {
                Event::Start => State::Running(e.trigger(t)),
                _ => State::Error,
            },
            State::Running(t) => match e {
                Event::Stop => State::Idle(e.trigger(t)),
                Event::Tick => State::Running(e.trigger(t)),
                _ => State::Error,
            },
            _ => self
        }
    }
}
