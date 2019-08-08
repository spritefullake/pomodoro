use crate::{
    fsm::Trigger,
    timer::Timer,
};
use std::{
    time::Duration,
    thread,
};
///Events are **only** responsible for carrying their configuration for the data
/// being acted upon. Items irrelevant to changing the data are not allowed.
#[derive(Debug, Clone, Copy)]
pub enum Event{
    Start,
    Stop,
    Tick,
}

impl Trigger for Event{
    type Data = Timer;
    fn trigger(&self, t: Self::Data) -> Self::Data {
        match self {
            //precondition: the duration is greater than or equal to the
            //amount that will be subtracted from it
            //TODO: finish trigger impl and refactor
            Event::Tick => {
                thread::sleep(Duration::new(1,0));
                if t.duration.as_secs() >= 1 {
                    Timer::new(t.duration - Duration::from_secs(1))
                }
                else{
                    t
                }
                
            }
            _ => t,
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn timer_duration_stays_0_when_0(){
        let data = Timer::new(Duration::from_secs(0));
        let data_copy = data.clone();
        let event = Event::Tick;
        let new_data = event.trigger(data);
        assert_eq!(new_data.duration, data_copy.duration);
    }
    #[test]
    fn timer_duration_changes_correctly_on_tick(){
        let n = 1;
        let data = Timer::new(Duration::from_secs(n));
        let data_copy = data.clone();
        let event = Event::Tick;
        let new_data = event.trigger(data);
        //ensure that older data - newer data = n 
        //(since tick *decrements* by n)
        assert_eq!(Duration::from_secs(n),data_copy.duration - new_data.duration);
    }
}