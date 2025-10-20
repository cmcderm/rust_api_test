use std::time::{SystemTime, UNIX_EPOCH};
use hmac::{ Hmac, Mac };
use sha2::Sha256;
use base64::prelude::*;
use serde::{ Deserialize, Serialize };
use serde_json::json::Json;

use crate::auth::User;

type HmacSha256 = Hmac<Sha256>;

pub struct OpenedJwt {
    header: JwtHeader,
    payload: JwtPayload,
}

#[derive(Debug, Deserialize, Serialize)]
struct JwtHeader {
    alg: String,
    typ: String,
}


#[derive(Debug, Deserialize, Serialize)]
struct JwtPayload {
    sub: String,
    name: String,
    admin: bool,
    iat: u64
}

pub fn create_jwt(user: User) -> String {
    let header = JwtHeader {
        alg: String::from("HS256A"),
        typ: String::from("JWT"),
    };

    let header_str = serde_json::to_string(&header).expect("Bad header somehow");

    let payload = JwtPayload {
        sub: user.user_id,
        name: user.username,
        admin: false,
        iat: SystemTime::now().duration_since(UNIX_EPOCH).expect("System time is before epoch!").as_secs(),
    };

    let payload_str = serde_json::to_string(&payload).expect("Bad payload");

    let header_payload = format!("{}.{}", header_str, payload_str);

    let mut mac = HmacSha256::new_from_slice("secretsecretsecret".as_bytes())
        .expect("This shouldn't happen");

    mac.update(header_payload.as_bytes());
    let signature = BASE64_URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes().to_vec());

    return format!("{}.{}", header_payload, signature);
}


/*
* Take a token that is "[base64<json>].[base64<json>].[base64<json>]"
*/
pub fn open_jwt(token: &str) -> Option<OpenedJwt> {
    let mut token_iter = token.split('.');

    let header_b64 = token_iter.next()?;
    let payload_b64 = token_iter.next()?;
    let signature_b64 = token_iter.next()?;
    drop(token_iter);

    let header_str = str::from_utf8(BASE64_URL_SAFE_NO_PAD.decode(&header_b64).ok()?.as_slice()).ok()?;
    let payload_str = str::from_utf8(BASE64_URL_SAFE_NO_PAD.decode(payload_b64).ok()?.as_slice()).ok()?;

    dbg!(header_str);
    dbg!(payload_str);

    let header: JwtHeader = serde_json::from_str(header_str).ok()?;
    let payload: JwtPayload = serde_json::from_str(payload_str).ok()?;

    Some(OpenedJwt { header, payload })
}

