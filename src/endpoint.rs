use protobuf::Message;
use std::sync::Arc;

use crate::{error::Error, proto, OK};

pub type Handler<T> = fn(ctx: &mut Context, request: &T) -> Error;

pub struct Context {
    flushed: bool,
    reply: String,
    connection: Arc<nats::Connection>,
}

impl Context {
    pub fn new(conn: Arc<nats::Connection>, reply: String) -> Context {
        Context { connection: conn, flushed: false, reply: reply }
    }
    pub fn info(&self, message: &str) {
        println!("{}", message)
    }

    pub fn response<T: protobuf::Message>(&mut self, message: T) {
        self.flush(200, message);
    }

    pub fn ok<T: protobuf::Message>(&mut self, message: T) -> Error {
        self.flush(200, message);
        OK
    }

    pub fn flush<T: protobuf::Message>(&mut self, status: i32, data: T) {
        let mut response = proto::Response::new();
        response.SessionID = 0;
        response.Code = status;
        let result = data.write_to_bytes();
        match result {
            Ok(body) => {
                response.Body = body;
                self.connection.publish(&self.reply, response.write_to_bytes().unwrap()).unwrap();
            }
            Err(err) => print!("Error in serialize response:{}", err),
        }
        self.flushed = true;
    }

    pub fn flushed(&self) -> bool {
        self.flushed
    }
}
