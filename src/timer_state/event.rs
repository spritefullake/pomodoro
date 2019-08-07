use crate::{
    fsm::Trigger,
    timer::Timer,
};

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
                if t.duration.as_secs() >= 1 {
                    Timer::new(t.duration - std::time::Duration::from_secs(1))
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
        let data = Timer::new(std::time::Duration::from_secs(0));
        let data_copy = data.clone();
        let event = Event::Tick;
        let new_data = event.trigger(data);
        assert_eq!(new_data.duration, data_copy.duration);
    }
    #[test]
    fn timer_duration_changes_with_tick(){
        let data = Timer::new(std::time::Duration::from_secs(1));
        let data_copy = data.clone();
        let event = Event::Tick;
        let new_data = event.trigger(data);
        assert_ne!(new_data.duration, data_copy.duration);
    }
}