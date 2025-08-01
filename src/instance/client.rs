use crate::protocol;
use interprocess::local_socket::prelude::LocalSocketStream;
use interprocess::local_socket::traits::Stream;
use interprocess::local_socket::{GenericNamespaced, NameType, ToFsName, ToNsName};
use interprocess::os::unix::local_socket::FilesystemUdSocket;
use serde::{Deserialize, Serialize};
use std::io::{self, Result};
use std::sync::Arc;

pub struct Client {
    stream: Arc<LocalSocketStream>,
}
impl From<LocalSocketStream> for Client {
    fn from(value: LocalSocketStream) -> Self {
        Self {
            stream: Arc::new(value),
        }
    }
}
impl Client {
    /// 指定された名前のサーバーに接続します。
    pub fn connect(name: &str) -> Result<Self> {
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
        })
    }

    /// サーバーにメッセージを送信します。
    pub fn send<T: Serialize>(&self, message: &T) -> Result<()> {
        let stream_clone = self.stream.clone();
        protocol::send_message(&mut &*stream_clone, message)
    }

    /// サーバーからメッセージを受信します。
    pub fn recv<T: for<'a> Deserialize<'a>>(&self) -> Result<T> {
        let stream_clone = self.stream.clone();
        protocol::recv_message(&mut &*stream_clone)
    }
}

#[cfg(target_os = "windows")]
fn socket_name(name: &str) -> String {
    format!(r"\\.\pipe\{}", name)
}

#[cfg(not(target_os = "windows"))]
fn socket_name(name: &str) -> String {
    format!("/tmp/{}", name)
}
