//! Layer two: Protobuf transport over TCP.

use std::io::Write;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

use ::protobuf::{CodedInputStream, Message};

use super::proto::{Event, Msg, Query};
use super::utils::{Error, Result};

pub struct TCPTransport {
    stream: TcpStream,
}

impl TCPTransport {
    pub fn connect<A: ToSocketAddrs + ?Sized>(addr: &A) -> Result<Self> {
        Ok(TCPTransport {
            stream: TcpStream::connect(addr)?,
        })
    }

    pub fn set_timeout(&mut self, timeout: Option<Duration>) -> Result<()> {
        self.stream.set_write_timeout(timeout)?;
        self.stream.set_read_timeout(timeout)?;
        Ok(())
    }

    pub fn send_msg(&mut self, msg: Msg) -> Result<Msg> {
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
        self.stream.write_all(&[((size >> 24) & 0xFF) as u8])?;
        self.stream.write_all(&[((size >> 16) & 0xFF) as u8])?;
        self.stream.write_all(&[((size >> 8) & 0xFF) as u8])?;
        self.stream.write_all(&[((size) & 0xFF) as u8])?;

        // Write the rest of the message.
        self.stream.write_all(&bytes)?;
        self.stream.flush()?;

        // CodedInputStream is used for the `read_raw_byte(s)` methods
        let mut input_stream = CodedInputStream::new(&mut self.stream);

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

    pub fn send_events(&mut self, events: Vec<Event>) -> Result<Msg> {
        self.send_msg({
            let mut msg = Msg::new();
            msg.set_events(::protobuf::RepeatedField::from_vec(events));
            msg
        })
    }

    pub fn send_query(&mut self, query: Query) -> Result<Msg> {
        self.send_msg({
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
