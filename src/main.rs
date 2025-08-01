use instance_pipe::{Client, Server, Event};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::{self, Read};
use std::env;
use std::thread;
use std::time::Duration;

// サーバーとクライアント間で送受信するメッセージ構造体。
#[derive(Serialize, Deserialize, Debug, Clone)]
enum Message {
    Key(String),
    Response(String),
}

fn main() -> Result<(), Box<dyn Error>> {
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

// サーバーモードを実行します。
fn run_server() -> Result<(), Box<dyn Error>> {
    let mut server = Server::start("key_pipe")?;
    println!("Server started, waiting for connections...");

    loop {
        match server.poll_event() {
            Ok(Some(Event::ConnectionAccepted(client))) => {
                println!("Client connected");
                // クライアントハンドリングを別スレッドで実行
                let client_clone = client.clone();
                thread::spawn(move || handle_client(client_clone).unwrap_or_else(|e| eprintln!("Client handler error: {}", e)));
            }
            Ok(Some(Event::MessageSent)) => {
                println!("Server sent a message (unexpected)");
            }
            Ok(Some(Event::MessageReceived(_))) => {
                println!("Server received a message (unexpected)");
            }
            Ok(None) => {
                // イベントなし、ループ継続
            }
            Err(e) => {
                eprintln!("Server error: {}", e);
                break;
            }
        }
        thread::sleep(Duration::from_millis(50));
    }

    server.stop()?;
    println!("Server stopped");
    Ok(())
}

// クライアントをハンドリングします。
fn handle_client(mut client: Client) -> Result<(), Box<dyn Error + Send + Sync>> {
    loop {
        match client.poll_event::<Message>() {
            Ok(Some(Event::MessageReceived(Message::Key(key)))) => {
                println!("Server received key: {}", key);
                // レスポンスを送信
                let response = Message::Response(format!("Received key: {}", key));
                client.send(&response)?;
                println!("Server sent response");
            }
            Ok(Some(Event::MessageReceived(Message::Response(_)))) => {
                println!("Server received unexpected response");
            }
            Ok(Some(Event::MessageSent)) => {
                println!("Server sent a message (handled)");
            }
            Ok(Some(Event::ConnectionAccepted(_))) => {
                println!("Unexpected connection event in client handler");
            }
            Ok(None) => {
                // イベントなし
            }
            Err(e) => {
                eprintln!("Client handler error: {}", e);
                break;
            }
        }
        thread::sleep(Duration::from_millis(50));
    }
    client.stop()?;
    Ok(())
}

// クライアントモードを実行します。
fn run_client() -> Result<(), Box<dyn Error>> {
    // サーバーが起動するのを待つ
    thread::sleep(Duration::from_millis(100));

    let mut client = Client::start("key_pipe")?;
    println!("Client connected to server");

    // 標準入力からキーを読み込む
    println!("Enter key to send (end with Ctrl+D or Ctrl+Z):");
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let key = buffer.trim().to_string();

    // サーバーにキーを送信
    let message = Message::Key(key.clone());
    client.send(&message)?;
    println!("Client sent key: {}", key);

    // サーバーからのレスポンスを受信
    loop {
        match client.poll_event::<Message>() {
            Ok(Some(Event::MessageReceived(Message::Response(response)))) => {
                println!("Client received response: {}", response);
                break;
            }
            Ok(Some(Event::MessageReceived(Message::Key(_)))) => {
                println!("Client received unexpected key");
            }
            Ok(Some(Event::MessageSent)) => {
                println!("Client sent a message (handled)");
            }
            Ok(Some(Event::ConnectionAccepted(_))) => {
                println!("Unexpected connection event in client");
            }
            Ok(None) => {
                // イベントなし
            }
            Err(e) => {
                eprintln!("Client error: {}", e);
                break;
            }
        }
        thread::sleep(Duration::from_millis(50));
    }

    client.stop()?;
    println!("Client stopped");
    Ok(())
}