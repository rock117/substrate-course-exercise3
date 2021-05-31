use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{Read, Write, Error};
mod trafficlight;
mod int_sum;
mod area;

fn main() {
}
fn call_on_ref_zero<F>(f: F) where for<'a> F: Fn(&'a i32) {
    let zero = 0;
    f(&zero);
}
struct Closure<F> {
    data: (u8, u16),
    func: F,
}

impl<F> Closure<F>
    where F: Fn(&(u8, u16)) -> &u8,
{
    fn call(&self) -> &u8 {
        (self.func)(&self.data)
    }
}

fn mytest<T:M + N>(test:T){
    test.test_m();
    test.test_n();
}

trait M{
    fn test_m(&self);
}
trait N{
    fn test_n(&self);
}
struct Mm;


fn test(t: impl M){}
fn test2<T:M>(t: T){}

fn get_index<'l, T>(v: &'l [T], index: usize) -> &'l T {
    &v[index]
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