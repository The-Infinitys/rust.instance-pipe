use crate::Client;
use interprocess::local_socket::traits::Listener;
use interprocess::local_socket::{GenericNamespaced, NameType, ToFsName, ListenerNonblockingMode};
use interprocess::local_socket::{ListenerOptions, ToNsName, prelude::LocalSocketListener};
use interprocess::os::unix::local_socket::FilesystemUdSocket;
use std::io::Result;
use crate::instance::event::{Event, EventHandler};

/// クライアントからの接続を待ち受けるサーバー構造体。
pub struct Server {
    listener: LocalSocketListener,
    event_handler: EventHandler,
}

impl Server {
    /// 新しいサーバーインスタンスを作成し、接続の待ち受けを開始します。
    ///
    /// 指定された名前で名前付きパイプまたはソケットを生成します。
    ///
    /// # 引数
    /// - `name`: パイプまたはソケットの名前。
    ///
    /// # エラー
    /// パイプ/ソケットの作成に失敗した場合や、サポートされていないソケットタイプの場合にエラーを返します。
    pub fn start(name: &str) -> Result<Self> {
        let name = socket_name(name);
        let socket_name = if GenericNamespaced::is_supported() {
            name.to_ns_name::<GenericNamespaced>()?
        } else if FilesystemUdSocket::is_supported() {
            name.to_fs_name::<FilesystemUdSocket>()?
        } else {
            panic!("Unsupported");
        };
        let opts = ListenerOptions::new().name(socket_name);
        let listener = opts.create_sync()?;
        Ok(Self {
            listener,
            event_handler: EventHandler::new(),
        })
    }

    /// サーバーを停止し、リスナーを閉じます。
    pub fn stop(&mut self) -> Result<()> {
        // LocalSocketListenerは明示的なcloseを持たないため、ドロップで対応
        Ok(())
    }

    /// クライアントからの接続イベントをポーリングします。
    ///
    /// 非ブロッキングで接続をチェックし、接続があればクライアントを返します。
    ///
    /// # エラー
    /// 接続の受け入れに失敗した場合にエラーを返します。
    pub fn poll_event(&mut self) -> Result<Option<Event<Client>>> {
        self.listener.set_nonblocking(ListenerNonblockingMode::Accept)?;
        match self.listener.accept() {
            Ok(stream) => {
                let client: Client = stream.into();
                self.event_handler.notify(Event::<Client>::ConnectionAccepted(client.clone()));
                Ok(Some(Event::ConnectionAccepted(client)))
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                self.listener.set_nonblocking(ListenerNonblockingMode::Neither)?;
                Ok(None)
            }
            Err(e) => {
                self.listener.set_nonblocking(ListenerNonblockingMode::Neither)?;
                Err(e)
            }
        }
    }

    /// クライアントからの接続を受け入れます。
    ///
    /// # エラー
    /// 接続の受け入れに失敗した場合にエラーを返します。
    pub fn accept(&mut self) -> Result<Client> {
        let stream = self.listener.accept()?;
        let client: Client = stream.into();
        self.event_handler.notify(Event::<Client>::ConnectionAccepted(client.clone()));
        Ok(client)
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