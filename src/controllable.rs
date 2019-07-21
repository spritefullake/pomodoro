use std::sync::mpsc;
use std::thread;
use std::time;

use super::timer::{Timer};
use super::controller::{Controller, Response, Request};
use std::error::Error;


/// Allows use of an api-controlled event system with message passing
pub trait Controllable {
    fn controlled(self) -> Result<Controller, Box<dyn Error> >;
    /// Activates the event loop for the controlled agent
    fn turn_on(self, tx: mpsc::SyncSender<Response>, rx: mpsc::Receiver<Request>) 
            -> Result<thread::JoinHandle<()>, Box<dyn Error> >;
}

impl Controllable for Timer {
    /// Generates the Controller for the controlled object
    fn controlled(self) -> Result<Controller, Box<dyn Error> > {
        // Rendezvous channels; sends from these block the current thread until received
        let (control_tx, controlled_rx) = mpsc::sync_channel::<Request>(0);
        let (controlled_tx, control_rx) = mpsc::sync_channel::<Response>(0);

        let clone_tx = mpsc::SyncSender::clone(&controlled_tx);

        self.turn_on(controlled_tx, controlled_rx)?;

        let controller = 
        Controller {
            control_tx,
            control_rx,

            controlled_tx: clone_tx, 
        };

        Ok(controller)

    }
    
    /// Begins the other thread, from which the controlled agent can receive requests
    fn turn_on(mut self, tx: mpsc::SyncSender<Response>, rx: mpsc::Receiver<Request>) 
    -> Result<thread::JoinHandle<()>, Box<dyn Error> > {
        
        let t = thread::Builder::new().name(self.name.clone())
        .spawn(move || {
            //block thread; waiting for start signal
            for received in &rx {
                if let Request::Start = received{
                    tx.send(Response::Starting).unwrap();
                    break;
                }
            };

            while self.duration.as_secs() > 0 {

                let received = &rx.try_recv().unwrap_or_else(|_| Request::Continue);

                if let Request::Pause = received {
                    tx.send(Response::Pausing(thread::current())).unwrap();
                    thread::park();
                };

                self.decrement_seconds(1);
                tx.send(Response::Ticking(self.duration)).unwrap();
                thread::sleep(time::Duration::new(1, 0));
            }
            
            tx.send(Response::Ending).unwrap();
        })?;

        Ok(t)
    }
}
