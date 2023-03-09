// The following code imports the necessary modules for TcpListener and TcpStream
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use std::thread;
use std::time;

use server_app::ThreadPool;

// This is the main function.
fn main() {
    // Create a new listener bound to localhost at port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);    

    // Start listening to incoming connections.
    // Incoming returns an iterator, meaning we can iterate over all incoming
    // connections and handle them individually.
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            println!("Hello from the pool!");
            handle_connection(stream);
        });  
    }
}

// This function handles the TCP connection streams
fn handle_connection(mut stream: TcpStream) {
    // create a buffer to store the contents of the request
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // Check if the first line of the request starts with "GET / HTTP/1.1"
    // We will only serve requests such as this one.
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    // If it does start with the expected string, return index.html file,
    // else return 404.html file
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")            
    }
    else if buffer.starts_with(sleep){
        // Sleep for 5 seconds
        std::thread::sleep(std::time::Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "index.html")
    }
    else{
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    // print the received request
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // Read the contents of file specified by filename variable
    // This should contain HTML that the client requested for.
    let contents = fs::read_to_string(filename).unwrap();

    // Generate HTTP response headers for the client, which include:
    //   - status line
    //   - content length
    //   - and blank line to separate headers from body 
    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", 
    status_line, 
    contents.len(),
    contents);

    // Send the response to the stream (i.e. send it back to the client)
    stream.write(response.as_bytes()).unwrap();
    
    // Flush the output stream.
    stream.flush().unwrap();
}
