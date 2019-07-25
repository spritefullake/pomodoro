use super::{
    timer::Timer,
    events::{Request, Response},
};
use std::{error::Error, thread, time};

pub type Sender = std::sync::mpsc::Sender<Response>;
pub type Receiver = std::sync::mpsc::Receiver<Request>;

/// Allows use of an api-controlled event system with message passing
pub trait Controllable 
    where Self: std::marker::Sized
{
    /// Activates the event loop for the controlled agent
    fn activate(
        self,
        tx: Sender,
        rx: Receiver,
    ) -> Result<thread::JoinHandle<Self>, Box<dyn Error>>;

    fn change(&mut self) -> ();
}


impl Controllable for Timer {
    /// Begins the other thread, from which the controlled agent can receive requests
    //  why am I returning a JoinHandle if I don't use it?
    fn activate(
        mut self,
        tx: Sender,
        rx: Receiver,
    ) -> Result<thread::JoinHandle<Self>, Box<dyn Error>> {
        let t = thread::Builder::new()
            .name(self.name.clone())
            .spawn(move || {
                // think about adding external while to check for an "end" signal

                //block thread; waiting for start signal
                wait_for_start(&tx, &rx);

                loop{
                    let received = &rx.try_iter().last().unwrap_or(Request::Continue);
                    

                    match received {

                        Request::Pause => {
                            let current = thread::current();
                            tx.send(Response::Pausing(current)).unwrap();
                            thread::park();
                        }

                        Request::Info => tx.send(Response::Ticking(self.duration)).unwrap(),

                        Request::End => break,

                        _ => tx.send(Response::Resetting).unwrap()
                    }

                    if self.duration.as_secs() > 0 {
                        self.change();
                    }
                }
                
                //Finally indicate the thread has ended
                tx.send(Response::Ending).unwrap();
                self
            })?;

        Ok(t)
    }
    fn change(&mut self){
        // Main change applied to the data here
        // consider modularizing?
        self.decrement_seconds(1);
        thread::sleep(time::Duration::new(1, 0));
    }
}

fn wait_for_start(tx: &Sender, rx: &Receiver) {
    for received in rx {
        if let Request::Start = received {
            tx.send(Response::Starting).unwrap();
            break;
        }
    }
}
