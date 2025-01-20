[![crates.io](https://img.shields.io/crates/v/android_clipboard.svg)](https://crates.io/crates/android_clipboard)
[![docs.rs](https://img.shields.io/docsrs/android_clipboard/latest)](https://docs.rs/android_clipboard/latest/)

Access text in the Android clipboard from Rust using a simple API.

```rust
use android_clipboard::{get_text, set_text};

fn hello() {
    set_text("Hello, Android clipboard!".to_string());
    println!("{}", get_text().unwrap());
}
```
