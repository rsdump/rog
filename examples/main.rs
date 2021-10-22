fn main() {
    // Register the module name `main` to rog, so all debug logs under the main module could be printed.
    rog::reg("main");
    rog::debugln!("debug");
    rog::println!("print");
}
