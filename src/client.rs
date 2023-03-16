use std::io::Error;
use tokio::io::BufReader;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub struct Client {
    pub address: String,
    pub connection: TcpStream,
}

impl Client {
    pub async fn new(ip: &str, port: &str) -> Result<Self, Error> {
        let address = format!("{}:{}", ip, port); //formats address
        let connection = TcpStream::connect(&address).await?; // connects to address
        connection.set_nodelay(true)?; //disables Nagle's algorithm meaning data is sent instantly
        eprintln!("Connected to: {}", address);
        Ok(Client {
            address,
            connection,
        })
    }

    pub async fn send_data(&mut self, data: String) -> Result<(), Error> {
        // write
        let result = self.connection.write_all(data.as_bytes()).await;
        println!(
            "Streamed {:?} || success={:?}",
            data.as_bytes(),
            result.is_ok()
        );

        //Add buffering so that the receiver can read messages from the stream
        let mut reader = BufReader::new(&mut self.connection);

        // Check if this input message values are u8
        let mut buffer: Vec<u8> = Vec::new();

        // Read input information
        let _future = reader.read_until(b'\n', &mut buffer);

        eprintln!("read from server: {:?}", buffer);
        eprintln!("");

        Ok(())
    }
}
