//! Layer two: Protobuf transport over TCP.

use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

use ::protobuf::{CodedInputStream, Message};

use super::proto::{Event, Msg, Query};
use super::utils::{Error, Result};

use std::sync::Arc;

use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;

pub struct TCPTransport {
    stream: TcpStream,
    tls_sess: Option<rustls::ClientSession>,
}

impl TCPTransport {
    pub fn connect<A: ToSocketAddrs + ?Sized>(addr: &A) -> Result<Self> {
        Ok(TCPTransport {
            stream: TcpStream::connect(addr)?,
            tls_sess: None,
        })
    }

    pub fn connect_tls(
        hostname: &str,
        port: u16,
        ca_file: &str,
        cert_file: &str,
        key_file: &str,
    ) -> Result<Self> {
        let addr = (hostname, port);
        let mut config = rustls::ClientConfig::new();
        config
            .root_store
            .add(
                load_certs(ca_file)
                    .or_else(|e| match e {
                        Error::Io(e) => Err(Error::CACert(format!(
                            "Fail to load CACert file ({}): {}",
                            ca_file, e
                        ))),
                        _ => unreachable!("CAcert issue"),
                    })?
                    .get(0)
                    .ok_or_else(|| {
                        Error::CACert(format!("Certificate file ({}) probably empty", ca_file))
                    })?,
            )
            .map_err(|e| Error::CACert(format!("Certificate format error: {}", e)))?;

        config.set_single_client_cert(
            load_certs(cert_file).or_else(|e| match e {
                Error::Io(e) => Err(Error::Key(format!(
                    "Fail to load client cert file ({}): {}",
                    cert_file, e
                ))),
                _ => unreachable!("Client cert issue"),
            })?,
            load_private_key(key_file).or_else(|e| match e {
                Error::Io(e) => Err(Error::Key(format!(
                    "Fail to load key file ({}): {}",
                    key_file, e
                ))),
                _ => unreachable!("Key issue"),
            })?,
        )?;

        let dns_name = webpki::DNSNameRef::try_from_ascii_str(hostname)?;
        let sess = rustls::ClientSession::new(&Arc::new(config), dns_name);
        let stream = TcpStream::connect(addr)?;
        Ok(TCPTransport {
            stream,
            tls_sess: Some(sess),
        })
    }

    pub fn set_timeout(&mut self, timeout: Option<Duration>) -> Result<()> {
        self.stream.set_write_timeout(timeout)?;
        self.stream.set_read_timeout(timeout)?;
        Ok(())
    }

    pub fn send_msg_encryption_wrapper(&mut self, msg: Msg) -> Result<Msg> {
        match self.tls_sess.as_mut() {
            Some(_) => send_msg(
                rustls::Stream::new(self.tls_sess.as_mut().unwrap(), &mut self.stream),
                msg,
            ),
            None => send_msg(&mut self.stream, msg),
        }
    }

    pub fn send_events(&mut self, events: Vec<Event>) -> Result<Msg> {
        self.send_msg_encryption_wrapper({
            let mut msg = Msg::new();
            msg.set_events(::protobuf::RepeatedField::from_vec(events));
            msg
        })
    }

    pub fn send_query(&mut self, query: Query) -> Result<Msg> {
        self.send_msg_encryption_wrapper({
            let mut msg = Msg::new();
            msg.set_query(query);
            msg
        })
    }
}

impl ::std::fmt::Debug for TCPTransport {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "TCPTransport {{ addr: {:?} }}", self.stream.peer_addr())
    }
}

fn load_certs(filename: &str) -> Result<Vec<rustls::Certificate>> {
    let certfile = File::open(filename)?;
    let mut reader = BufReader::new(certfile);
    Ok(rustls_pemfile::certs(&mut reader)?
        .iter()
        .map(|v| rustls::Certificate(v.clone()))
        .collect())
}

fn load_private_key(filename: &str) -> Result<rustls::PrivateKey> {
    let keyfile = File::open(filename)?;
    let mut reader = BufReader::new(keyfile);

    loop {
        match rustls_pemfile::read_one(&mut reader)? {
            Some(rustls_pemfile::Item::RSAKey(key)) => return Ok(rustls::PrivateKey(key)),

            Some(rustls_pemfile::Item::PKCS8Key(key)) => return Ok(rustls::PrivateKey(key)),
            None => break,
            _ => {}
        }
    }

    Err(Error::Key("Key not found".to_string()))
}

fn send_msg<T: Read + Write>(mut stream: T, msg: Msg) -> Result<Msg> {
    // Prepare the message for writing.
    let size = msg.compute_size();
    let bytes = msg.write_to_bytes()?;

    assert!(
        size == bytes.len() as u32,
        "Message computed size ({}) and encoded length ({}) do not \
             match, you are going to have a bad day.",
        size,
        bytes.len()
    );

    // Write the message size as a big-endian unsigned integer.
    stream.write_all(&[((size >> 24) & 0xFF) as u8])?;
    stream.write_all(&[((size >> 16) & 0xFF) as u8])?;
    stream.write_all(&[((size >> 8) & 0xFF) as u8])?;
    stream.write_all(&[((size) & 0xFF) as u8])?;

    // Write the rest of the message.
    stream.write_all(&bytes)?;
    stream.flush()?;

    // CodedInputStream is used for the `read_raw_byte(s)` methods
    let mut input_stream = CodedInputStream::new(&mut stream);

    // Read the message size as a big-endian 32 bit unsigned integer.
    let mut size: u32 = 0;
    size += (input_stream.read_raw_byte()? as u32) << 24;
    size += (input_stream.read_raw_byte()? as u32) << 16;
    size += (input_stream.read_raw_byte()? as u32) << 8;
    size += input_stream.read_raw_byte()? as u32;

    // Read the expected bytes and parse them as a message.
    let bytes = input_stream.read_raw_bytes(size)?;
    let msg: Msg = protobuf::Message::parse_from_bytes(&bytes)?;

    // If the message has set `ok: false`, transform it into an `Err`
    if msg.get_ok() {
        Ok(msg)
    } else {
        Err(Error::Riemann(msg.get_error().to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_valid_cert() {
        let mut config = rustls::ClientConfig::new();
        assert_eq!(
            config.root_store.add(
                load_certs("test_certs/valid_cert.pem")
                    .unwrap()
                    .get(0)
                    .unwrap(),
            ),
            Ok(()),
        );
    }

    #[test]
    #[should_panic(
        expected = "called `Result::unwrap()` on an `Err` value: Io(Os { code: 2, kind: NotFound, message: \"No such file or directory\" }"
    )]
    fn test_load_missing_cert() {
        let mut config = rustls::ClientConfig::new();
        assert_eq!(
            config.root_store.add(
                load_certs("test_certs/missing_cert.pem")
                    .unwrap()
                    .get(0)
                    .unwrap(),
            ),
            Ok(()),
        );
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn test_load_empty_cert() {
        let mut config = rustls::ClientConfig::new();
        assert_eq!(
            config.root_store.add(
                load_certs("test_certs/empty_cert.pem")
                    .unwrap()
                    .get(0)
                    .unwrap(),
            ),
            Ok(()),
        );
    }

    #[test]
    fn test_load_invalid_cert() {
        let mut config = rustls::ClientConfig::new();
        assert_eq!(
            config.root_store.add(
                load_certs("test_certs/invalid_cert.pem")
                    .unwrap()
                    .get(0)
                    .unwrap(),
            ),
            Err(webpki::Error::BadDER),
        );
    }

    #[test]
    fn test_load_valid_key() {
        let mut config = rustls::ClientConfig::new();
        assert_eq!(
            config.set_single_client_cert(
                load_certs("test_certs/valid_client_cert.pem").unwrap(),
                load_private_key("test_certs/valid_client_key").unwrap(),
            ),
            Ok(())
        );
    }

    #[test]
    #[should_panic(
        expected = "called `Result::unwrap()` on an `Err` value: Key(\"Key not found\")"
    )]
    fn test_load_empty_key() {
        let mut config = rustls::ClientConfig::new();
        assert_eq!(
            config.set_single_client_cert(
                load_certs("test_certs/valid_client_cert.pem").unwrap(),
                load_private_key("test_certs/empty_client_key").unwrap(),
            ),
            Ok(())
        );
    }

    #[test]
    fn test_load_invalid_key() {
        let mut config = rustls::ClientConfig::new();
        assert_eq!(
            config.set_single_client_cert(
                load_certs("test_certs/valid_client_cert.pem").unwrap(),
                load_private_key("test_certs/invalid_client_key").unwrap(),
            ),
            Err(rustls::TLSError::General("invalid private key".to_string()))
        );
    }
}
