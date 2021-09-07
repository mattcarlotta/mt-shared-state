use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

mod scheduler;
mod worker;

fn main() {
    let host = format!("{}:{}", "127.0.0.1", 5000);

    println!("Listening for requests on: {}", &host);

    let listener = TcpListener::bind(host).unwrap();
    let scheduler = scheduler::Scheduler::new();

    let c = Arc::new(Mutex::new(0));
    let counter = Arc::clone(&c);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => scheduler.create(|| {
                handle_request(stream, counter);
            }),
            Err(e) => println!("Unable to handle request: {}", e),
        }
    }
}

pub fn handle_request(mut stream: TcpStream, counter: Arc<Mutex<u32>>) {
    let mut buffer = [0; 1024];

    let mut hit_counter = counter.lock().unwrap();

    *hit_counter += 1;

    stream.read(&mut buffer).unwrap();

    let body = fs::read_to_string("hello.html").unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        body.len(),
        body
    );

    println!("Hit counter: {}", hit_counter);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
