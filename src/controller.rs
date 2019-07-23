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
    // Do I need this? Seems coupled
    pub controlled_tx: mpsc::SyncSender<Response>,
    
}
/* 
    If user clicks start button
    timer gets messaged to start
    timer returns back that it is starting
    MESSAGE PASSING (alternative is called shared memory)
    Timer will be running in a different thread
    We are running timer in a different thread because we do not want timer
    to interfere with the main thread (so the main thread does not get stopped/blocked since
    we would be waiting for timer) 
 */

/// The output from any controller request is the response from the controlled agent
type Output =  Result< Response, Box<dyn Error> >;
/// TODO: implement error handling/propagation and improve the return type
impl Controller {
    // Sending messages to the controlled actor
    // Each sending action should return the controlled agent's response

    // TODO: Handle errors in here instead of the CLI

    /// Sends a request to the controlled agent and returns the response
    fn send(&self, req: Request) -> Output{
        self.control_tx.send(req)?;

        let res = self.control_rx.recv()?;
        Ok(res)
    }
    
    /// Begin the associated controlled agent event loop
    pub fn start(&self) -> Output{
        self.send(Request::Start)
    }
    pub fn end(&self) -> Output{
        self.send(Request::End)
    }
    pub fn info(&self) -> Output {
        self.send(Request::Info)
    }
    pub fn pause(&self) -> Output {
        self.send(Request::Pause)
    }

    // Actually controlling the controlled actor
    // Should I add an unpause response from the controlled ?
    pub fn unpause(&self){
        let received = self.control_rx.recv().unwrap();
        if let Response::Pausing(thread) = received {
            thread.unpark();
        }
    }
    

}


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
    Reset(time::Duration),
    End,
}
/// Sent from the controlled thread to the control thread
#[derive(Debug)]
pub enum Response {
    Starting,
    Ticking(time::Duration),
    Pausing(thread::Thread),
    Resetting,
    Ending,
}