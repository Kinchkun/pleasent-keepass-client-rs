pub struct Context;
use lazy_static::lazy_static;

lazy_static! {
    static ref ctx: Context = { Context::new() };
}

impl Context {
    fn new() -> Self {
        pretty_env_logger::init_timed();
        Context {}
    }
}

pub fn setup() -> &'static Context {
    &ctx
}
