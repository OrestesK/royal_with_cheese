use std::io::Error;
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
        write_to_server_connection: &mut OwnedWriteHalf,
        data: String,
    ) -> Result<(), Error> {
        //
        // write
        //
        write_to_server_connection
            .write_all(data.as_bytes())
            .await?;
        println!("Streamed to server: {:?}", data.as_bytes(),);

        Ok(())
    }

    async fn read_data_from_server(mut server_read_connection: OwnedReadHalf) -> Result<(), Error> {
        //
        // read
        //
        eprintln!("Entered Tokio Read Client Thread");
        let mut fps = fps_clock::FpsClock::new(1);
        loop {
            let mut buf = vec![0; 3];
            eprintln!("before wait");
            server_read_connection
                .read(&mut buf)
                .await
                .expect("Failed to read from server");
            eprintln!("after wait");
            eprintln!("Received: {:?}", buf);
            fps.tick();
        }
    }

    pub async fn initiate(self) -> Result<OwnedWriteHalf, Error> {
        let (read, write) = self.connection.into_split();
        tokio::spawn(Client::read_data_from_server(read));

        Ok(write)
    }
}
