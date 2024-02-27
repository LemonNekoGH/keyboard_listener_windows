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
```
Sample output:
```
Keyboard event: Event { timestamp: 1709032907009, is_key_down: true, key: "KeyA" }
Keyboard event: Event { timestamp: 1709032907036, is_key_down: true, key: "KeyS" }
Keyboard event: Event { timestamp: 1709032907140, is_key_down: true, key: "KeyD" }
Keyboard event: Event { timestamp: 1709032907216, is_key_down: false, key: "KeyA" }
Keyboard event: Event { timestamp: 1709032907373, is_key_down: false, key: "KeyS" }
Keyboard event: Event { timestamp: 1709032907456, is_key_down: false, key: "KeyD" }
```
You can clone this repository then run this example using cargo:
```shell
$ cd example/listen && cargo run
```

OK, that's all, if you need more feature, please use crate [rdev](https://github.com/Narsil/rdev).
