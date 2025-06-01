use multipart::server::Multipart;
use once_cell::sync::Lazy;
use rusqlite::params;
use std::sync::{Arc, Mutex};
use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Write},
    net::TcpStream,
};

use crate::database::conn::conn_to_database;

/// Extract boundry from content-type
fn extract_boundary(content_type: &str) -> Option<String> {
    content_type.split(';').map(|s| s.trim()).find_map(|param| {
        if param.starts_with("boundary=") {
            Some(param["boundary=".len()..].trim_matches('"').to_string())
        } else {
            None
        }
    })
}

// Get user id from claims token
static USER_ID: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));
pub fn get_id(id: &str) {
    let mut user_id = USER_ID.lock().unwrap();
    *user_id = id.to_string();
}

// Sends to local device storage
fn to_local_storage(
    file_vec: &Vec<u8>,
    type_value: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let conn = conn_to_database()?;
    let user_id = USER_ID.lock().unwrap();

    let mut stmt = conn
        .prepare("SELECT congregation FROM users WHERE id = ?")
        .unwrap();
    let congregation = stmt
        .query_row(params![&*user_id], |row| {
            let row1: String = row.get(0).unwrap();
            Ok([row1])
        })
        .unwrap();

    // Checks which congregation before writing a file to the directory
    if congregation[0] == "brampton" {
        let mut stmt = conn
            .prepare("SELECT pdf_name FROM storage WHERE congregation = ?")
            .unwrap();
        let pdfs = stmt
            .query_map([&congregation[0]], |row| row.get::<_, String>(0))
            .unwrap();

        let mut num = 0;
        for _files in pdfs {
            num += 1
        }

        let file_name = format!("public/storage/brampton/file_{}.pdf", num);
        conn.execute(
            "
                INSERT INTO storage (congregation, pdf_name, type) VALUES (?, ?, ?)
            ",
            [&congregation[0], &file_name.to_string(), type_value],
        )
        .unwrap();

        let mut new_file = File::create_new(file_name).unwrap();
        new_file.write_all(file_vec).unwrap();
    } else if congregation[0] == "scotby" {
        let mut stmt = conn
            .prepare("SELECT pdf_name FROM storage WHERE congregation = ?")
            .unwrap();
        let pdfs = stmt
            .query_map([&congregation[0]], |row| row.get::<_, String>(0))
            .unwrap();

        let mut num = 0;
        for _files in pdfs {
            num += 1
        }

        let file_name = format!("public/storage/scotby/file_{}.pdf", num);
        conn.execute(
            "
                INSERT INTO storage (congregation, pdf_name, type) VALUES (?, ?, ?)
            ",
            [&congregation[0], &file_name.to_string(), type_value],
        )
        .unwrap();

        let mut new_file = File::create_new(file_name).unwrap();
        new_file.write_all(file_vec).unwrap();
    } else if congregation[0] == "moorhouse" {
        let mut stmt = conn
            .prepare("SELECT pdf_name FROM storage WHERE congregation = ?")
            .unwrap();
        let pdfs = stmt
            .query_map([&congregation[0]], |row| row.get::<_, String>(0))
            .unwrap();

        let mut num = 0;
        for _files in pdfs {
            num += 1
        }

        let file_name = format!("public/storage/moorhouse/file_{}.pdf", num);
        conn.execute(
            "
                INSERT INTO storage (congregation, pdf_name, type) VALUES (?, ?, ?)
            ",
            [&congregation[0], &file_name.to_string(), type_value],
        )
        .unwrap();

        let mut new_file = File::create_new(file_name).unwrap();
        new_file.write_all(file_vec).unwrap();
    }

    Ok(())
}

/// Adds pdf to device storage
pub fn add_pdfs(mut stream: &TcpStream, reader: &mut BufReader<&TcpStream>) {
    let mut content_type = String::new();
    let mut content_length = 0;
    let mut line = String::new();

    loop {
        line.clear();
        if reader.read_line(&mut line).unwrap_or(0) == 0 {
            println!("Failed to read line or connection closed.");
            return;
        }

        if line == "\r\n" {
            break;
        }

        if line.to_lowercase().starts_with("content-type:") {
            if let Some(idx) = line.find(':') {
                content_type = line[(idx + 1)..].trim().to_string();
            }
        }

        if line.to_lowercase().starts_with("content-length:") {
            if let Some(idx) = line.find(':') {
                content_length = line[(idx + 1)..].trim().parse::<usize>().unwrap_or(0);
            }
        }
    }

    let boundary = match extract_boundary(&content_type) {
        Some(b) => b,
        None => {
            println!("No boundary found in Content-Type header.");
            return;
        }
    };

    // Read body
    let mut body_buf = vec![0u8; content_length];
    if let Err(e) = reader.read_exact(&mut body_buf) {
        println!("Failed to read full request body: {}", e);
        return;
    }

    let cursor = std::io::Cursor::new(body_buf);
    let mut multipart = Multipart::with_body(cursor, boundary);

    let mut value = String::new();
    // Gets type value from request
    multipart.foreach_entry(|mut e| {
        if e.headers.name == Arc::from("type") {
            e.data.read_to_string(&mut value).unwrap();
        }

        if e.headers.name == Arc::from("pdf") {
                let mut file_bytes = Vec::new();
                e.data.read_to_end(&mut file_bytes).unwrap();

                if file_bytes.is_empty() {
                    println!("File is empty.");
                    return;
                }

                if let Err(e) = to_local_storage(&file_bytes, &value) {
                    eprintln!("Error sending file data to database: {}", e);
                }

                let json = r#"{"ok": "Successfully added pdf to device storage."}"#;
                let len = json.len();
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                    len, json
                );

                return stream.write_all(response.as_bytes()).unwrap();

        }
    }).unwrap();
}
