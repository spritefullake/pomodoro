/* Every event for a state machine can trigger an action implemented
through the Trigger trait*/

///Executes an event that modifies some data (the state)
pub trait Trigger<Data>{
    ///Consumes the data and returns an updated version
    fn trigger(&self, d: Data) -> Data;
    ///Modifies the data in-place
    fn trigger_mut(&self, d: &mut Data) -> (){
        return;
    }
}
