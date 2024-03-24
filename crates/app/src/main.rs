use app_core::Plugin;
use std::{error::Error, thread::sleep, time::Duration};

// cargo run --bin app -- libhello_world.dylib
fn main() -> Result<(), Box<dyn Error>> {
    let plugin_name = std::env::args()
        .nth(1)
        .expect("Provide the library name as an argument (e.g libhello_world.dylib)");

    loop {
        let plugin_lib = unsafe { libloading::Library::new(&plugin_name) }?;
        let plugin_loader: libloading::Symbol<fn() -> Box<dyn Plugin>> =
            unsafe { plugin_lib.get(b"plugin") }?;
        let mut plugin = plugin_loader();

        plugin.init()?;
        plugin.update()?;
        plugin.update()?;
        plugin.deinit()?;

        sleep(Duration::from_secs(1));
        println!("Closing library and reloading");
        plugin_lib.close()?;
    }
}
