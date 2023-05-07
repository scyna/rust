use crate::Engine;
use crate::{path, proto};
use core::time;
use protobuf::Message;
use reqwest::StatusCode;
use std::io;

impl Engine {
    pub fn next_sequence(&self) -> u64 {
        let mut id = self.session_id.lock().unwrap();
        *id += 1;
        *id
    }

    pub(crate) fn create_session(url: &str, module: &str, secret: &str) -> io::Result<proto::CreateSessionResponse> {
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
}
