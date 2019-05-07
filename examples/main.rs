use rog::{self, debugln};

fn main() {
    // Register the module name `main` to rog, so all debug logs under the main
    // module will be printed.
    rog::reg(vec!["main"]);
    debugln!("Debug");
    println!("Print");
}
