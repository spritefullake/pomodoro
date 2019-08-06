// An abstract, platform-agnostic, pomodoro-input module
use super::{
    controller,
    timer_state::{Timer, Event, State},
    pomodoro::Pomodoro,
    events::{Response},
    controllable::Controllable,
    fsm::{Stateful}
};
use std::{
    time::Duration,
    thread,
    collections::VecDeque,
    error::Error,
};

pub fn begin(duration : Duration){

    let (main_mail, timer_mail) = controller::new_pair::<Event, State>();

    let mut p = Pomodoro{ tasks: VecDeque::new()};

    let handle = timer_mail.activate(Timer::new(duration)).unwrap();
    main_mail.start(handle.thread());
}

type TimerState = Result<State, Box<dyn Error>>;
type TimerEvent = Result<Event, Box<dyn Error>>;

impl controller::Controller<Event, State> {
    pub fn start(&self, t: &thread::Thread) -> TimerState{
        //ensure the timer thread is NOT frozen
        t.unpark();
        self.send(Event::Start)
    }
    pub fn stop(&self) -> TimerState {
        self.send(Event::Stop)
    }
    pub fn tick(&self) -> TimerState {
        self.send(Event::Tick)
    }
}


impl Controllable for controller::Controller<State, Event> {
    type Data = Timer;
    ///Starts the timer thread 
    fn activate(self, d: Self::Data) 
    -> Result<thread::JoinHandle<Self>, Box<dyn Error>>
    {
        let mut state = State::init(d);
        let t = thread::spawn(move ||{
            thread::park();
            for received in self.rx{
                if let Event::Start = received {
                    state = state.next(received);
                    break;
                }
            }
            self.tx.send(state).unwrap();

            loop{
                //tries to see if a message is in the mailbox,
                //otherwise moves on immediately so the timer isn't slowed down
                //this is critical since keeping the time is important here
                let received = self.rx.try_recv().unwrap_or(Event::Tick);
                
                //THOUGHT: query the state to determine what to do to the thread
                state = state.next(received);


                match state {

                    State::Idle(t) => {
                        let current = thread::current();
                        self.tx.send(state).unwrap();
                        thread::park();
                    }

                    State::Running(t) => self.tx.send(state).unwrap(),

                    State::Error => break,

                    _ => (),
                }

                
            }

            self
        });

        Ok(t)
    }
}