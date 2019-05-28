//! A Rust logger. Provides and only provides macro `debugln!()`
//!
//! # Example
//! ```edition2018
//! fn main() {
//!     // Register the module name `main` to rog, so all debug logs under the main
//!     // module will be printed.
//!     rog::reg("main");
//!     rog::debugln!("Debug");
//!     rog::println!("Print");
//! }
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
