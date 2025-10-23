use std::{io::Write, time::{SystemTime, UNIX_EPOCH}};
use hmac::{ Hmac, Mac };
use sha2::Sha256;
use base64::prelude::*;
use rocket::serde::{ Deserialize, Serialize, json::Json };
use serde_json;

use crate::auth::User;

type HmacSha256 = Hmac<Sha256>;

pub struct OpenedJwt {
    pub header: JwtHeader,
    pub payload: JwtPayload,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JwtHeader {
    alg: String,
    typ: String,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct JwtPayload {
    pub sub: String,
    pub name: String,
    pub admin: bool,
    pub iat: u64
}

fn utf8_to_b64(input: &str) -> String {
    BASE64_URL_SAFE_NO_PAD.encode(input.as_bytes())
}

fn b64_to_utf8(input: &str) -> Option<String> {
    let bytes = BASE64_URL_SAFE_NO_PAD.decode(input).ok()?;
    Some(str::from_utf8(&bytes).ok()?.to_string())
}

pub fn create_jwt(user: User) -> String {
    let header = JwtHeader {
        alg: String::from("HS256"),
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

    let header_b64 = utf8_to_b64(&header_str);
    let payload_b64 = utf8_to_b64(&payload_str);

    let header_payload = format!("{}.{}", header_b64, payload_b64);

    let signature = generate_signature(&header_payload, "secretsecretsecret");

    return format!("{}.{}", header_payload, signature);
}


/*
* Take a token that is "[base64<json>].[base64<json>].[base64<json>]"
* Should be a Result but that means putting together an Error enum
*/
pub fn open_jwt(token: &str) -> Option<OpenedJwt> {
    let mut token_iter = token.split('.');

    let header_b64 = token_iter.next()?;
    let payload_b64 = token_iter.next()?;
    let signature = token_iter.next()?;
    drop(token_iter);

    let sig = generate_signature(&format!("{}.{}", header_b64, payload_b64), "secretsecretsecret");

    if signature != sig {
        println!("Signature doesn't match! Received: {} Computed: {}", signature, sig);
        return None
    }

    let header_bytes = BASE64_URL_SAFE_NO_PAD.decode(&header_b64).ok()?;
    let header_str = str::from_utf8(header_bytes.as_slice()).ok()?;

    let payload_bytes = BASE64_URL_SAFE_NO_PAD.decode(&payload_b64).ok()?;
    let payload_str = str::from_utf8(payload_bytes.as_slice()).ok()?;

    let header: JwtHeader = serde_json::from_str(header_str).ok()?;
    let payload: JwtPayload = serde_json::from_str(payload_str).ok()?;

    Some(OpenedJwt { header, payload })
}

fn generate_signature(content: &str, secret: &str) -> String {
    println!("Content: {} Secret: {}", content, secret);
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .expect("This shouldn't happen");

    mac.update(content.as_bytes());

    BASE64_URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes().to_vec())
}

