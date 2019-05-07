use std::collections::HashSet;
pub use std::println;

static mut C: Option<HashSet<&'static str>> = None;

/// Returns a hashset indicating which modules have been registered.
pub fn cfg() -> &'static mut HashSet<&'static str> {
    unsafe { C.get_or_insert(HashSet::new()) }
}

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
pub fn reg(module: Vec<&'static str>) {
    let c = cfg();
    for e in &module {
        c.insert(e);
    }
}
