//! instance-pipe: Rust製プロセス間通信ライブラリ
//!
//! このライブラリは、名前付きパイプまたはソケットを使用して、
//! 複数のプロセス間で構造体や列挙型のメッセージを送受信することを可能にします。

pub mod instance;
pub mod protocol;

pub use instance::client::Client;
pub use instance::server::Server;

// カスタムderiveマクロを再エクスポート
pub use instance_pipe_derive::*;
