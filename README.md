closure-capture
===

Capture variables are moved into closure or async block

[![Latest version](https://img.shields.io/crates/v/closure-capture.svg)](https://crates.io/crates/closure-capture)
[![Documentation](https://docs.rs/closure-capture/badge.svg)](https://docs.rs/closure-capture)
![License](https://img.shields.io/crates/l/log.svg)


When using the move keyword, all external variables used in the closure will be moved into the closure.

Sometimes you may only need to move a few variables, and the rest of the variables will remain referenced.

At this time, you can use closure-capture to specify the variables to be captured.

### Usage

link `closure-capture`

cargo.toml
```toml
[dependencies]
closure-capture = "0.1"
```

Move variables a and b into the closure

```rust
fn main() {
    let a = 1;
    let b = 2;
    
    std::thread::spawn(closure_capture::closure!([a, b] () {
        println!("{}", a + b)
    }))
    .join()
    .unwrap();
}
```

Move variables a and b into the closure and modify a

```rust
fn main() {
    let a = 1;
    let b = 2;
    
    std::thread::spawn(closure_capture::closure!([mut a, b] () {
        a += b;
        println!("{}", a)
    }))
    .join()
    .unwrap();
}
```

With async block

```rust
#[tokio::main]
async fn main() {
    let a = 1;
    let b = 2;

    tokio::spawn(closure_capture::async_block!([mut a, b] async {
        a += b;
        println!("{}", a)
    }))
    .await
    .unwrap();
}
```