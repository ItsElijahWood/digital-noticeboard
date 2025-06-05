use dotenv::dotenv;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, errors::ErrorKind};
use std::{env, io::{BufReader, Write}, net::TcpStream};

use crate::add_img::add_imgs;
use crate::fetch_imgs::fetch_pngs;
use crate::fetch_congregation::fetch_congregation;

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

pub fn validate_token_img_pass(mut stream: &TcpStream, token: &str, reader: &mut BufReader<&TcpStream>, content_type: &str, content_length: usize) {
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
            
            add_imgs(&stream, reader, &user_id, &content_type, content_length);

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
            fetch_pngs(&stream, &user_id);

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

pub fn validate_token_fetch_congregation(mut stream: &TcpStream, token: &str) {
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
            fetch_congregation(&stream, &user_id);

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
