fn main() {
    // Register the module name `main` to rog, so all debug logs under the main
    // module will be printed.
    rog::reg("main");
    rog::debugln!("Debug");
    rog::println!("Print");
}
