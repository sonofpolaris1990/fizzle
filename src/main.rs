use gilrs::{Gilrs, Event};

fn main() {
    println!("Running fizzle ...");

    let mut gilrs = Gilrs::new().expect("Failed to initialize gilrs");

    loop {
        while let Some(Event { id, event, time , ..}) = gilrs.next_event() {
            println!("{:?} at {:?}: {:?}", id, time, event);
            // TODO: Map and handle Joystick movement ...
        }
    }
}
