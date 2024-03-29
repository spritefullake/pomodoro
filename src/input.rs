// An abstract, platform-agnostic, pomodoro-input module
use super::{
    mailbox,
    timer::Timer,
    timer_state::{Event, State},
    pomodoro::Pomodoro,
    controllable::Controllable,
    fsm::{Stateful, Trigger},
    task::Task,
};
use std::{
    thread,
    error::Error,
    time::Duration,
};

///Begin should implement the minimum necessary connection between the pomodoros
/// and the timer.
pub fn begin(t: Timer, p: &mut Pomodoro) -> Result<mailbox::Mailbox<Event, State>,Box<dyn Error>>{

    //Timer and main loop mailbox setup
    let (controller_mail, timer_mail) = mailbox::new_pair::<Event, State>();
    let handle = timer_mail.activate(t)?;
    controller_mail.start(handle.thread())?;
    //Complete a task everytime the timer for tasks ends. Start the break timer when the pomodoro is on break.
    for received in &controller_mail.rx{
        if let State::Idle(_) = received{
            p.complete_next();
            controller_mail.start(handle.thread())?;
        }
    }

    Ok(controller_mail)
}

pub fn sync_pomodoro_with_state(s: &State, p: &mut Pomodoro)
{
    //Complete a task everytime the timer for tasks ends. Start the break timer when the pomodoro is on break.
    if let State::Idle(_) = s{
        p.complete_next();
    }

} 

type TimerState = Result<State, Box<dyn Error>>;
type TimerEvent = Result<Event, Box<dyn Error>>;

impl mailbox::Mailbox<Event, State> {
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
    pub fn react<'a>(&self, t: &thread::Thread, s: &'a State) -> Result<&'a Timer, TimerState>{
        match s {
            State::Idle(timer) => {
                self.start(t).unwrap();
                Ok(timer)
            },
            State::Running(timer) => Ok(timer),
        }
    }
}
impl Controllable for mailbox::Mailbox<State, Event> {
    type Data = Timer;
    ///Starts the timer thread 
    fn activate(self, d: Self::Data) 
    -> Result<thread::JoinHandle<Self>, Box<dyn Error>>
    {
        let mut state = State::init(d);
        let t = thread::spawn(move ||{
            thread::park();
            'event_response: loop{
                let event = &self.rx.try_recv().unwrap_or(Event::Tick);
                state = state.next(event).unwrap();
                self.tx.send(state.clone()).unwrap();

                //special actions not applied to the state or data that are important for behavior:
                //For example, pausing the thread on a stop event
                //push errors into typesystem with result and only match to events!
                match event {
                    Event::Stop => thread::park(),
                    _ => (),
                }
            }

            
        });

        Ok(t)
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    #[ignore]
    fn complete_task_on_timer_end(){
        let n = 1;
        let t = Timer::new(Duration::from_secs(n));
        //Timer and main loop mailbox setup
        let (controller_mail, timer_mail) = mailbox::new_pair::<Event, State>();
        let handle = timer_mail.activate(t).unwrap();
        controller_mail.start(handle.thread()).expect("Starting Error");

        //pomodoro setup
        let tasks = std::collections::VecDeque::from(vec![
            Task::new(String::from("task1")),
            Task::new(String::from("task2"))
        ]);
        let p = Pomodoro::new(tasks);

        for received in controller_mail.rx {
            if let State::Idle(t) = received {
                if !(t.duration.as_secs() == 0) {
                    panic!();
                }
            }
        }
    }
}