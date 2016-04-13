extern crate entity_rust;

use entity_rust::events;

#[test]
fn event_queue_accessors() {
    let i : i64 = 5;
    events::set_event_queue("my_queue", i);
    match events::get_event_queue_mut("my_queue") {
        Some(queue) => {
            assert!(queue.len() == 1);
            assert!(i == queue[0])
        }
        None => assert!(false)
    };
}
