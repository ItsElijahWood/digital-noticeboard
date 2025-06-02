use ::rusqlite::params;
use bcrypt::verify;
use dotenv::dotenv;
use jsonwebtoken::{EncodingKey, Header, encode};
use std::{
    env,
    io::{BufRead, BufReader, Read, Write},
    net::TcpStream,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::database::conn::conn_to_database;

/// Manages the POST req's
pub fn manage_posts(
    req_status: &String,
    reader: &mut BufReader<&TcpStream>,
    mut stream: &TcpStream,
) {
    dotenv().ok();

    // Format POST req to just /api/login ect
    let space: Vec<&str> = req_status.split_whitespace().collect();
    let mut path = String::from("/");
    path.push_str(space[1].trim_start_matches("/"));
    let space: &str = &path;

    // Login API
    if space == "/api/login" {
        let mut content_length = 0;
        let mut line = String::new();

        loop {
            line.clear();
            if reader.read_line(&mut line).unwrap_or(0) == 0 {
                break;
            }

            if line == "\r\n" {
                break;
            }

            // Gets the body of the POST request
            if line.to_lowercase().starts_with("content-length:") {
                // Get the value of content-length a..
                if let Ok(len) = line["content-length:".len()..].trim().parse::<usize>() {
                    content_length = len
                }
            }
        }

        if content_length > 0 {
            let mut body = vec![0; content_length];

            // Read the exact bytes out of the buffer
            if let Ok(_) = reader.read_exact(&mut body) {
                let body_str = String::from_utf8_lossy(&body);
                let conn = conn_to_database().unwrap();

                // Create credentials struct to put in login details from login form
                #[derive(serde::Deserialize)]
                struct Credentials {
                    username: String,
                    password: String,
                }

                let creds: Credentials = serde_json::from_str(&body_str).unwrap();

                // Contacts the database to get username and password
                let mut stmt = conn
                    .prepare("SELECT id, name, password FROM users WHERE name = ?")
                    .unwrap();
                let user = stmt
                    .query_row(params![&creds.username], |row| {
                        let id: String = row.get(0).unwrap();
                        let name: String = row.get(1).unwrap();
                        let password: String = row.get(2).unwrap();
                        Ok([id, name, password])
                    })
                    .unwrap_or_else(|_e| {
                        let body = r#"{"error":"Invalid User or Password"}"#;

                        let response = format!(
                            "HTTP/1.1 401 Unauthorized\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                            body.len(),
                            body
                        );

                        stream.write_all(response.as_bytes()).unwrap();

                        // Return nothing since could not find user in database
                        return ["".to_string(), "".to_string(), "".to_string()];
                    });

                // Process of checking user details from database and login form
                if verify(creds.password, &user[2]).unwrap_or_else(|_e| return false) {
                    // Creates struct to store id and expirey time for JWT Token
                    #[derive(serde::Serialize)]
                    struct Claims {
                        sub: String,
                        exp: usize,
                    }

                    let expire = SystemTime::now()
                        .checked_add(Duration::from_secs(30 * 24 * 60 * 60)) // 30 Days
                        .unwrap()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs() as usize;

                    // user[0] = id
                    let user_claims = Claims {
                        sub: user[0].clone(),
                        exp: expire,
                    };
                    let jwt_token = env::var("JWT_AUTHENTICATION_TOKEN").unwrap();

                    // Creates token using default algorithm HS256
                    let token = encode(
                        &Header::default(),
                        &user_claims,
                        &EncodingKey::from_secret(jwt_token.as_bytes()),
                    )
                    .unwrap();

                    let http_only_cookie = format!(
                        "Set-Cookie: token={}; HttpOnly; Secure; SameSite=Strict; Path=/\r\n",
                        token
                    );

                    let response =
                        format!("HTTP/1.1 200 OK\r\nContent-Length: 0\r\n{http_only_cookie}\r\n");

                    return stream.write_all(response.as_bytes()).unwrap();
                } else {
                    let body = r#"{"error":"Invalid User or Password"}"#;

                    let response = format!(
                        "HTTP/1.1 401 Unauthorized\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                        body.len(),
                        body
                    );

                    return stream.write_all(response.as_bytes()).unwrap();
                }
            }
        }
    }
}
