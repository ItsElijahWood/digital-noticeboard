use dotenv::dotenv;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, errors::ErrorKind};
use std::{env, io::Write, net::TcpStream};

use crate::add_img::get_id;
use crate::fetch_imgs::get_id_;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn validate_token(mut stream: &TcpStream, token: &str) {
    dotenv().ok();

    let jwt_token = env::var("JWT_AUTHENTICATION_TOKEN").unwrap();

    let valid = Validation::new(Algorithm::HS256);

    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_token.as_bytes()),
        &valid,
    ) {
        Ok(_token_data) => {
            let json_req = r#"{"ok": "Token accepted."}"#;
            let length = json_req.len();
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {length}\r\n\r\n{json_req}"
            );

            stream.write_all(response.as_bytes()).unwrap();
        }
        Err(e) => match e.kind() {
            ErrorKind::ExpiredSignature => {
                let body = r#"{"error":"Expired Token"}"#;

                let response = format!(
                    "HTTP/1.1 401 Unauthorized\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                    body.len(),
                    body
                );

                stream.write_all(response.as_bytes()).unwrap();
            }
            _ => {
                let body = r#"{"error":"Invalid Token"}"#;

                let response = format!(
                    "HTTP/1.1 401 Unauthorized\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                    body.len(),
                    body
                );

                stream.write_all(response.as_bytes()).unwrap();
            }
        },
    }
}

pub fn validate_token_img_pass(mut stream: &TcpStream, token: &str) {
    dotenv().ok();

    let jwt_token = env::var("JWT_AUTHENTICATION_TOKEN").unwrap();

    let valid = Validation::new(Algorithm::HS256);

    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_token.as_bytes()),
        &valid,
    ) {
        Ok(token_data) => {
            let json_req = r#"{"ok": "Token accepted."}"#;
            let length = json_req.len();
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {length}\r\n\r\n{json_req}"
            );

            let user_id = token_data.claims.sub.clone();
            get_id(&user_id);

            stream.write_all(response.as_bytes()).unwrap();
        }
        Err(e) => match e.kind() {
            ErrorKind::ExpiredSignature => {
                let body = r#"{"error":"Expired Token"}"#;

                let response = format!(
                    "HTTP/1.1 401 Unauthorized\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                    body.len(),
                    body
                );

                stream.write_all(response.as_bytes()).unwrap();
            }
            _ => {
                let body = r#"{"error":"Invalid Token"}"#;

                let response = format!(
                    "HTTP/1.1 401 Unauthorized\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                    body.len(),
                    body
                );

                stream.write_all(response.as_bytes()).unwrap();
            }
        },
    }
}

pub fn validate_token_img_fetch(mut stream: &TcpStream, token: &str) {
    dotenv().ok();

    let jwt_token = env::var("JWT_AUTHENTICATION_TOKEN").unwrap();

    let valid = Validation::new(Algorithm::HS256);

    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_token.as_bytes()),
        &valid,
    ) {
        Ok(token_data) => {
            let json_req = r#"{"ok": "Token accepted."}"#;
            let length = json_req.len();
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {length}\r\n\r\n{json_req}"
            );

            let user_id = token_data.claims.sub.clone();
            get_id_(&user_id);

            stream.write_all(response.as_bytes()).unwrap();
        }
        Err(e) => match e.kind() {
            ErrorKind::ExpiredSignature => {
                let body = r#"{"error":"Expired Token"}"#;

                let response = format!(
                    "HTTP/1.1 401 Unauthorized\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                    body.len(),
                    body
                );

                stream.write_all(response.as_bytes()).unwrap();
            }
            _ => {
                let body = r#"{"error":"Invalid Token"}"#;

                let response = format!(
                    "HTTP/1.1 401 Unauthorized\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                    body.len(),
                    body
                );

                stream.write_all(response.as_bytes()).unwrap();
            }
        },
    }
}
