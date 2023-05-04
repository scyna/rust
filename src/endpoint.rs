use crate::{error::Error, OK};

pub type Handler<T> = fn(ctx: &mut Context, request: &T) -> Error;

pub struct Context {
    flushed: bool,
}

impl Context {
    pub fn new() -> Context {
        Context { flushed: false }
    }
    pub fn info(&self, message: &str) {
        println!("{}", message)
    }

    pub fn response<T>(&mut self, message: T)
    where
        T: protobuf::Message,
    {
        self.flush(200, message);
    }

    pub fn ok<T>(&mut self, message: T) -> Error
    where
        T: protobuf::Message,
    {
        self.flush(200, message);
        OK
    }

    pub fn flush<T>(&mut self, status: i32, data: T)
    where
        T: protobuf::Message,
    {
        //todo!()
        self.flushed = true;
    }

    pub fn flushed(&self) -> bool {
        self.flushed
    }
}
