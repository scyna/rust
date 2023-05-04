use crate::{endpoint, error, proto};
use nats;
use protobuf::Message;
use std::str;

pub struct Engine {
    connection: nats::Connection,
}

impl Engine {
    pub fn new(url: &str, _secret: &str) -> std::io::Result<Engine> {
        println!("Creating Engine from {url}");
        let nc = nats::connect("demo.nats.io")?;
        Ok(Engine { connection: nc })
    }

    pub fn start(&self) {
        //todo!()
    }

    pub fn stop(&self) {
        //todo!()
    }

    pub fn register_endpoint<T>(&self, url: &str, handler: endpoint::Handler<T>) -> std::io::Result<()>
    where
        T: protobuf::MessageFull,
    {
        println!("Register Endpoint: {}", url);
        self.connection.queue_subscribe(url, "API")?.with_handler(move |msg| {
            let request = proto::Request::parse_from_bytes(msg.data.as_slice()).unwrap();
            let message: T;

            if request.JSON {
                message = protobuf_json_mapping::parse_from_str(str::from_utf8(request.Body.as_slice()).unwrap()).unwrap();
            } else {
                message = T::parse_from_bytes(request.Body.as_slice()).unwrap();
            }

            let mut ctx = endpoint::Context::new();
            let err = handler(&mut ctx, &message);

            if err.code() == error::OK.code() {
                if !ctx.flushed() {
                    ctx.flush(200, error::OK.to_proto())
                }
            } else {
                ctx.flush(400, err.to_proto())
            }
            Ok(())
        });
        Ok(())
    }
}
