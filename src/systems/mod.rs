
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
///   state! { i: i64 }
///   on!(event_name, { positions: Position}, { descriptions: Description }) {
///     state.i += 1;
///     let position = positions[0];
///     descriptions[0].set(format!("Position = {},{}", position.x, position.y));
///   } 
/// }
///
/// Expands to:
///
/// pub mod my_system {
///   struct State { i: i64 }
///   
///   pub fn event_name(state: &mut State, data: &Vec<event_name::Data>, positions: &Vec<Position>, descriptions: &mut Vec<Description> ) {
///     let position = positions[0];
///     descriptions[0].set(format!("Position = {},{}", position.x, position.y));
///   }
///
///   pub fn register() {
///      events::register_handler(event_name); // etc..
///   }
/// }
///
/// How are we going to register this handler at the event loop?
///
/// Ideally we would have just one 'register' call per system. But to have that the register call should enable
/// all nested handlers. We could also just have one handler per system. That would make things easier..
/// Maybe there's some scheme using recursive macros that we could use to generate a register_system that collects
/// all on! calls. Ok this is definitely a possibility. We could simply match the on! macro in the system! macro
/// and use that to populate the register function.
///
/// 