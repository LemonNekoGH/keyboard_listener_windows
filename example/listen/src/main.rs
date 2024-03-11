use keyboard_listener_windows::{start_listen, stop_listen, Event};
use std::time::Duration;

fn main() {
    start_listen(callback);
    println!("start listen");

    // you can stop listen any time
    std::thread::spawn(|| {
        std::thread::sleep(Duration::new(5, 0));
        stop_listen();
        println!("stop listen")
    })
    .join()
    .unwrap();
}

fn callback(event: Event) {
    println!("Keyboard event: {:?}", event);
}
