use app_core::{Context, Plugin};
use std::{error::Error, thread::sleep, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    let plugin_name = std::env::args()
        .nth(1)
        .expect("Provide the library name as an argument (e.g libhello_world.dylib)");
    let mut context = Context { invocations: 0 };

    loop {
        // Be careful about explicitly calling Library::close, as it might deinitialize
        // libstd funcions such as `Box::drop`. Drop order is important. and should be
        // taken into consideration.
        let plugin_lib = unsafe { libloading::Library::new(&plugin_name) }?;
        let plugin_loader: libloading::Symbol<fn() -> Box<dyn Plugin>> =
            unsafe { plugin_lib.get(b"plugin") }?;
        let mut plugin = plugin_loader();
        plugin.hook(&mut context)?;
        context.invocations += 1;
        sleep(Duration::from_secs(1));
        println!("Reloading...")
    }
}
