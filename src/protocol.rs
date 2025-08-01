
use serde::{de::DeserializeOwned, Serialize};
use std::io::{self, Read, Write};

/// メッセージをシリアライズして指定されたライターに送信します。
///
/// メッセージをbincode形式でエンコードし、長さプレフィックス付きで送信します。
///
/// # 引数
/// - `writer`: メッセージの書き込み先となる`Write`トレイトを実装するオブジェクト。
/// - `message`: 送信するメッセージ。`Serialize`トレイトを実装している必要があります。
///
/// # エラー
/// I/Oエラーまたはシリアライズエラーが発生した場合に`io::Result`を返します。
pub fn send_message<T: Serialize, W: Write>(writer: &mut W, message: &T) -> io::Result<()> {
    // bincode v2 を使ってメッセージをバイナリにシリアライズ
    let encoded = bincode::serde::encode_to_vec(message, bincode::config::standard())
        .map_err(io::Error::other)?;

    // メッセージの長さをリトルエンディアンで4バイトのプレフィックスとして書き込む
    let len = encoded.len() as u32;
    writer.write_all(&len.to_le_bytes())?;
    
    // メッセージデータを書き込む
    writer.write_all(&encoded)?;
    writer.flush()?;
    Ok(())
}

/// 指定されたリーダーからメッセージを受信し、デシリアライズします。
///
/// 長さプレフィックスを読み取り、bincode形式でデコードします。
///
/// # 引数
/// - `reader`: メッセージの読み込み元となる`Read`トレイトを実装するオブジェクト。
///
/// # エラー
/// I/Oエラーまたはデシリアライズエラーが発生した場合に`io::Result`を返します。
pub fn recv_message<T: DeserializeOwned, R: Read>(reader: &mut R) -> io::Result<T> {
    let mut len_bytes = [0u8; 4];
    reader.read_exact(&mut len_bytes)?;
    
    let len = u32::from_le_bytes(len_bytes) as usize;
    let mut encoded = vec![0u8; len];
    reader.read_exact(&mut encoded)?;

    // bincode v2 を使ってバイナリをメッセージにデシリアライズ
    let (message, _) = bincode::serde::decode_from_slice(&encoded, bincode::config::standard())
        .map_err(io::Error::other)?;
    
    Ok(message)
}