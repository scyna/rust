use crate::error::Error;

pub type Handler<T> = fn(ctx: Context, request: T) -> Error;

pub struct Context {}

impl Context {
    pub fn info(&self, message: &str) {
        println!("{}", message)
    }
}
