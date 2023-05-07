use crate::{endpoint, error, path, proto, utils, OK};
use core::time;
use nats;
use protobuf::Message;
use reqwest::{Result, StatusCode};
use std::{io, str, sync::Arc, thread};

pub struct Engine {
    connection: Arc<nats::Connection>,
}

impl Engine {
    pub fn init(url: &str, module: &str, secret: &str) -> io::Result<Engine> {
        println!("Creating Engine from {url}");

        match Engine::create_session(url, module, secret) {
            Ok(response) => {
                println!("SessionID={}", response.SessionID);

                let nc = nats::connect("nats://127.0.0.1:4222")?;
                Ok(Engine { connection: Arc::new(nc) })
            }
            Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
        }
    }

    fn create_session(url: &str, module: &str, secret: &str) -> io::Result<proto::CreateSessionResponse> {
        let client = reqwest::blocking::Client::new();
        let uri = url.to_string() + path::SESSION_CREATE_URL;
        let mut request = proto::CreateSessionRequest::new();
        request.Module = module.into();
        request.Secret = secret.into();
        let response = client.post(uri).body(request.write_to_bytes().unwrap()).timeout(time::Duration::from_secs(10)).send().unwrap();
        match response.status() {
            StatusCode::OK => {
                let data = response.bytes().unwrap();
                match proto::CreateSessionResponse::parse_from_bytes(data.as_ref()) {
                    Ok(body) => Ok(body),
                    Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
                }
            }
            _ => return Err(io::Error::new(io::ErrorKind::Other, "Error in create session")),
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

        let connection = self.connection.clone();
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
}
