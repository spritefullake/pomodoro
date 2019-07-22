use std::{
    sync::mpsc,
    thread,
    time,
    error::Error,
};


/// Exposes an api of the events sent and received to the controlled actor. 
pub struct Controller {

    /// Used by the control to message the controlled actor
    pub control_tx: mpsc::SyncSender<Request>,
    /// The mailbox for responses from the controlled actor
    pub control_rx: mpsc::Receiver<Response>,

    /// Used by the controlled to message the controller
    pub controlled_tx: mpsc::SyncSender<Response>,
    
}

/// The output for any controller request
type Output =  Result< Response, Box<dyn Error> >;
/// TODO: implement error handling/propagation and improve the return type
impl Controller {
    // Sending messages to the controlled actor
    // Each sending action should return the controlled agent's response
    
    /// Begin the associated controlled agent event loop
    pub fn start(&self) -> Output{
        self.control_tx.send(Request::Start)?;
    
        let res = self.control_rx.recv()?;
        Ok(res)
    }
    pub fn end(&self) -> Output{
        self.control_tx.send(Request::End)?;
    
        let res = self.control_rx.recv()?;
        Ok(res)
    }

    // Actually controlling the controlled actor

    pub fn unpause(&self){
        for received in &self.control_rx {
            if let  Response::Pausing(thread) = received {
                thread.unpark();
            }
        };
    }

}


/// The events emitted during the lifecyle.
/// Enums with data contain the remaining duration of the timer
/// Meant for event lifecycles with a defined Start and End
/// TODO: Implement trait objects or generic parameters for decoupling
pub enum Request {
    Start,
    Continue,
    Waiting,
    Pause,
    Reset(time::Duration),
    End,
}

#[derive(Debug)]
pub enum Response {
    Starting,
    Ticking(time::Duration),
    Pausing(thread::Thread),
    Resetting,
    Ending,
}