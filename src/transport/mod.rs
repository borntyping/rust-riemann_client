//! Layer two: Protobuf transport over TCP.

use std::io::Write;
use std::iter::{IntoIterator, FromIterator};
use std::net::{TcpStream, ToSocketAddrs};

use ::protobuf::{Message, CodedInputStream, parse_from_bytes};

use super::utils::{Error, Result};
use super::proto::{Event, Msg, Query};

pub struct TCPTransport {
    stream: TcpStream
}

impl TCPTransport {
    pub fn connect<A: ToSocketAddrs + ?Sized>(addr: &A) -> Result<Self> {
        Ok(TCPTransport { stream: try!(TcpStream::connect(addr)) })
    }

    pub fn send_msg(&mut self, msg: Msg) -> Result<Msg> {
        // Prepare the message for writing.
        let size = msg.compute_size();
        let bytes = try!(msg.write_to_bytes());

        assert!(size == bytes.len() as u32,
            "Message computed size ({}) and encoded length ({}) do not \
             match, you are going to have a bad day.", size, bytes.len());

        // Write the message size as a big-endian unsigned integer.
        try!(self.stream.write_all(&[((size >> 24) & 0xFF) as u8]));
        try!(self.stream.write_all(&[((size >> 16) & 0xFF) as u8]));
        try!(self.stream.write_all(&[((size >>  8) & 0xFF) as u8]));
        try!(self.stream.write_all(&[((size >>  0) & 0xFF) as u8]));

        // Write the rest of the message.
        try!(self.stream.write_all(&bytes));
        try!(self.stream.flush());

        // CodedInputStream is used for the `read_raw_byte(s)` methods
        let mut input_stream = CodedInputStream::new(&mut self.stream);

        // Read the message size as a big-endian 32 bit unsigned integer.
        let mut size: u32 = 0;
        size += (try!(input_stream.read_raw_byte()) as u32) << 24;
        size += (try!(input_stream.read_raw_byte()) as u32) << 16;
        size += (try!(input_stream.read_raw_byte()) as u32) <<  8;
        size += (try!(input_stream.read_raw_byte()) as u32) <<  0;

        // Read the expected bytes and parse them as a message.
        let bytes = try!(input_stream.read_raw_bytes(size));
        let msg: Msg = try!(parse_from_bytes(&bytes));

        // If the message has set `ok: false`, transform it into an `Err`
        if msg.get_ok() {
            Ok(msg)
        } else {
            Err(Error::Riemann(msg.get_error().to_string()))
        }
    }

    pub fn send_events<E>(&mut self, events: E) -> Result<Msg>
        where E: IntoIterator<Item = Event>
    {
        self.send_msg({
            let mut msg = Msg::new();
            msg.set_events(::protobuf::RepeatedField::from_iter(events));
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
