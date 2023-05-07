use crate::error::*;
use crate::proto;
use scyna::*;

pub fn handler(ctx: &mut endpoint::Context, request: &proto::RegisterAccountRequest) -> Error {
    ctx.info("Receive RegisterAccount request");

    if request.email.len() == 0 {
        return scyna::REQUEST_INVALID;
    }

    if request.email != "a@gmail.com" {
        return ACCOUNT_NOT_FOUND;
    }

    let mut response = proto::RegisterAccountResponse::new();
    response.ID = 0;

    ctx.ok(response)
}
