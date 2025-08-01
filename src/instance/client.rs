use crate::instance::event::{Event, EventHandler};
use crate::protocol;
use interprocess::local_socket::prelude::*;
use interprocess::local_socket::traits::Stream;
use interprocess::local_socket::{GenericNamespaced, NameType, ToFsName, ToNsName};
use interprocess::os::unix::local_socket::FilesystemUdSocket;
use serde::{Deserialize, Serialize};
use std::io::{self, Result};
use std::sync::Arc;

/// サーバーに接続するためのクライアント構造体。
#[derive(Clone)]
pub struct Client {
    stream: Arc<LocalSocketStream>,
    event_handler: EventHandler,
}

impl From<LocalSocketStream> for Client {
    /// `LocalSocketStream`から`Client`を生成します。
    fn from(value: LocalSocketStream) -> Self {
        Self {
            stream: Arc::new(value),
            event_handler: EventHandler::new(),
        }
    }
}

impl Client {
    /// 指定された名前のサーバーに接続を開始します。
    ///
    /// 名前付きパイプまたはソケットを使用して接続を確立します。
    ///
    /// # 引数
    /// - `name`: 接続するサーバーのパイプまたはソケット名。
    ///
    /// # エラー
    /// 接続に失敗した場合や、サポートされていないソケットタイプの場合にエラーを返します。
    pub fn start(name: &str) -> Result<Self> {
        let name = socket_name(name);
        let socket_name = if GenericNamespaced::is_supported() {
            name.to_ns_name::<GenericNamespaced>()?
        } else if FilesystemUdSocket::is_supported() {
            name.to_fs_name::<FilesystemUdSocket>()?
        } else {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "Neither namespaced nor filesystem-based sockets are supported",
            ));
        };
        let stream = LocalSocketStream::connect(socket_name)?;
        Ok(Self {
            stream: Arc::new(stream),
            event_handler: EventHandler::new(),
        })
    }

    /// クライアントを停止し、接続を閉じます。
    pub fn stop(&mut self) -> Result<()> {
        // LocalSocketStreamは明示的なshutdownを持たないため、ドロップで対応
        Ok(())
    }

    /// サーバーからのイベントをポーリングします。
    ///
    /// 非ブロッキングでメッセージを受信し、イベントとして返します。
    ///
    /// # エラー
    /// メッセージの受信またはデシリアライズに失敗した場合にエラーを返します。
    pub fn poll_event<T: for<'a> Deserialize<'a>>(&mut self) -> Result<Option<Event<T>>> {
        self.stream.set_nonblocking(true)?;
        match protocol::recv_message(&mut &*self.stream) {
            Ok(message) => Ok(Some(Event::MessageReceived(message))),
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                self.stream.set_nonblocking(false)?;
                Ok(None)
            }
            Err(e) => {
                self.stream.set_nonblocking(false)?;
                Err(e)
            }
        }
    }

    /// サーバーにメッセージを送信します。
    ///
    /// # 引数
    /// - `message`: 送信するメッセージ。`Serialize`トレイトを実装している必要があります。
    ///
    /// # エラー
    /// メッセージのシリアライズまたは送信に失敗した場合にエラーを返します。
    pub fn send<T: Serialize>(&self, message: &T) -> Result<()> {
        let stream_clone = self.stream.clone();
        protocol::send_message(&mut &*stream_clone, message)?;
        self.event_handler.notify(Event::<()>::MessageSent);
        Ok(())
    }

    /// サーバーからメッセージを受信します。
    ///
    /// # エラー
    /// メッセージの受信またはデシリアライズに失敗した場合にエラーを返します。
    pub fn recv<T: for<'a> Deserialize<'a> + Clone>(&self) -> Result<T> {
        let stream_clone = self.stream.clone();
        let message: T = protocol::recv_message(&mut &*stream_clone)?;
        self.event_handler
            .notify(Event::MessageReceived(message.clone()));
        Ok(message)
    }
}

/// プラットフォームに応じたソケット名を生成します（Windows用）。
#[cfg(target_os = "windows")]
fn socket_name(name: &str) -> String {
    format!(r"\\.\pipe\{}", name)
}

/// プラットフォームに応じたソケット名を生成します（非Windows用）。
#[cfg(not(target_os = "windows"))]
fn socket_name(name: &str) -> String {
    format!("/tmp/{}", name)
}
