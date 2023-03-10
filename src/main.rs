use cursive::views::{Dialog, TextView};

use royal_with_cheese::client::Client;
use royal_with_cheese::server::Server;
use std::env;
use std::io;

const ADDRESS: &str = "localhost";
const PORT: &str = "7878";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && args[1] == "S" {
        server()
    } else {
        client()
    }
}

fn server() {
    // unwrap panics on error
    let server: Server = Server::new(ADDRESS, PORT).unwrap();

    Server::init_server(server).unwrap_or_else(|error| eprintln!("{:?}", error));
}

fn client() {
    //cursive();

    // unwrap panics on error
    let mut client: Client = Client::new(ADDRESS, PORT).unwrap();

    loop {
        let input = terminal_input();
        Client::send_data(&mut client, input).unwrap_or_else(|error| eprintln!("{:?}", error));
        Client::read_data(&mut client).unwrap_or_else(|error| eprintln!("{:?}", error));
    }
}

// TEMPORARY FOR TESTING
fn terminal_input() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read");

    input
}

fn cursive() {
    let mut siv = cursive::default();

    siv.add_layer(
        Dialog::around(TextView::new("Hello Dialog!"))
            .title("Cursive")
            .button("Quit", |s| s.quit()),
    );
    siv.run();
}
