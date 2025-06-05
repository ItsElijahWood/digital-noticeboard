use serde_json::to_string;
use std::{io::Write, net::TcpStream};

use crate::database::conn::conn_to_database;

// Fetch congregation from token id
pub fn fetch_congregation(mut stream: &TcpStream, id: &str) {
    let conn = conn_to_database().unwrap();

    let congregation_res: Result<String, _> = conn
        .query_row(
            "SELECT congregation FROM users WHERE id = ?",
            [id],
            |row| row.get(0),
        );

    let congregation = congregation_res.unwrap();

    let json_arr = to_string(&congregation).unwrap();
    let length = json_arr.len();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {length}\r\n\r\n{}",
        json_arr
    );

    stream.write_all(response.as_bytes()).unwrap();
}
