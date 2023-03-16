use super::shared::Shared;
use std::io::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::OwnedReadHalf;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::TcpListener;

pub struct Server {
    pub address: String,
    pub connection: TcpListener,
    pub shared: Shared,
}

impl Server {
    pub async fn new(ip: &str, port: &str) -> Result<Self, Error> {
        let address = format!("{}:{}", ip, port); //formats address
        let connection = TcpListener::bind(&address).await?; // binds server to address
        eprintln!("Listening on: {}", address);
        let shared = Shared::new()?; //creates 'Shared' Struct
        Ok(Server {
            address,
            connection,
            shared,
        })
    }

    async fn read_data_from_client(mut client_read_connection: OwnedReadHalf) {
        eprintln!("Entered Tokio Client Thread");
        loop {
            let mut buf = vec![0; 1024];
            client_read_connection.read(&mut buf).await.unwrap();
            if buf[0] == 0 {
                eprintln!("Client Disconnected");
                break;
            }
            let data = String::from_utf8(buf).expect("Found invalid UTF-8");
            eprintln!("Data: {:?}", data);
        }
    }

    async fn send_data_to_client(mut client_write_connection: OwnedWriteHalf, data: String) {
        let future = client_write_connection.write_all(data.as_bytes());
    }

    pub async fn initiate(&mut self) -> Result<(), Error> {
        let (client_connection, _) = self.connection.accept().await?;

        eprintln!(
            "Received connection from {:?}",
            client_connection.peer_addr().unwrap()
        );

        let (read, write) = client_connection.into_split();

        tokio::spawn(Server::read_data_from_client(read));
        tokio::spawn(Server::send_data_to_client(
            write,
            "Received Data".to_string(),
        ));

        Ok(())
    }
}
