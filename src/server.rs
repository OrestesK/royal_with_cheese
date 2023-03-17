use crate::shared::Action;

use super::shared::Shared;
use std::{io::Error, sync::Arc};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{tcp::OwnedReadHalf, tcp::OwnedWriteHalf, TcpListener},
    sync::Mutex,
};

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

    // sends data to client, runs constantly
    async fn write_data_to_client(
        shared: Arc<Mutex<Shared>>,
        mut client_write_connection: OwnedWriteHalf,
    ) {
        eprintln!("Entered Tokio Write Client Thread");
        let mut fps = fps_clock::FpsClock::new(60);
        loop {
            let shared = shared.lock().await;
            let data = &shared.map;
            client_write_connection
                .write_all(b"temporary value")
                .await
                .expect("Failed to write to client");

            //eprintln!("Sent: {:?}", data);
            fps.tick();
        }
    }

    // reads data from client, runs constantly
    async fn read_data_from_client(
        shared: Arc<Mutex<Shared>>,
        mut client_read_connection: OwnedReadHalf,
        id: u8,
    ) {
        eprintln!("Entered Tokio Read Client Thread");
        let mut fps = fps_clock::FpsClock::new(60);
        loop {
            let mut buf = vec![0; 3];
            client_read_connection
                .read(&mut buf)
                .await
                .expect("Failed to read from client");

            if buf[0] == 0 {
                eprintln!("Client Disconnected");
                break;
            }

            let mut shared = shared.lock().await;
            shared.actions.push_back(Action {
                code: buf[0],
                user: id,
            });

            eprintln!("Received: {:?} from Client {:?}", buf, id);
            fps.tick();
        }
    }

    // initiates sending and reading
    pub async fn initiate(self, shared: Arc<Mutex<Shared>>) -> Result<(), Error> {
        let mut id: u8 = 0;
        loop {
            id += 1;
            let (client_connection, _) = self.connection.accept().await?;
            let shared_copy_read = shared.clone();
            let shared_copy_write = shared.clone();

            eprintln!(
                "Received connection from {:?}",
                client_connection.peer_addr().unwrap()
            );

            // splits connection into read and write connections
            let (read, write) = client_connection.into_split();

            tokio::spawn(Server::read_data_from_client(shared_copy_read, read, id));
            tokio::spawn(Server::write_data_to_client(shared_copy_write, write));
        }
    }
}
