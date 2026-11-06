
use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufWriter, Read, Write};

fn handle_client(stream: TcpStream) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut writer = BufWriter::new(stream);

    let mut buffer = [0; 512];

    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                writer.write_all(&buffer[..n]).unwrap();
                writer.flush().unwrap();

                println!(
                    "Received: {}",
                    String::from_utf8_lossy(&buffer[..n])
                );
            }
            Err(e) => {
                println!("Error reading stream: {}", e);
                break;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server listening on port 7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                handle_client(stream);
            }
            Err(e) => println!("Connection failed: {}", e),
        }
    }
}
