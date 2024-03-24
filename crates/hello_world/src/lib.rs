use app_core::Plugin;

#[derive(Debug)]
struct HelloWorld {}

impl Plugin for HelloWorld {
    fn init(&mut self) -> Result<(), app_core::Error> {
        println!("Plugin loaded");
        Ok(())
    }

    fn update(&mut self) -> Result<(), app_core::Error> {
        println!("Received an update");
        // println!("Extra update");
        Ok(())
    }

    fn deinit(&mut self) -> Result<(), app_core::Error> {
        println!("Plugin unloaded");
        Ok(())
    }
}

#[no_mangle]
pub fn plugin() -> Box<dyn Plugin> {
    Box::new(HelloWorld {})
}
