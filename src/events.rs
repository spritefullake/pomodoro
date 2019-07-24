use std::{
    time::{Duration},
    thread::{Thread},
    sync::mpsc,
};

pub type Sender = mpsc::SyncSender<Response>;
pub type Receiver = mpsc::Receiver<Request>;
/// The events emitted during the lifecyle.
/// Enums with data contain the remaining duration of the timer
/// Meant for event lifecycles with a defined Start and End
/// TODO: Implement trait objects or generic parameters for decoupling
/// Sent from the control thread to the controlled thread
pub enum Request {
    Start,
    Continue,
    Info,
    Waiting,
    Pause,
    Reset(Duration),
    End,
}
/// Sent from the controlled thread to the control thread
#[derive(Debug)]
pub enum Response {
    Starting,
    Ticking(Duration),
    Pausing(Thread),
    Resetting,
    Ending,
}