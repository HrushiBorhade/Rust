use std::{
    fs,
    io::{prelude::*, BufReader, Result},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use web_server::ThreadPool;

fn main() {
    // Attempt to bind to the TCP address, with error handling for failure.
    let listener = match TcpListener::bind("127.0.0.1:7878") {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind to address: {}", e);
            return; // Exit early on failure to bind to address.
        }
    };

    // Create a thread pool with 5 threads.
    let pool = ThreadPool::new(5);

    // Listen for incoming connections.
    for stream in listener.incoming() {
        // Handle each connection in a separate thread in the pool.
        let _stream = match stream {
            Ok(stream) => {
                println!("Connection Established");
                pool.execute(move || {
                    if let Err(e) = handle_connection(stream) {
                        // Log any errors that occur while handling the connection.
                        eprintln!("Error handling connection: {:?}", e);
                    }
                });
            }
            Err(e) => {
                // Log error if we fail to establish a connection.
                eprintln!("Failed to establish connection: {}", e);
            }
        };
    }
    println!("Shutting Down");
}

/// Handles the HTTP connection from a client.
/// Returns a Result, with an error if something goes wrong during the process.
fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let buf_reader = BufReader::new(&stream);

    // Read the first line of the request.
    let request_line = match buf_reader.lines().next() {
        Some(Ok(line)) => line,
        Some(Err(e)) => {
            // Handle any error that occurs while reading the request line.
            eprintln!("Error reading request line: {}", e);
            return Err(e); // Propagate error up to caller.
        }
        None => {
            // Handle case where no request is received (client disconnected prematurely).
            eprintln!("Client disconnected before sending request");
            return Ok(()); // Return Ok as there is no further processing to do.
        }
    };

    // Match on the request line to determine the appropriate response.
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "welcome.html"),
        "GET /sleep HTTP/1.1" => {
            // Simulate a delay for a specific route.
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "slow-request.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    // Attempt to read the content of the requested file.
    let contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        // Log error and provide a fallback page in case of file read failure.
        eprintln!("Error loading page: {}", filename);
        String::from("Error loading page")
    });

    let length = contents.len();

    // Construct the HTTP response.
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // Attempt to write the response to the stream.
    if let Err(e) = stream.write_all(response.as_bytes()) {
        // Log any error during response writing to the client.
        eprintln!("Failed to send response: {}", e);
        return Err(e); // Return error to propagate failure.
    }

    Ok(()) // Successfully handled the request.
}
