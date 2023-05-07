use crate::{endpoint, error, proto, utils};
use core::time;
use nats;
use protobuf::Message;

use std::{
    io, str,
    sync::{Arc, Mutex},
    thread,
};

pub struct Engine {
    module: &'static str,
    session_id: Mutex<u64>,
    connection: Arc<nats::Connection>,
}

impl Engine {
    pub fn init(url: &str, module: &'static str, secret: &str) -> io::Result<Arc<Engine>> {
        println!("Creating Engine from {url}");

        match Engine::create_session(url, module, secret) {
            Ok(response) => {
                println!("SessionID={}", response.SessionID);

                let nc = nats::connect("nats://127.0.0.1:4222")?;
                Ok(Arc::new(Engine {
                    module,
                    session_id: Mutex::new(response.SessionID),
                    connection: Arc::new(nc),
                }))
            }
            Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
        }
    }

    pub fn start(&self) {
        /* FIXME: replace with sigint/sigterm */
        let ten_millis = time::Duration::from_millis(100);
        loop {
            thread::sleep(ten_millis);
        }
    }

    pub fn register_endpoint<T: protobuf::MessageFull>(&self, url: &str, handler: endpoint::Handler<T>) -> io::Result<()> {
        println!("Register Endpoint: {}", url);

        let connection = self.connection();
        connection.queue_subscribe(utils::subscribe_url(url).as_str(), "API")?.with_handler(move |msg| {
            let request = proto::Request::parse_from_bytes(msg.data.as_slice())?;
            let message: T;

            if request.JSON {
                let result = protobuf_json_mapping::parse_from_str::<T>(str::from_utf8(request.Body.as_slice()).unwrap());
                match result {
                    Ok(m) => message = m,
                    Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
                }
            } else {
                message = T::parse_from_bytes(request.Body.as_slice())?;
            }

            let mut ctx = endpoint::Context::new(connection.clone(), msg.reply.unwrap());
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

    pub fn module(&self) -> &str {
        self.module
    }

    pub fn connection(&self) -> Arc<nats::Connection> {
        self.connection.clone()
    }
}

mod id;
mod session;
