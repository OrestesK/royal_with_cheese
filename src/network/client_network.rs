use crate::{
    dclient,
    network::shared::{Shared, FPS},
    network::shared_io::{data_to_active_tiles, get_and_clear_actions},
};
use std::{
    io::Error,
    sync::{Arc, Mutex},
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
};

// struct Client
pub struct Client {
    pub address: String,
    pub connection: TcpStream,
}

impl Client {
    // creates Client connection
    pub async fn new(ip: &str, port: &str) -> Result<Self, Error> {
        let address = format!("{}:{}", ip, port); //formats address
        let connection = TcpStream::connect(&address).await?; // connects to address
        connection.set_nodelay(true)?; //disables Nagle's algorithm meaning data is sent instantly

        dclient!("Connected to: {}", address);

        Ok(Client {
            address,
            connection,
        })
    }

    // writes data to sever, runs constantly
    pub async fn write_data_to_server(
        shared: Arc<Mutex<Shared>>,
        mut write_to_server_connection: OwnedWriteHalf,
    ) {
        //
        // write
        //
        dclient!("Writing data to server");
        let mut fps = fps_clock::FpsClock::new(FPS);
        loop {
            let actions = get_and_clear_actions(shared.clone());

            for action in actions {
                write_to_server_connection
                    .write_u8(action.code)
                    .await
                    .expect("Failed to write to server");

                dclient!("Streamed to server: {:?}", action.code);
            }
            fps.tick();
        }
    }

    // reads data from server, runs constantly but awaits data
    pub async fn read_data_from_server(
        shared: Arc<Mutex<Shared>>,
        mut read_from_server_connection: OwnedReadHalf,
    ) {
        //
        // read
        //
        dclient!("Reading data from server");
        let mut fps = fps_clock::FpsClock::new(FPS);
        loop {
            dclient!("looping");
            // reads size of incoming data
            let size = read_from_server_connection
                .read_u16()
                .await
                .expect("Failed to read content size") as u16;
            dclient!("debug");
            if size == 0 {
                dclient!("Empty active tiles"); //only happens whens debugging
                fps.tick();
                continue;
            }

            // VERY CAREFUL, usize can be smaller than u16 VERY BAD
            let mut active_tiles_data = vec![0; size as usize];
            read_from_server_connection
                .read_exact(&mut active_tiles_data)
                .await
                .expect("Failed to read from server");

            dclient!("Received: {:?} {:?}", size / 3, active_tiles_data);

            tokio::spawn(data_to_active_tiles(shared.clone(), active_tiles_data));

            fps.tick();
        }
    }

    // initializes client
    pub async fn initialize_client(self, shared: Arc<Mutex<Shared>>) {
        // splits connection into read and write connections
        let (read, write) = self.connection.into_split();

        // spawns reading and writing threads
        tokio::spawn(Client::read_data_from_server(shared.clone(), read));
        tokio::spawn(Client::write_data_to_server(shared.clone(), write));
    }
}
