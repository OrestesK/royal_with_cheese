use std::{
    io::Error,
    sync::{Arc, Mutex},
};
use crate::shared_io::{self, data_to_active_tiles};
use super::{shared::Shared, shared::FPS, dprint};
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
        
        dprint!("Connected to: {}", address);
        
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
                let code = actions.get(i).unwrap().code;
                let data = vec![code];
                write_to_server_connection
                    .write(data.as_slice())
                    .await
                    .expect("Failed to write");
                
                dprint!("Streamed to server: {:?}", data.as_slice());
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
        let mut fps = fps_clock::FpsClock::new(FPS);
        loop {

            let size = read_from_server_connection.read_u8().await.expect("Failed to read content size") as usize;
            if size == 0{ dprint!("Empty active tiles"); continue;} //only happens when debugging :)

            let mut buf = vec![0; size];
            read_from_server_connection
                .read_exact(&mut buf)
                .await
                .expect("Failed to read from server");

            dprint!("Received: {:?} {:?}", size/3, buf);

            if buf[0] == 0 {
                dprint!("Server Disconnected");
                break;
            }

            tokio::spawn(data_to_active_tiles(shared.clone(), buf));

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
