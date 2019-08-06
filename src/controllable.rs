use super::{
    timer_state::{Event, State, Timer},
    fsm::Stateful,
};
use std::{error::Error, thread, time};



/// Allows use of an api-controlled event system with message passing
/// 
/// TODO: rename and recomment
pub trait Controllable 
{   
    type Data;
    /// Activates the event loop for the controlled agent
    fn activate(self, d: Self::Data) -> Result<thread::JoinHandle<Self>, Box<dyn Error>>;
}

