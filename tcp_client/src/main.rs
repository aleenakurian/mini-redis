
use std::io::{BufReader, BufWriter, Read, Write};
use std::net::TcpStream;

fn main() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(stream) => {
            println!("Successfully connected to server");

            let mut reader = BufReader::new(stream.try_clone().unwrap());
            let mut writer = BufWriter::new(stream);

            let messages = [
    b"Hello, server!".as_slice(),
    b"How are you?".as_slice(),
    b"This is message 3!".as_slice(),
];

for msg in messages {
    writer.write_all(msg).unwrap();
    writer.flush().unwrap();
    println!("Sent: {}", String::from_utf8_lossy(msg));
}//when stream is flushed, the entire msg is written out in a single system call

            

            let mut buffer = [0; 512];

            match reader.read(&mut buffer) {
                Ok(n) => {
                    println!(
                        "Received: {}",
                        String::from_utf8_lossy(&buffer[..n])
                    );
                }
                Err(e) => println!("Failed to receive data: {}", e),
            }
        }
        Err(e) => println!("Failed to connect: {}", e),
    }
}
