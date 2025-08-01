use instance_pipe::{Client, Server};
use std::error::Error;
use std::env;
use std::io::{self, Read};
use sha2::{Sha256, Digest};

// サーバーとクライアント間で送受信するメッセージ構造体。
#[derive(serde::Deserialize, serde::Serialize, Debug)]
enum Message {
    Data(Vec<u8>),
    Hash(String),
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
    let mut server = Server::new("hash_pipe")?;
    println!("Server started, waiting for connections...");

    let client = server.accept()?;
    println!("Client connected");

    // クライアントからデータを受信
    let received_msg: Message = client.recv()?;
    if let Message::Data(data) = received_msg {
        println!("Server received data ({} bytes).", data.len());

        // データのハッシュ値を計算
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let hash = format!("{:x}", hasher.finalize());
        println!("Calculated hash: {}", hash);

        // ハッシュ値をクライアントに送信
        let response = Message::Hash(hash);
        client.send(&response)?;
        println!("Server sent hash.");
    } else {
        return Err("Unexpected message type received by server.".into());
    }

    Ok(())
}

// クライアントモードを実行します。
fn run_client() -> Result<(), Box<dyn Error>> {
    let client = Client::connect("hash_pipe")?;
    println!("Client connected to server");

    // 標準入力からデータを読み込む
    println!("Please enter text to send (end with Ctrl+D or Ctrl+Z):");
    let mut buffer = Vec::new();
    io::stdin().read_to_end(&mut buffer)?;
    
    // サーバーにデータを送信
    let message = Message::Data(buffer);
    client.send(&message)?;
    println!("Client sent data to server.");

    // サーバーからのハッシュ値を受信して表示
    let response: Message = client.recv()?;
    if let Message::Hash(hash) = response {
        println!("Client received hash from server:");
        println!("{}", hash);
    } else {
        return Err("Unexpected message type received by client.".into());
    }

    Ok(())
}