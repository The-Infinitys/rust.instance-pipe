use crate::Client;
use interprocess::local_socket::traits::Listener;
use interprocess::local_socket::{GenericNamespaced, NameType, ToFsName};
use interprocess::local_socket::{ListenerOptions, ToNsName, prelude::LocalSocketListener};
use interprocess::os::unix::local_socket::FilesystemUdSocket;
use std::io::Result;

pub struct Server {
    listener: LocalSocketListener,
}

impl Server {
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

    pub fn accept(&mut self) -> Result<Client> {
        let stream = self.listener.accept()?;
        Ok(stream.into())
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
