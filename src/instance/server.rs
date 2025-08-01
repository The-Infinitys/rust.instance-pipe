
use crate::Client;
use interprocess::local_socket::traits::Listener;
use interprocess::local_socket::{GenericNamespaced, NameType, ToFsName};
use interprocess::local_socket::{ListenerOptions, ToNsName, prelude::LocalSocketListener};
use interprocess::os::unix::local_socket::FilesystemUdSocket;
use std::io::Result;

/// クライアントからの接続を待ち受けるサーバー構造体。
pub struct Server {
    listener: LocalSocketListener,
}

impl Server {
    /// 新しいサーバーインスタンスを作成します。
    ///
    /// 指定された名前で名前付きパイプまたはソケットを生成します。
    ///
    /// # 引数
    /// - `name`: パイプまたはソケットの名前。
    ///
    /// # エラー
    /// パイプ/ソケットの作成に失敗した場合や、サポートされていないソケットタイプの場合にエラーを返します。
    pub fn new(name: &str) -> Result<Self> {
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
        Ok(Self { listener })
    }

    /// クライアントからの接続を受け入れます。
    ///
    /// # エラー
    /// 接続の受け入れに失敗した場合にエラーを返します。
    pub fn accept(&mut self) -> Result<Client> {
        let stream = self.listener.accept()?;
        Ok(stream.into())
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