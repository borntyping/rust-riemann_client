//! Layer two: Protobuf transport over TCP.

use std::{
    convert::TryFrom,
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
    sync::Arc,
    time::Duration,
};

use protobuf::{CodedInputStream, Message, MessageField};

use rustls::pki_types::pem::PemObject;
use rustls::pki_types::{CertificateDer, PrivateKeyDer, ServerName};
use rustls::{ClientConfig, ClientConnection, RootCertStore};

use super::proto::{Event, Msg, Query};
use super::utils::{Error, Result};

pub struct TCPTransport {
    stream: TcpStream,
    tls_sess: Option<rustls::ClientConnection>,
}

impl TCPTransport {
    pub fn connect<A: ToSocketAddrs + ?Sized>(addr: &A) -> Result<Self> {
        Ok(TCPTransport {
            stream: TcpStream::connect(addr)?,
            tls_sess: None,
        })
    }

    pub fn connect_tls(
        hostname: String,
        port: u16,
        ca_file: &str,
        cert_file: &str,
        key_file: &str,
    ) -> Result<Self> {
        let addr = (hostname.clone(), port);

        let ca_certs = load_certs(ca_file)?;

        if ca_certs.is_empty() {
            return Err(Error::CACert(format!(
                "Certificate file ({}) probably empty",
                ca_file
            )));
        }

        let mut root_store = RootCertStore::empty();
        let (added, invalid) = root_store.add_parsable_certificates(ca_certs);
        if added == 0 {
            return Err(Error::CACert(format!(
                "No valid CA certificates found in '{}'",
                ca_file
            )));
        }
        if invalid > 0 {
            return Err(Error::CACert(format!(
                "{} certificate(s) in '{}' were ignored as malformed",
                invalid, ca_file
            )));
        }

        let certs = load_certs(cert_file)?;
        let key = load_private_key(key_file)?;

        let config = ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_client_auth_cert(certs, key)?;

        let server_name = ServerName::try_from(hostname)?;

        let sess = ClientConnection::new(Arc::new(config), server_name)?;
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
            msg.events = events;
            msg
        })
    }

    pub fn send_query(&mut self, query: Query) -> Result<Msg> {
        self.send_msg_encryption_wrapper({
            let mut msg = Msg::new();
            msg.query = MessageField::some(query);
            msg
        })
    }
}

impl ::std::fmt::Debug for TCPTransport {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "TCPTransport {{ addr: {:?} }}", self.stream.peer_addr())
    }
}

fn load_certs(filename: &str) -> Result<Vec<CertificateDer<'static>>> {
    CertificateDer::pem_file_iter(filename)
        .map_err(|e| {
            Error::Key(format!(
                "Fail to load client cert file ({}): {}",
                filename, e
            ))
        })?
        .map(|cert| cert.map_err(Error::PemParse))
        .collect::<Result<Vec<_>>>()
}

fn load_private_key(filename: &str) -> Result<PrivateKeyDer<'static>> {
    PrivateKeyDer::from_pem_file(filename)
        .map_err(|e| Error::Key(format!("Fail to load key file ({}): {}", filename, e)))
}

fn send_msg<T: Read + Write>(mut stream: T, msg: Msg) -> Result<Msg> {
    // Prepare the message for writing.
    let size = msg.compute_size();
    let bytes = msg.write_to_bytes()?;

    assert!(
        size == (bytes.len() as u32).into(),
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
    if !msg.has_error() {
        Ok(msg)
    } else {
        Err(Error::Riemann(msg.error().to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rustls;

    #[test]
    fn test_load_valid_cert() {
        let mut root_store = RootCertStore::empty();
        let cert = load_certs("test_certs/ca.crt")
            .unwrap()
            .into_iter()
            .next()
            .unwrap();
        assert!(root_store.add(cert).is_ok());
    }

    #[test]
    #[should_panic(
        expected = "called `Result::unwrap()` on an `Err` value: Key(\"Fail to load client cert file (test_certs/missing_client.crt): I/O error: No such file or directory (os error 2)\")"
    )]
    fn test_load_missing_cert() {
        let mut root_store = RootCertStore::empty();
        let cert = load_certs("test_certs/missing_client.crt")
            .unwrap()
            .into_iter()
            .next()
            .unwrap();
        assert!(root_store.add(cert).is_ok());
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn test_load_empty_cert() {
        let mut root_store = RootCertStore::empty();
        let cert = load_certs("test_certs/empty_client.crt")
            .unwrap()
            .into_iter()
            .next()
            .unwrap();
        assert!(root_store.add(cert).is_ok());
    }

    #[test]
    fn test_load_invalid_cert() {
        let mut root_store = RootCertStore::empty();
        let cert = load_certs("test_certs/invalid_client.crt")
            .unwrap()
            .into_iter()
            .next()
            .unwrap();
        assert!(root_store.add(cert).is_err());
    }

    #[test]
    fn test_load_valid_key() {
        let certs = load_certs("test_certs/client.crt").unwrap();
        let key = load_private_key("test_certs/client.key").unwrap();

        let result = ClientConfig::builder()
            .with_root_certificates(RootCertStore::empty())
            .with_client_auth_cert(certs, key);
        assert!(result.is_ok());
    }

    #[test]
    #[should_panic(
        expected = "called `Result::unwrap()` on an `Err` value: Key(\"Fail to load key file (test_certs/empty_client.key): no items found\")"
    )]
    fn test_load_empty_key() {
        let certs = load_certs("test_certs/client.crt").unwrap();
        let key = load_private_key("test_certs/empty_client.key").unwrap();
        let _ = ClientConfig::builder()
            .with_root_certificates(RootCertStore::empty())
            .with_client_auth_cert(certs, key)
            .unwrap();
    }

    #[test]
    #[should_panic(
        expected = "called `Result::unwrap()` on an `Err` value: General(\"failed to parse private key as RSA, ECDSA, or EdDSA\")"
    )]
    fn test_load_invalid_key() {
        let certs = load_certs("test_certs/client.crt").unwrap();
        let key = load_private_key("test_certs/invalid_client.key").unwrap();
        let _ = ClientConfig::builder()
            .with_root_certificates(rustls::RootCertStore::empty())
            .with_client_auth_cert(certs, key)
            .unwrap();
    }
}
