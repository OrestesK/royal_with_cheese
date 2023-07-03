use super::{shared::Shared, shared::Action, shared_io, shared::FPS, dprint};

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
        
        dprint!("Listening on: {}", address);
        
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
        let mut fps = fps_clock::FpsClock::new(FPS);
        loop {
            let data_to_send = shared_io::active_tiles_to_data(shared.clone());

            _ = client_write_connection.write_u8(data_to_send.len() as u8).await;

            _ = client_write_connection
                .write_all(data_to_send.as_slice())
                .await;
            
            dprint!("Sent: {:?}", data_to_send.as_slice());
            
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
        let mut fps = fps_clock::FpsClock::new(FPS);
        loop {
            let action = client_read_connection
                .read_u8()
                .await
                .expect("Failed to read from client (Client Disconnected)");

            if action == 0 {
                dprint!("Client Disconnected");
                break;
            }

            // pushes an action
            shared_io::push_action(
                shared.clone(),
                Action {
                    user: id,
                    code: action,
                },
            );

            dprint!("Received: {:?} from Client {:?}", action, id);

            // processes client actions (updates active tiles)
            tokio::spawn(shared_io::process_actions(shared.clone()));

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

            dprint!(
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
