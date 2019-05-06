# rog

A Rust logger.

```toml
[dependencies]
rog = "0.1"
```

# Example

```rust
fn main() {
    rog::init(rog::Print, rog::LTIME);
    rog::debugln!("Debug");
    rog::println!("Hello");
}
```

This outputs:

```
2019-05-06 10:57:28 Hello
```

You can run the above example with:

```bash
cargo run --example main
```

# Licence

WTFPL
