mod error;
mod proto;
mod register_account;

fn main() -> std::io::Result<()> {
    let engine = scyna::Engine::init("http://127.0.0.1:8081", "scyna_test", "123456")?;
    engine.register_endpoint("/account/register", register_account::handler)?;
    engine.start();
    Ok(())
}
