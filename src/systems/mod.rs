
/// 
/// Systems are a set of event handlers and optionally some associated state.
///
/// The event handlers are invoked with a list of event data and have access to component
/// lists. This access can be immutable or mutable which is made explicit so that
/// the event handler scheduler might parallelize handler invocations.
///
/// To properly enforce this I guess the scheduler would acquire the locks on the component
/// lists.
///
/// So the context of the handler function would consist of:
///
///   The handler state
///   The event data list
///   Immutable components lists
///   Mutable components lists
/// 
/// For example the following invocation:
///
/// system! my_system {
///	  on!(event_name, { positions: Position}, { descriptions: Description }) {
///     let position = positions[0];
///     descriptions[0].set(format!("Position = {},{}", position.x, position.y));
///   } 
/// }
///
/// Expands to:
///
/// pub mod my_system {
///   struct State {}
///   
///   pub fn event_name(state: &mut State, data: &Vec<event_name::Data>, positions: &Vec<Position>, descriptions: &mut Vec<Description> ) {
///     let position = positions[0];
///     descriptions[0].set(format!("Position = {},{}", position.x, position.y));
///   }
///
/// }