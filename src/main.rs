//#![cfg(windows)]
// Let's put this so that it won't open the console
//#![windows_subsystem = "windows"]

//-----------------------------------------------------------------------------

const PROP_FILE_NAME: &str = "deadjustice.prop";
const BUILD: &str = env!("CARGO_PKG_VERSION_PRE");

//-----------------------------------------------------------------------------

fn run() {
    if cfg!(debug_assertions) {
        println!("Dead Justice DEBUG Build {BUILD}");
    } else {
        println!("Dead Justice Build Release {BUILD}");
    }
}

//-----------------------------------------------------------------------------

fn main() {
    run();
}
