use futures::executor::block_on;
use royal_with_cheese::{client_network::Client, display, server_network::Server, shared::Shared, dprint};
use std::{
    env,
    panic,
    sync::{Arc, Mutex},
};

const ADDRESS: &str = "0.0.0.0";
const PORT: &str = "8080";

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    // panic_hook();
    
    if args.len() == 2 && args[1] == "S" {
        server()
    } else {
        client()
    }
}

// declares panic hook
fn panic_hook(){
    panic::set_hook(Box::new(|panic_info| {
        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            dprint!("panic: {:?}", s);
        } else {
            dprint!("panic occurred");
        }
    }));
}

// server side
fn server() {
    // builds server connection to socket
    let server: Server = 
        block_on(Server::new(ADDRESS, PORT)).expect(&format!("Failed to listen on {}:{}", ADDRESS, PORT)[..]);


    // creates shared 'Shared' data
    let shared = Shared::new().unwrap();
    let shared = Arc::new(Mutex::new(shared)); //creates shared 'Shared' Struct

    // initializes server, reading/writing from/to clients
    tokio::spawn(Server::initialize_server(server, shared.clone()));

    // wait for 3 seconds, #TODO make a concurrent listener
    let _ = tokio::time::sleep(tokio::time::Duration::from_secs(2)); //drop(var_name) to end early

    // initializes and runs GUI
    display::cursive(shared.clone(), false);

    loop{} //FOR TEST ONLY
}

// client side
fn client() {
    // builds client connection to server
    let client: Client =
        block_on(Client::new(ADDRESS, PORT)).expect(&format!("Failed to connect to {} on {}", ADDRESS, PORT)[..]);

    // creates shared 'Shared' data
    let shared = Shared::new().unwrap();
    let shared = Arc::new(Mutex::new(shared)); //creates shared 'Shared' Struct

    // initializes client, reading/writing from/to server
    block_on(Client::initialize_client(client, shared.clone()));

    // initializes and runs GUI
    display::cursive(shared.clone(), true);
}
