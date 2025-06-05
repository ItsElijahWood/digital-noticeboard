use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

use crate::jwt::validate_token::validate_token;
use crate::jwt::validate_token::validate_token_fetch_congregation;
use crate::jwt::validate_token::validate_token_img_fetch;
use crate::jwt::validate_token::validate_token_img_pass;

/// Manage the JWT requests
pub fn jwt_req(mut stream: &TcpStream, reader: &mut BufReader<&TcpStream>, req_status: &String) {
    if req_status.starts_with("POST /api/fetch_dashboard_congregation") {
        let mut json_line = String::new();
        let mut found_cookie = false;

        loop {
            json_line.clear();

            // Read tcpstream into string
            if reader.read_line(&mut json_line).unwrap_or(0) == 0 {
                break;
            }

            // If line = nothing break out of loop
            if json_line == "\r\n" {
                break;
            }

            // Trims the cookie reqest to get the value of the JWT token
            if json_line.to_lowercase().starts_with("cookie:") {
                if let Some((_, cookie_value)) = json_line.split_once(':') {
                    let cookie_string = cookie_value.trim().to_string();

                    let cookie_strings = cookie_string.split_once("=").unwrap();
                    let cookie = cookie_strings.1;

                    found_cookie = true;

                    validate_token_fetch_congregation(&stream, cookie);
                }
            }
        }

        if !found_cookie {
            let body = r#"{"error":"Expired Token"}"#;

            let response = format!(
                "HTTP/1.1 401 Unauthorized\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                body.len(),
                body
            );

            stream.write_all(response.as_bytes()).unwrap();
        }
    } else if req_status.starts_with("POST /api/add_img") {
        let mut json_line = String::new();
        let mut content_type = String::new();
        let mut content_length = 0;
        let mut found_cookie = false;

        loop {
            json_line.clear();

            // Read tcpstream into string
            if reader.read_line(&mut json_line).unwrap_or(0) == 0 {
                break;
            }

            let header_line = json_line.trim_end();

            // If line = nothing break out of loop
            if json_line == "\r\n" {
                break;
            }

            if let Some((key, value)) = header_line.split_once(':') {
                let key = key.trim().to_lowercase();
                let value = value.trim();

                match key.as_str() {
                    "content-type" => {
                        content_type = value.to_string();
                    }
                    "content-length" => {
                        content_length = value.parse::<usize>().unwrap_or(0);
                    }
                    _ => {}
                }
            }

            // Trims the cookie reqest to get the value of the JWT token
            if json_line.to_lowercase().starts_with("cookie:") {
                if let Some((_, cookie_value)) = json_line.split_once(':') {
                    let cookie_string = cookie_value.trim().to_string();

                    let cookie_strings = cookie_string.split_once("=").unwrap();
                    let cookie = cookie_strings.1;

                    found_cookie = true;

                    validate_token_img_pass(&stream, cookie, reader, &content_type, content_length);
                }
            }
        }

        if !found_cookie {
            let body = r#"{"error":"Expired Token"}"#;

            let response = format!(
                "HTTP/1.1 401 Unauthorized\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                body.len(),
                body
            );

            stream.write_all(response.as_bytes()).unwrap();
        }
    } else if req_status.starts_with("GET /api/protected_img_fetch") {
        let mut json_line = String::new();
        let mut found_cookie = false;

        loop {
            json_line.clear();

            // Read tcpstream into string
            if reader.read_line(&mut json_line).unwrap_or(0) == 0 {
                break;
            }

            // If line = nothing break out of loop
            if json_line == "\r\n" {
                break;
            }

            // Trims the cookie reqest to get the value of the JWT token
            if json_line.to_lowercase().starts_with("cookie:") {
                if let Some((_, cookie_value)) = json_line.split_once(':') {
                    let cookie_string = cookie_value.trim().to_string();

                    let cookie_strings = cookie_string.split_once("=").unwrap();
                    let cookie = cookie_strings.1;

                    found_cookie = true;

                    validate_token_img_fetch(&stream, cookie);
                }
            }
        }

        if !found_cookie {
            let body = r#"{"error":"Expired Token"}"#;

            let response = format!(
                "HTTP/1.1 401 Unauthorized\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                body.len(),
                body
            );

            stream.write_all(response.as_bytes()).unwrap();
        }
    } else {
        let mut json_line = String::new();
        let mut found_cookie = false;

        loop {
            json_line.clear();

            // Read tcpstream into string
            if reader.read_line(&mut json_line).unwrap_or(0) == 0 {
                break;
            }

            // If line = nothing break out of loop
            if json_line == "\r\n" {
                break;
            }

            // Trims the cookie reqest to get the value of the JWT token
            if json_line.to_lowercase().starts_with("cookie:") {
                if let Some((_, cookie_value)) = json_line.split_once(':') {
                    let cookie_string = cookie_value.trim().to_string();

                    let cookie_strings = cookie_string.split_once("=").unwrap();
                    let cookie = cookie_strings.1;

                    found_cookie = true;

                    validate_token(&stream, &cookie);
                }
            }
        }

        if !found_cookie {
            let body = r#"{"error":"Expired Token"}"#;

            let response = format!(
                "HTTP/1.1 401 Unauthorized\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                body.len(),
                body
            );

            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}
