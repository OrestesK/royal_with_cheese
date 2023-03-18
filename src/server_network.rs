use super::shared::Shared;
use super::{shared::Action, shared_io};
use std::{
    io::Error,
    sync::{Arc, Mutex},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{tcp::OwnedReadHalf, tcp::OwnedWriteHalf, TcpListener},
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
        //
        // write
        //
        eprintln!("Entered Tokio Write Client Thread");
        let mut fps = fps_clock::FpsClock::new(1);
        loop {
            let data_to_send = shared_io::active_tiles_to_data(shared.clone());

            _ = client_write_connection
                .write_all(data_to_send.as_slice())
                .await;
            eprintln!("Sent: {:?}", data_to_send.as_slice());
            fps.tick();
        }
    }

    // reads data from client, runs constantly
    async fn read_data_from_client(
        shared: Arc<Mutex<Shared>>,
        mut client_read_connection: OwnedReadHalf,
        id: u8,
    ) {
        //
        // read
        //
        eprintln!("Entered Tokio Read Client Thread");
        let mut fps = fps_clock::FpsClock::new(60);
        loop {
            let mut buf = vec![0; 2]; //CHANGE TO 1 AFTER TESTING
            client_read_connection
                .read(&mut buf)
                .await
                .expect("Failed to read from client");

            if buf[0] == 0 {
                eprintln!("Client Disconnected");
                break;
            }

            // adds action
            shared_io::add_action(
                shared.clone(),
                Action {
                    user: id,
                    code: buf[0],
                },
            );

            // updates active tiles
            tokio::spawn(shared_io::update_active_tiles(shared.clone()));

            eprintln!("Received: {:?} from Client {:?}", buf, id);
            fps.tick();
        }
    }

    // initiates reading and writing
    pub async fn initialize_server(self, shared: Arc<Mutex<Shared>>) {
        let mut id: u8 = 0;
        loop {
            id += 1;
            let (client_connection, _) = self
                .connection
                .accept()
                .await
                .expect("Failed to accept connection");

            eprintln!(
                "Received connection from {:?}",
                client_connection.peer_addr().unwrap()
            );

            // splits connection into read and write connections
            let (read, write) = client_connection.into_split();

            tokio::spawn(Server::read_data_from_client(shared.clone(), read, id));
            tokio::spawn(Server::write_data_to_client(shared.clone(), write));
        }
    }
}
