use crate::fsm::Trigger;
use super::timer::Timer;

#[derive(Debug)]
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
            _ => t,
        }
    }
}