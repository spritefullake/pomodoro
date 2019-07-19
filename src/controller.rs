use std::sync::mpsc;
use std::thread;
use std::time;

use super::timer::{Timer};


pub struct Controller {
    pub control_tx: mpsc::Sender<Request>,
    control_rx: mpsc::Receiver<Response>,

    /// Used by the controlled to message the controller
    pub controlled_tx: mpsc::Sender<Response>,
}
/// Allows use of an api-controlled event system with message passing
pub trait Controllable {
    fn controlled(self) -> Controller;
    fn turnOn(mut self, tx: mpsc::Sender<Response>, rx: mpsc::Receiver<Request>) -> Result<thread::JoinHandle<Timer>,std::io::Error>;
}

/// TODO: implement error handling/propagation and improve the return type
impl Controller {
    pub fn start(&self) -> Result<(), mpsc::SendError<Request>>{
        self.control_tx.send(Request::Start)
    }
    pub fn stop(&self) -> Result<(), mpsc::SendError<Request>>{
        self.control_tx.send(Request::Pause)
    }

    pub fn unpause(&self){
        for received in &self.control_rx {
            if let  Response::Pausing(thread) = received {
                thread.unpark();
            }
        };
    }

}

impl Controllable for Timer {
    fn controlled(self) -> Controller {
        let (control_tx, controlled_rx) = mpsc::channel::<Request>();
        let (controlled_tx, control_rx) = mpsc::channel::<Response>();

        let clone_tx = mpsc::Sender::clone(&controlled_tx);

        self.turnOn(controlled_tx, controlled_rx);

        Controller {
            control_tx,
            control_rx,

            controlled_tx: clone_tx,
        }

    }

    fn turnOn(mut self, tx: mpsc::Sender<Response>, rx: mpsc::Receiver<Request>) -> Result<thread::JoinHandle<Self>,std::io::Error> {
        thread::Builder::new().name(self.name.clone())
        .spawn(move || {
            //block thread; waiting for start signal
            for received in &rx {
                if let Request::Start = received{
                    tx.send(Response::Starting);
                    break;
                }
            };

            while self.duration.as_secs() > 0 {

                let received = &rx.try_recv().unwrap_or_else(|err| Request::Continue);

                if let Request::Pause = received {
                    tx.send(Response::Pausing(thread::current()));
                    thread::park();
                };

                self.decrement_seconds(1);
                tx.send(Response::Ticking(self.duration));
                thread::sleep(time::Duration::new(1, 0));
            }

            self
        })
    }
}

/// The events emitted during the lifecyle.
/// Enums with data contain the remaining duration of the timer
/// TODO: Implement trait objects or generic parameters for decoupling
pub enum Request {
    Start,
    Continue,
    Waiting,
    Pause,
    Reset(time::Duration),
    End,
}
pub enum Response {
    Starting,
    Ticking(time::Duration),
    Pausing(thread::Thread),
    Resetting,
    Ending,
}
