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
}

impl State {
    pub fn init(t : Timer) -> Self {
        State::Idle(t)
    }
}

impl Stateful<Event> for State {
    fn next(self, e: &Event) -> Option<Self> {
        match self{
            State::Idle(t) => match e {
                Event::Start => Some(State::Running(e.trigger(t))),
                _ => None,
            },
            State::Running(t) => match e {
                Event::Stop => Some(State::Idle(e.trigger(t))),
                Event::Tick => Some(State::Running(e.trigger(t))),
                _ => None,
            },
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn fsm_none_on_wrong_input(){
        let s = State::init(Timer::new(std::time::Duration::new(1,0)));//in Idle state
        assert!(s.next(&Event::Stop).is_none());
    }
}