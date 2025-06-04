use once_cell::sync::Lazy;
use serde_json::to_string;
use std::{io::Write, net::TcpStream, sync::Mutex};

use crate::database::conn::conn_to_database;

static USER_ID: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));

pub fn get_id_(id: &str) {
    if let Ok(mut user_id) = USER_ID.lock() {
        *user_id = id.to_string();
    } else {
        eprintln!("Warning: USER_ID mutex poisoned while setting id");
    }
}

// Fetch png paths and types from database
pub fn fetch_pngs(mut stream: &TcpStream) {
    let conn = conn_to_database().unwrap();

    let id = match USER_ID.lock() {
        Ok(guard) => guard.clone(),
        Err(poisoned) => {
            eprintln!("USER_ID mutex poisoned");
            poisoned.into_inner().clone()
        }
    };

    let congregation_res: Result<String, _> = conn
        .query_row(
            "SELECT congregation FROM users WHERE id = ?",
            [&id],
            |row| row.get(0),
        )
        .map_err(|e| {
            eprintln!("No user found with ID '{}'. Error: {:?}", id, e);
            e
        });

    let congregation = match congregation_res {
        Ok(c) => c,
        Err(e) => {
            eprintln!(
                "error retrieving rows from database for id: '{}': {:?}",
                id,
                e
            );
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
