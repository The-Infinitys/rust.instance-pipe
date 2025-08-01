
use instance_pipe::{Client, Server};
use std::error::Error;
use std::env;

/// テスト用のメッセージ構造体。
#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct TestMessage {
    id: u32,
    content: String,
}

/// プログラムのエントリーポイント。
///
/// コマンドライン引数に基づいてサーバーまたはクライアントを実行します。
/// 使用方法: `[実行ファイル] [server|client]`
///
/// # エラー
/// 引数が無効な場合や、サーバー/クライアントの実行中にエラーが発生した場合にエラーを返します。
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

/// サーバーモードを実行します。
///
/// 名前付きパイプを作成し、クライアントからの接続を待機します。
/// 接続後、メッセージを受信し、応答を送信します。
///
/// # エラー
/// パイプの作成、接続の受け入れ、メッセージの送受信でエラーが発生した場合にエラーを返します。
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

/// クライアントモードを実行します。
///
/// 指定された名前付きパイプに接続し、メッセージを送信して応答を受信します。
///
/// # エラー
/// サーバーへの接続、メッセージの送受信でエラーが発生した場合にエラーを返します。
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