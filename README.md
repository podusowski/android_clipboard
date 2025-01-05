Access text in the Android clipboard from Rust using a simple API.

```rust
use android_clipboard::{get_text, set_text};

fn hello() {
    set_text("Hello, Android clipboard!".to_string());
    println!("{}", get_text().unwrap());
}
```
