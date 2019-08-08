use std::{error::Error, thread};
/// Allows use of an api-controlled event system with message passing
/// 
/// TODO: rename and recomment
pub trait Controllable 
where Self : std::marker::Sized
{   
    type Data;
    /// Activates the event loop for the controlled agent
    fn activate(self, d: Self::Data) -> Result<thread::JoinHandle<Self>, Box<dyn Error>>;
}