use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use crate::pages::dashboard::dashboard_page;
use crate::pages::index::index_page;
use crate::pages::shared_map::shared_map_page;
use crate::pages::brampton_map::brampton_map_page;
use crate::pages::moorhouse_map::moorhouse_map_page;
use crate::pages::scotby_map::scotby_map_page;
use crate::pages::moorhouse::moorhouse_page;
use crate::pages::scotby::scotby_page;
use crate::pages::login::login_page;
use crate::pages::brampton::brampton_page;
use crate::req_manager::req_manager;

/// Starts listening for clients using TcpListener
pub fn server_initiasise(port: &str) {
    // Puts the port and the ip together
    let addr = format!("127.0.0.1:{}", port);

    // Creates a new listener for clients and returns the Result<TcpListener, Error> using
    // .unwrap()
    let listener = TcpListener::bind(addr).unwrap();

    println!("Running on 127.0.0.1:{}", port);

    for stream in listener.incoming() {
        // Spawn a thread so multiple clients can connect at the same time
        thread::spawn(move || {
            // Unwrap the value of Result<TcpStream, Error>
            let stream = stream.unwrap();

            stream_handler(&stream);
        });
    }
}

fn stream_handler(mut stream: &TcpStream) {
    // Creates a new reader to read a tcpstream
    let mut buf = BufReader::new(stream);
    let mut req_status = String::new();

    // Reads the tcpstream and appends the Result into the mutable string
    buf.read_line(&mut req_status).unwrap();

    // Allows specific requests
    req_manager(&stream, &req_status, buf);

    // Pages
    if req_status.trim() == "GET / HTTP/1.1" {
        let content = index_page();
        let length = content.len();
        let req_body = format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{content}");

        stream.write_all(req_body.as_bytes()).unwrap();
    } else if req_status.trim() == "GET /login HTTP/1.1" {
        let content = login_page();
        let length = content.len();
        let req_body = format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{content}");

        stream.write_all(req_body.as_bytes()).unwrap();
    } else if req_status.trim() == "GET /dashboard HTTP/1.1" {
        let content = dashboard_page();
        let length = content.len();
        let req_body = format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{content}");

        stream.write_all(req_body.as_bytes()).unwrap();
    } else if req_status.trim() == "GET /brampton HTTP/1.1" {
        let content = brampton_page();
        let length = content.len();
        let req_body = format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{content}");

        stream.write_all(req_body.as_bytes()).unwrap();
    } else if req_status.trim() == "GET /moorhouse HTTP/1.1" {
        let content = moorhouse_page();
        let length = content.len();
        let req_body = format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{content}");

        stream.write_all(req_body.as_bytes()).unwrap();
    } else if req_status.trim() == "GET /scotby HTTP/1.1" {
        let content = scotby_page();
        let length = content.len();
        let req_body = format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{content}");

        stream.write_all(req_body.as_bytes()).unwrap();
    } else if req_status.trim() == "GET /shared_map HTTP/1.1" {
        let content = shared_map_page();
        let length = content.len();
        let req_body = format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{content}");

        stream.write_all(req_body.as_bytes()).unwrap();
    } else if req_status.trim() == "GET /brampton_map HTTP/1.1" {
        let content = brampton_map_page();
        let length = content.len();
        let req_body = format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{content}");

        stream.write_all(req_body.as_bytes()).unwrap();
    } else if req_status.trim() == "GET /scotby_map HTTP/1.1" {
        let content = scotby_map_page();
        let length = content.len();
        let req_body = format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{content}");

        stream.write_all(req_body.as_bytes()).unwrap();
    } else if req_status.trim() == "GET /moorhouse_map HTTP/1.1" {
        let content = moorhouse_map_page();
        let length = content.len();
        let req_body = format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{content}");

        stream.write_all(req_body.as_bytes()).unwrap();
    }
}
