//The guaranteed api for state machine events and states

/* Every event for a state machine can trigger an action implemented
through the Trigger trait

is the naming confusing? potentially*/

///Executes an event that modifies some data
pub trait Trigger{
    //an associated type lets the implementer choose the type rather than separately implement for each type
    type Data;
    ///Consumes the data and returns an updated version
    /// 
    /// There is no purpose in keeping old data once the event triggers
    fn trigger(&self, d: Self::Data) -> Self::Data;
}

/* the state machine only transitions to the next state;
whatever action is intended by the event is not
the responsibility of the state machine 

A concrete <T: Trigger> is needed rather than using impl Trigger
since <T: Trigger> represents a concrete type*/
pub trait Stateful<T: Trigger>
where Self : Sized
{
    ///Input an event into the finite state machine.
    fn next(self, event: &T) -> Option<Self>;
}

pub trait Responsive<T: Trigger>{
    fn respond(&mut self, event: T);
}