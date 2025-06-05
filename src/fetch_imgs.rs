use serde_json::to_string;
use std::{io::Write, net::TcpStream};

use crate::database::conn::conn_to_database;

// Fetch png paths and types from database
pub fn fetch_pngs(mut stream: &TcpStream, id: &str) {
    let conn = conn_to_database().unwrap();

    let congregation_res: Result<String, _> = conn.query_row(
        "SELECT congregation FROM users WHERE id = ?",
        [&id],
        |row| row.get(0),
    );

    let congregation = match congregation_res {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to unwrap congregation from congregation_res: {}", e);
            return;
        }
    };

    let mut stmt2 = match conn.prepare("SELECT img_name, type FROM storage") {
        Ok(stmt) => stmt,
        Err(e) => {
            eprintln!("Failed to prepare statement: {:?}", e);
            return;
        }
    };

    let query = match stmt2.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)
                .expect("Failed to get img_name from database"),
            row.get::<_, String>(1)
                .expect("Failed to get type from database"),
        ))
    }) {
        Ok(rows) => rows,
        Err(e) => {
            eprintln!("Query failed: {:?}", e);
            return;
        }
    };

    // Unwrap the values of the type and img relative paths
    let mut arr: Vec<(String, String, &String)> = Vec::new();
    for row_res in query {
        let (frow, srow) = row_res.expect("Failed to unwrap Result<(Result<String, Error>, Error)");

        arr.push((frow, srow, &congregation));
    }

    let json_arr = to_string(&arr).unwrap();
    let length = json_arr.len();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {length}\r\n\r\n{}",
        json_arr
    );

    stream.write_all(response.as_bytes()).unwrap();
}
