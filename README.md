# keyboard_listener_windows
Simple library to listen **globally** to keyboard **ONLY** on Windows.

This library is subset of caret [rdev](https://github.com/Narsil/rdev).  
You should use crate [rdev](https://github.com/Narsil/rdev) if you don't just need to listen on Windows.

## Install
```shell
$ cargo add keyboard_listener_windows
```

## Listening to global events
Example:
```rust
use std::time::Duration;
use keyboard_listener_windows::{start_listen, stop_listen, Event};

fn main() {
    start_listen(callback);
    println!("start listen");

    // you can stop listen any time
    std::thread::spawn(||{
        std::thread::sleep(Duration::new(5,0));
        stop_listen();
        println!("stop listen")
    }).join().unwrap();
}

fn callback(event: Event) {
    println!("Keyboard event: {:?}", event);
}

```
Sample output:
```
start listen
Keyboard event: Event { timestamp: 1709032907009, is_key_down: true, key: "KeyA" }
Keyboard event: Event { timestamp: 1709032907036, is_key_down: true, key: "KeyS" }
Keyboard event: Event { timestamp: 1709032907140, is_key_down: true, key: "KeyD" }
Keyboard event: Event { timestamp: 1709032907216, is_key_down: false, key: "KeyA" }
Keyboard event: Event { timestamp: 1709032907373, is_key_down: false, key: "KeyS" }
Keyboard event: Event { timestamp: 1709032907456, is_key_down: false, key: "KeyD" }
stop listen
```
You can clone this repository then run this example using cargo:
```shell
$ cd example/listen && cargo run
```

OK, that's all, if you need more feature, please use crate [rdev](https://github.com/Narsil/rdev).
