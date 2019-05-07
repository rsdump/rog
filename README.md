# rog

A Rust logger. Provides and only provides macro `debugln!()`

```toml
[dependencies]
rog = "0.1"
```

# Example

```rust
use rog::{self, debugln};

fn main() {
    rog::open("main");
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
