use super::shared::Shared;
use std::io::Error;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::OwnedReadHalf;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

// struct Server
pub struct Server {
    pub address: String,
    pub connection: TcpListener,
}

impl Server {
    // creates Server connection
    pub async fn new(ip: &str, port: &str) -> Result<Self, Error> {
        let address = format!("{}:{}", ip, port); //formats address
        let connection = TcpListener::bind(&address).await?; // binds server to address
        eprintln!("Listening on: {}", address);
        Ok(Server {
            address,
            connection,
        })
    }

    // sends data to server, runs constantly
    async fn write_data_to_client(
        shared: Arc<Mutex<Shared>>,
        mut client_write_connection: OwnedWriteHalf,
    ) {
        eprintln!("Entered Tokio Write Client Thread");
        let mut fps = fps_clock::FpsClock::new(1);
        loop {
            let shared = shared.lock().await;
            let data = &shared.map;
            client_write_connection
                .write_all(data.as_slice())
                .await
                .expect("Failed to write to client");
            eprintln!("Sent: {:?}", data);

            fps.tick();
        }
    }

    // reads data from server, runs constantly
    async fn read_data_from_client(mut client_read_connection: OwnedReadHalf) {
        eprintln!("Entered Tokio Read Client Thread");
        let mut fps = fps_clock::FpsClock::new(60);
        loop {
            let mut buf = vec![0; 20];
            client_read_connection
                .read(&mut buf)
                .await
                .expect("Failed to read from client");
            if buf[0] == 0 {
                eprintln!("Client Disconnected");
                break;
            }
            if buf[0] == 122 {
                eprint!("Got z\n")
            }
            let data = String::from_utf8(buf).expect("Found invalid UTF-8");
            eprintln!("Received: {:?}", data);
            fps.tick();
        }
    }

    // initiates sending and reading
    pub async fn initiate(self, shared: Arc<Mutex<Shared>>) -> Result<(), Error> {
        loop {
            let (client_connection, _) = self.connection.accept().await?;
            let shared_copy = shared.clone();

            eprintln!(
                "Received connection from {:?}",
                client_connection.peer_addr().unwrap()
            );

            // splits connection into read and write connections
            let (read, write) = client_connection.into_split();

            tokio::spawn(Server::read_data_from_client(read));
            tokio::spawn(Server::write_data_to_client(shared_copy, write));
        }
    }
}
