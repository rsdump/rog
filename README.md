# rog

A Rust logger. Provides and only provides macro `debugln!()`

```toml
[dependencies]
rog = "0.1"
```

Tested in the following environments:

- Rust 1.34.0

I am fed up with the days of downloading xx packages in order to print a line of logs. No third-party dependencies, no extra features, make rog simple and pure.

# Example

```rust
use rog::{self, debugln};

fn main() {
    // Register the module name `main` to rog, so all debug logs under the main
    // module will be printed.
    rog::reg("main");
    debugln!("Debug");
    println!("Print");
}
```

You can run the above example with:

```bash
cargo run --example main
```

# Licence

WTFPL
