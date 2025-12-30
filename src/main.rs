use gilrs::{Gilrs, Event};

fn main() {
    println!("Running fizzle ...");

    let mut gilrs = Gilrs::new().expect("Failed to initialize gilrs");

    println!("Connected gamepads:");
    for (id, gamepad) in gilrs.gamepads() {
        println!("{}: {}", id, gamepad.name());
    }

    loop {
        while let Some(Event { id, event, time , ..}) = gilrs.next_event() {
            println!("{:?} at {:?}: {:?}", id, time, event);
        }
    }
}
