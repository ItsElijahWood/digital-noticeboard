use std::{
    fs,
    io::{BufReader, Write},
    net::TcpStream,
};

use crate::jwt::manage_req::jwt_req;
use crate::posts::manage_posts;

/// req_manager is a function that allows certent files through network requests
pub fn req_manager(mut stream: &TcpStream, req_status: &String, mut reader: BufReader<&TcpStream>) {
    let status = "HTTP/1.1 200 OK";

    // Allows ttf fonts
    if req_status.contains(".ttf") {
        // Collects the string slices and substring slices in req_status
        let space: Vec<&str> = req_status.split_whitespace().collect();

        let path = format!("{}", space[1].trim_start_matches("/"));
        let content = fs::read(&path).unwrap();
        let length = content.len();
        let req_body =
            format!("{status}\r\nContent-Type: font/ttf\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {length}\r\n\r\n");

        stream.write_all(req_body.as_bytes()).unwrap();
        return stream.write_all(&content).unwrap();
    }

    // Allows pngs
    if req_status.contains(".png") {
        let space: Vec<&str> = req_status.split_whitespace().collect();

        let path = format!("{}", space[1].trim_start_matches("/"));
        let content = fs::read(&path).unwrap();
        let length = content.len();
        let req_body =
            format!("{status}\r\nContent-Type: image/png\r\nContent-Length: {length}\r\n\r\n");

        stream.write_all(req_body.as_bytes()).unwrap();
        return stream.write_all(&content).unwrap();
    }

    // Direct POST requests
    if req_status.trim().starts_with("POST") {
        manage_posts(&req_status, &mut reader, &stream);
    }

    // Check if logged in
    if req_status.starts_with("GET /api/protected")
    {
        jwt_req(&stream, &mut reader, &req_status);
    }

    // Handle adding pngs to local storage for notice board
    if req_status.starts_with("POST /api/add_img") {
        jwt_req(&stream, &mut reader, &req_status);
    }

    // Fetching all pngs for displaying on noticeboard
    if req_status.starts_with("GET /api/protected_img_fetch") {
        jwt_req(&stream, &mut reader, &req_status);
    }

    // Fetch congregation from id from token
    if req_status.starts_with("POST /api/fetch_dashboard_congregation")
    {
        jwt_req(&stream, &mut reader, &req_status);
    }
}
