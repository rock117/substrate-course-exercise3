use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{Read, Write, Error};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let port:u32 = if args.is_empty() {
        9999
    } else {
        (&args[0]).parse::<u32>().unwrap_or(9999)
    };
    println!("Server start at {}", port);
    start_server(port)
}

pub fn start_server(port: u32) {
    // init a ServerSocket, listen to {port}
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).expect(format!("Server binding to port {} failed", port).as_str());
    loop {
        // wait for remote connection
        match listener.accept() {
            Ok(mut client) => {
                // accepted connection success, launch a thread to handle this connection
                std::thread::spawn(move || {
                    let handle_result = handle_client(client.0);
                    match handle_result {
                       Err(e) => eprintln!("Remote connection network error:: {:?}", e),
                        _ => println!("Remote connection closed!")
                    }
                });
            }
            // failed to accepted connection
            Err(_) => {
                eprintln!("Failed to accept client connection!")
            }
        }

    }
}

fn handle_client(mut client: TcpStream) -> std::io::Result<()> {
    // repeatedly read data from remote server and write back the received data
    // until remote server is closed or a network error occur
    loop {
        let mut buf = [0u8; 1024];
        // read data into buf from client
        let result = client.read(&mut buf);
        match result {
            Ok(0) => { // remote server closed
                return Ok(());
            }
            Ok(n) => { // read n bytes data
                let mut content: &[u8] = &buf[0..n];
                // convert bytes to string
                let content_str = String::from_utf8_lossy(content).to_string();
                print!("{}", content_str);
                std::io::stdout().flush()?;
                // write data back to client
                client.write(content)?;
                // flush the socket to sending data from buffer immediately
                client.flush()?;
            }
            Err(e) => { // network error
                return Err(e);
            }
        }
    }
}