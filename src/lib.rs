use std::collections::HashSet;
pub use std::println;

static mut R: Option<HashSet<&'static str>> = None;

pub fn rule() -> &'static mut HashSet<&'static str> {
    unsafe { R.get_or_insert(HashSet::new()) }
}

#[macro_export]
macro_rules! debugln {
    ($($arg:tt)*) => ({
        if rog::rule().contains(module_path!()) {
            print!($($arg)*);
            print!("\n");
        }
    })
}

pub fn open(module: &'static str) {
    rule().insert(module);
}
