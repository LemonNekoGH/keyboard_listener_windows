use keyboard_listener_windows::{listen, Event};

fn main() {
    // This will block.
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}

fn callback(event: Event) {
    println!("Keyboard event: {:?}", event);
}
