use multipart::server::Multipart;
use rusqlite::params;
use std::collections::HashMap;
use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Write},
    net::TcpStream,
};

use crate::database::conn::conn_to_database;

/// Extract boundary from content-type header
fn extract_boundary(content_type: &str) -> Option<String> {
    println!("Extracting boundary from Content-Type: '{}'", content_type);
    content_type.split(';').map(|s| s.trim()).find_map(|param| {
        if param.starts_with("boundary=") {
            let b = param["boundary=".len()..].trim_matches('"').to_string();
            println!("Found boundary: '{}'", b);
            Some(b)
        } else {
            None
        }
    })
}

/// Save file to local storage and update DB
fn to_local_storage(
    file_vec: &[u8],
    typevalue: &str,
    user_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let conn = conn_to_database()?;

    let mut stmt = conn.prepare("SELECT congregation FROM users WHERE id = ?")?;
    let congregation: String = stmt.query_row(params![&user_id], |row| row.get(0))?;

    let mut stmt = conn.prepare("SELECT img_name FROM storage WHERE congregation = ?")?;
    let imgs = stmt.query_map([&congregation], |row| row.get::<_, String>(0))?;

    let mut num = 0;
    for _ in imgs {
        num += 1;
    }

    let dir_path = format!("public/storage/{}", congregation);
    std::fs::create_dir_all(&dir_path)?;

    let file_name = format!("{}/file_{}.png", dir_path, num);

    conn.execute(
        "INSERT INTO storage (congregation, img_name, type) VALUES (?, ?, ?)",
        [&congregation, &file_name, typevalue],
    )?;

    let mut new_file = File::create(file_name)?;
    new_file.write_all(file_vec)?;

    Ok(())
}

/// Adds images to device storage from multipart data
pub fn add_imgs(
    mut stream: &TcpStream,
    reader: &mut BufReader<&TcpStream>,
    id: &str,
    content_type: &str,
    content_length: usize
) {
    let boundary = match extract_boundary(&content_type) {
        Some(b) => b,
        None => {
            println!("No boundary found in Content-Type header");
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

    // Collect parts in a hashmap
    let mut parts: HashMap<String, Vec<u8>> = HashMap::new();

    if let Err(e) = multipart.foreach_entry(|mut e| {
        let mut buf = Vec::new();
        e.data.read_to_end(&mut buf).unwrap();
        parts.insert(e.headers.name.to_string(), buf);
    }) {
        eprintln!("Error reading multipart entries: {}", e);
        return;
    }

    let type_value = match parts.get("type") {
        Some(buf) => String::from_utf8_lossy(buf).to_string(),
        None => {
            println!("No type part found in multipart data");
            return;
        }
    };

    let file_bytes = match parts.get("img") {
        Some(buf) if !buf.is_empty() => buf,
        Some(_) => {
            println!("File img part is empty");
            return;
        }
        None => {
            println!("No img part found in multipart");
            return;
        }
    };

    if let Err(e) = to_local_storage(file_bytes, &type_value, &id) {
        eprintln!("Error sending file data to local storage: {}", e);
        return;
    }

    let json = r#"{"ok": "Successfully added png to device storage."}"#;
    let len = json.len();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        len, json
    );

    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to write response: {}", e);
    }
}
