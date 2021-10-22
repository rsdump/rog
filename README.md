# rog

A Rust logger. Provides macro `debugln!()` and `println!()`.

```toml
[dependencies]
rog = "0.1"
```

I am fed up with the days of downloading xx packages in order to print a line of logs. No third-party dependencies, no extra features, make rog simple and pure.

# Example

```rust
use rog::{self, debugln};

fn main() {
    // Register the module name `main` to rog, so all debug logs under the main
    // module will be printed.
    rog::reg("main");
    debugln!("debug");
    println!("print");
}
```

You can run the above example with:

```bash
$ cargo run --example main
```

# Licence

MIT
