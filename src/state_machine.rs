/* The finite state machine for the timer */

enum Event{
    Start,
    Stop,
    Tick,
}

enum State{
    Idle,
    Running,
    Error,
}

impl State {
    fn next(self, e: Event) -> Self {
        match self {
            State::Idle => match e {
                Event::Start => State::Running,
                _ => State::Error,
            },
            State::Running => match e {
                Event::Stop => State::Idle,
                Event::Tick => State::Running,
                _ => State::Error,
            },
            _ => self
        }
    }
}