/* The finite state machine for the timer */

pub mod event;
pub mod timer;

pub use event::Event;
pub use timer::Timer;

use crate::fsm::{Stateful, Trigger};

///Carries a label for the current state with the data wrapped inside
#[derive(Debug)]
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
