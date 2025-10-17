use base64::prelude::*;
use serde::json::{Deserialize};

struct OpenJwt {

}

#[derive(Debug)]
struct JwtHeader {
    alg: String,
    typ: String,
}


struct JwtPayload {
    sub: String,
    name: String,
    admin: bool,
    iat: u32
}

pub fn create_jwt() -> () {
    ()
}


/*
* Take a token that is "[base64<json>].[base64<json>].[base64<json>]"
*/
pub fn open_jwt(token: &str) -> Option<OpenJwt> {
    let mut token_iter = token.split('.');

    let header_b64 = token_iter.next();
    let payload_b64 = token_iter.next();
    let signature_b64 = token_iter.next();

    let header = BASE64_STANDARD.decode(header_b64)?;

    dbg!(header);

    OpenJwt {

    }
}
