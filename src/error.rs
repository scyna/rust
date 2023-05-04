use crate::proto;

pub struct Error {
    code: i32,
    message: &'static str,
}

pub const OK: Error = Error::new(0, "OK");
pub const REQUEST_INVALID: Error = Error::new(20, "Request Invalid");

impl Error {
    pub const fn new(code: i32, message: &'static str) -> Error {
        Error { code, message }
    }

    pub fn to_proto(&self) -> proto::Error {
        let mut ret = proto::Error::new();
        ret.code = self.code;
        ret.message = String::from(self.message);
        return ret;
    }

    pub fn code(&self) -> i32 {
        self.code
    }

    pub fn message(&self) -> String {
        self.message.to_string()
    }
}
