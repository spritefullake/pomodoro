use std::{
    sync::mpsc,
    error::Error,
};
///Sets up a mailbox for sending messages **to** some place 
/// and receiving the messages **from** *some other* place.
pub struct Controller<S: Send, R: Send> {
    pub tx: mpsc::Sender<S>,
    pub rx: mpsc::Receiver<R>,
}

/// TODO: implement error handling/propagation and improve the return type
impl<'a, S, R> Controller<S,R> 
where S: Send + 'a, R: Send
{
    // Sending messages to the controlled actor
    // Each sending action should return the controlled agent's response

    // TODO: Handle errors in here instead of the CLI

    pub fn send(&self, request: S) -> Result<R, Box<dyn Error + 'a>>
    {
        self.tx.send(request)?;

        //controller waits/blocks on a response from the controlled agent
        let response = self.rx.recv()?;
        Ok(response)
    }
    
    pub fn from(tx: mpsc::Sender<S>, rx: mpsc::Receiver<R>) -> Self
    {
        Self {
            tx,
            rx,
        }
    }
}

///Returns a pair of Controllers that have their mailboxes linked
pub fn new_pair<S: Send, R: Send>() -> (Controller<S,R>,Controller<R,S>){
    //tx sends items received by rx_other
    //tx_other sends items received by rx
    let (tx, rx_other) = mpsc::channel::<S>();
    let (tx_other, rx) = mpsc::channel::<R>();

    (
        Controller::from(tx,rx),
        Controller::from(tx_other, rx_other)
    )
}