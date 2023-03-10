use std::io::{self, prelude::*, BufReader, Write};
use std::net::TcpStream;
use std::str;

pub struct Client {
    pub address: String,
    pub connection: TcpStream,
}

impl Client {
    pub fn new(ip: &str, port: &str) -> Result<Self, io::Error> {
        let address = format!("{}:{}", ip, port);
        let connection = TcpStream::connect(&address)?;
        Ok(Client {
            address,
            connection,
        })
    }

    pub fn send_data(&mut self, data: String) -> Result<(), io::Error> {
        // Write the message so that the receiver can access it
        self.connection
            .write(data.as_bytes())
            .expect("failed to write");

        Ok(())
    }

    pub fn read_data(&mut self) -> Result<(), io::Error> {
        //Add buffering so that the receiver can read messages from the stream
        let mut reader = BufReader::new(&self.connection);

        // Check if this input message values are u8
        let mut buffer: Vec<u8> = Vec::new();

        // Read input information
        reader.read_until(b'\n', &mut buffer)?;

        println!("read from server:{}", str::from_utf8(&buffer).unwrap());
        println!("");
        Ok(())
    }
}
