//! A Rust logger. Provides macro `debugln!()` and `println!()`.
//!
//! # Example
//! ```edition2021
//! rog::reg("main");
//! rog::debugln!("debug");
//! rog::println!("print");
//! ```

use std::collections::HashSet;
pub use std::println;

static mut C: Option<HashSet<&'static str>> = None;

/// Returns a hashset indicating which modules have been registered.
pub fn cfg() -> &'static mut HashSet<&'static str> {
    unsafe { C.get_or_insert(HashSet::new()) }
}

/// Debugs to the standard output, with a newline.
#[macro_export]
macro_rules! debugln {
    ($($arg:tt)*) => ({
        if rog::cfg().contains(module_path!()) {
            print!($($arg)*);
            print!("\n");
        }
    })
}

/// Register a series of module names, the logs in this module will be printed.
pub fn reg(module: &'static str) {
    let c = cfg();
    c.insert(module);
}
