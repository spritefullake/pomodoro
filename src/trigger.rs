/* Every event for a state machine can trigger an action implemented
through the Trigger trait*/

///Executes an event
trait Trigger{
    fn trigger(self);
}