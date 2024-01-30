use crate::{
    dserver,
    network::shared::{Shared, FPS},
    network::shared_io::{active_tiles_to_data, add_tile, process_actions, push_action},
};
use std::{
    io::Error,
    sync::{Arc, Mutex},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpListener,
    },
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

        dserver!("Listening on: {}", address);

        Ok(Server {
            address,
            connection,
        })
    }

    // writes data to client, runs constantly
    async fn write_data_to_client(
        shared: Arc<Mutex<Shared>>,
        mut client_write_connection: OwnedWriteHalf,
        id: u8,
    ) -> Result<(), Error> {
        //
        // write
        //
        dserver!("Writing to client {:?}", id);
        let mut fps = fps_clock::FpsClock::new(FPS);
        loop {
            let data_to_send = active_tiles_to_data(shared.clone());
            if data_to_send.len() == 0 {
                fps.tick();
                continue;
            }

            _ = client_write_connection
                .write_u16(data_to_send.len() as u16)
                .await?;

            _ = client_write_connection
                .write_all(data_to_send.as_slice())
                .await?;

            dserver!(
                "Sent: [{:?}:{:?}] {:?}",
                id,
                data_to_send.len() / 3,
                data_to_send.as_slice()
            );

            fps.tick();
        }
    }

    // reads data from client, runs constantly but awaits data
    async fn read_data_from_client(
        shared: Arc<Mutex<Shared>>,
        mut client_read_connection: OwnedReadHalf,
        id: u8,
    ) -> Result<(), Error> {
        //
        // read
        //
        dserver!("Reading from client {:?}", id);
        let mut fps = fps_clock::FpsClock::new(FPS);
        loop {
            let action = client_read_connection.read_u8().await?;
            // .expect("Failed to read from client (Client Disconnected)");

            // pushes an action
            push_action(shared.clone(), id as u8, action as u8);

            dserver!("Received: {:?} from Client {:?}", action, id);

            // WANTED only a single thread for this
            // processes client actions (updates active tiles)
            // tokio::spawn(process_actions(shared.clone()));

            fps.tick();
        }
    }

    async fn process_all_client_actions(shared: Arc<Mutex<Shared>>) {
        let mut fps = fps_clock::FpsClock::new(FPS);

        loop {
            // maybe make this tokio spawn later
            process_actions(shared.clone()).await;
            fps.tick();
        }
    }

    // initializes server, loop runs constantly to accept new clients
    pub async fn initialize_server(self, shared: Arc<Mutex<Shared>>) {
        // process actions
        tokio::spawn(Server::process_all_client_actions(shared.clone()));

        // server loop
        let mut id: u8 = 0;
        while id < 4 {
            id += 1;
            let (client_connection, _) = self
                .connection
                .accept()
                .await
                .expect("Failed to accept connection");

            dserver!(
                "Received connection from {:?}",
                client_connection.peer_addr().unwrap()
            );

            // places user on map
            add_tile(shared.clone(), id, 0, 20 * id, 20 * id).await;
            add_tile(shared.clone(), id, 0, 20 * id + 1, 20 * id).await;
            add_tile(shared.clone(), id, 0, 20 * id + 2, 20 * id).await;

            // splits connection into read and write connections
            let (read, write) = client_connection.into_split();

            // spawns reading and writing threads
            tokio::spawn(Server::read_data_from_client(shared.clone(), read, id));
            tokio::spawn(Server::write_data_to_client(shared.clone(), write, id));
        }
    }
}
