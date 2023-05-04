use crate::proto;
use scyna::endpoint::*;

pub fn handler(ctx: Context, request: proto::RegisterAccountRequest) -> scyna::Error {
    ctx.info("Receive RegisterAccount request");
    scyna::OK
}
