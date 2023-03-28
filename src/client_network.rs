use std::{
    io::Error,
    sync::{Arc, Mutex},
};

use crate::shared_io;

use super::shared::Shared;
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
        eprintln!("Connected to: {}", address);
        Ok(Client {
            address,
            connection,
        })
    }

    pub async fn write_data_to_server(
        shared: Arc<Mutex<Shared>>,
        mut write_to_server_connection: OwnedWriteHalf,
    ) {
        //
        // write
        //
        loop {
            let actions = shared_io::get_and_clear_server_actions(shared.clone());
            for i in 0..actions.len() {
                eprintln!("{:?}", actions);
                let code = actions.get(i).unwrap().code;
                let data = vec![code];
                write_to_server_connection
                    .write(data.as_slice())
                    .await
                    .expect("Failed to write");
                println!("Streamed to server: {:?}", data.as_slice());
            }
        }
    }

    pub async fn read_data_from_server(
        shared: Arc<Mutex<Shared>>,
        mut read_from_server_connection: OwnedReadHalf,
    ) {
        //
        // read
        //
        eprintln!("Entered Tokio Read Client Thread");
        let mut fps = fps_clock::FpsClock::new(60);
        loop {
            // let mut buf = vec![0; 100 * 100 * 3];
            let mut buf = vec![0; 9];
            read_from_server_connection
                .read(&mut buf)
                .await
                .expect("Failed to read from server");

            eprintln!("Received: {:?}", buf);

            if buf[0] == 0 {
                eprintln!("Server Disconnected");
                break;
            }

            tokio::spawn(shared_io::data_to_active_tiles(shared.clone(), buf));

            fps.tick();
        }
    }

    pub async fn initialize_client(self, shared: Arc<Mutex<Shared>>) {
        // splits connection into read and write connections
        let (read, write) = self.connection.into_split();

        // initializes reading data from server
        tokio::spawn(Client::read_data_from_server(shared.clone(), read));
        tokio::spawn(Client::write_data_to_server(shared.clone(), write));
    }
}
