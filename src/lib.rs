//! instance-pipe: Rust製プロセス間通信ライブラリ
//!
//! このライブラリは、名前付きパイプまたはソケットを使用して、
//! 複数のプロセス間で構造体や列挙型のメッセージを送受信することを可能にします。
//! 主な機能として、サーバーとクライアント間の双方向通信を提供します。

/// クライアントおよびサーバー関連の機能を提供するモジュール。
pub mod instance;
/// メッセージの送受信プロトコルを提供するモジュール。
pub mod protocol;

/// クライアント構造体。サーバーとの通信を管理します。
pub use instance::client::Client;
/// イベント関連の機能を提供します。
pub use instance::event::{Event, EventHandler};
/// サーバー構造体。クライアントからの接続を待ち受けます。
pub use instance::server::Server;
