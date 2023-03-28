use futures::executor::block_on;
use royal_with_cheese::shared::Action;
use royal_with_cheese::shared_io;
use royal_with_cheese::{client_network::Client, display, server_network::Server, shared::Shared};
use std::{
    env,
    sync::{Arc, Mutex},
};

const ADDRESS: &str = "0.0.0.0";
const PORT: &str = "8080";

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && args[1] == "S" {
        server()
    } else {
        client()
    }
}

// server side
fn server() {
    // builds server connection to socket
    let server: Server = block_on(Server::new(ADDRESS, PORT)).expect("Failed to create server");

    // creates shared 'Shared' data
    let shared = Shared::new().unwrap();
    let shared = Arc::new(Mutex::new(shared)); //creates shared 'Shared' Struct

    // initializes reading and writing from clients
    tokio::spawn(Server::initialize_server(server, shared.clone()));

    // initializes and runs GUI
    display::cursive(shared.clone(), false);
    loop {}
}

// client side
fn client() {
    // builds client connection to server
    let client: Client =
        block_on(Client::new(ADDRESS, PORT)).expect("Failed to connect to address");

    // creates shared 'Shared' data
    let shared = Shared::new().unwrap();
    let shared = Arc::new(Mutex::new(shared)); //creates shared 'Shared' Struct

    block_on(Client::initialize_client(client, shared.clone()));
    display::cursive(shared.clone(), true);
}
