use super::{
    timer::Timer,
    events::{Request, Response, Sender, Receiver},
};
use std::{error::Error, thread, time};

/// Allows use of an api-controlled event system with message passing
pub trait Controllable {
    /// Activates the event loop for the controlled agent
    fn activate(
        self,
        tx: Sender,
        rx: Receiver,
    ) -> Result<thread::JoinHandle<()>, Box<dyn Error>>;

    fn change(&mut self) -> ();
}


impl Controllable for Timer {
    /// Begins the other thread, from which the controlled agent can receive requests
    //  why am I returning a JoinHandle if I don't use it?
    fn activate(
        mut self,
        tx: Sender,
        rx: Receiver,
    ) -> Result<thread::JoinHandle<()>, Box<dyn Error>> {
        let t = thread::Builder::new()
            .name(self.name.clone())
            .spawn(move || {
                // think about adding external while to check for an "end" signal
                'quitCheck: loop {
                    let start_duration = self.duration;
                    //block thread; waiting for start signal
                    wait_for_start(&tx, &rx);

                    while self.duration.as_secs() > 0 {
                        let received = &rx.try_recv().unwrap_or_else(|_| Request::Continue);

                        match received {

                            Request::Pause => {
                                let current = thread::current();
                                tx.send(Response::Pausing(current)).unwrap();
                                thread::park();
                            }

                            Request::Info => tx.send(Response::Ticking(self.duration)).unwrap(),

                            Request::End => break 'quitCheck,

                            _ => self.change()
                        }
                    }

                    if let Request::End = &rx.recv().unwrap() {
                        break 'quitCheck;
                    }
                }
                //Finally indicate the thread has ended
                tx.send(Response::Ending).unwrap();
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
