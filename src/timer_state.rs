mod timer;
mod state;
mod event;

use state::State;
use timer::Timer;
use event::Event;

use super::fsm::{Trigger, Stateful, Responsive};
use std::{
    time::Duration,
};

pub struct TimerState{
    timer: Timer,
    state: State,
}

impl TimerState{
    pub fn new(d: Duration, name: String) -> Self 
    {
        Self {
            timer: Timer::new(d, name),
            state: State::init(),
        }
    }
    pub fn get_state(&self) -> State {
        self.state
    }
}

impl Responsive<Event> for TimerState{
    fn respond(&mut self, e: Event){
        let (t, s) = (self.timer, self.state);

        self.timer = e.trigger(t);
        self.state = s.next(e);
    }
}