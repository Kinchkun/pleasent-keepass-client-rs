pub struct Context;

impl Context {
    fn new() -> Self {
        pretty_env_logger::init_timed();
        Context {}
    }
}

pub fn setup() -> Context {
    Context::new()
}
