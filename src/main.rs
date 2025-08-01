use instance_pipe::{Client, Server};
use std::error::Error;
use std::env;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct TestMessage {
    id: u32,
    content: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} [server|client]", args[0]);
        std::process::exit(1);
    }

    match args[1].as_str() {
        "server" => run_server(),
        "client" => run_client(),
        _ => {
            eprintln!("Invalid argument. Use 'server' or 'client'.");
            std::process::exit(1);
        }
    }
}

fn run_server() -> Result<(), Box<dyn Error>> {
    let mut server = Server::new("test_pipe")?;
    println!("Server started, waiting for connections...");

    // Accept a client connection
    let client = server.accept()?;
    println!("Client connected");

    // Receive a message from the client
    let received: TestMessage = client.recv()?;
    println!("Server received: {:?}", received);

    // Send a response back
    let response = TestMessage {
        id: received.id + 1,
        content: format!("Server response to: {}", received.content),
    };
    client.send(&response)?;
    println!("Server sent: {:?}", response);

    Ok(())
}

fn run_client() -> Result<(), Box<dyn Error>> {
    let client = Client::connect("test_pipe")?;
    println!("Client connected to server");

    // Send a message to the server
    let message = TestMessage {
        id: 1,
        content: String::from("Hello from client!"),
    };
    client.send(&message)?;
    println!("Client sent: {:?}", message);

    // Receive response from server
    let response: TestMessage = client.recv()?;
    println!("Client received: {:?}", response);

    Ok(())
}