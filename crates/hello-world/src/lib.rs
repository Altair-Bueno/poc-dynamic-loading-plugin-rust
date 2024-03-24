use app_core::Plugin;

#[derive(Debug)]
struct HelloWorld;

impl Plugin for HelloWorld {
    fn hook(&mut self, _: &mut app_core::Context) -> Result<(), app_core::Error> {
        println!("Hello world");
        Ok(())
    }
}

#[no_mangle]
pub fn plugin() -> Box<dyn Plugin> {
    Box::new(HelloWorld)
}
