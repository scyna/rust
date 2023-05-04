use crate::endpoint;
use nats;

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

    pub fn register_endpoint<T>(&self, url: &str, _handler: endpoint::Handler<T>) -> std::io::Result<()> {
        println!("Register Endpoint: {}", url);
        self.connection.queue_subscribe(url, "API")?.with_handler(|msg: nats::Message| {
            msg.ack()?;
            Ok(())
        });
        Ok(())
    }
}
