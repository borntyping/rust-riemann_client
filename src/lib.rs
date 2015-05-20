//! A [Riemann](http://riemann.io/) client library

extern crate libc;
#[macro_use]
extern crate log;
extern crate protobuf;

use std::io::Write;
use std::net::{TcpStream,ToSocketAddrs};

use protobuf::{Message,CodedInputStream};
use protobuf::error::ProtobufError;

use self::proto::{Event,Msg,Query};

pub mod proto;
pub mod utils;

#[derive(Debug)]
pub enum Error {
    Io(::std::io::Error),
    Protobuf(ProtobufError),
    Riemann(String)
}

impl From<::std::io::Error> for Error {
    fn from(err: ::std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<ProtobufError> for Error {
    fn from(err: ProtobufError) -> Self {
        Error::Protobuf(err)
    }
}

impl From<Msg> for Error {
    fn from(msg: Msg) -> Self {
        Error::Riemann(msg.get_error().to_string())
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;

pub struct Client {
    stream: TcpStream
}

impl Client {
    pub fn connect<A: ToSocketAddrs + ?Sized>(addr: &A) -> Result<Self> {
        Ok(Client { stream: try!(TcpStream::connect(addr)) })
    }

    fn send_msg(&mut self, msg: Msg) -> Result<()> {
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

        return Ok(());
    }

    fn recv_msg(&mut self) -> Result<Msg> {
        let mut input_stream = CodedInputStream::new(&mut self.stream);

        // Read the message size as a big-endian 32 bit unsigned integer.
        let mut size: u32 = 0;
        size += (try!(input_stream.read_raw_byte()) as u32) << 24;
        size += (try!(input_stream.read_raw_byte()) as u32) << 16;
        size += (try!(input_stream.read_raw_byte()) as u32) <<  8;
        size += (try!(input_stream.read_raw_byte()) as u32) <<  0;

        // Read the expected bytes and parse them as a message.
        let bytes = try!(input_stream.read_raw_bytes(size));
        let msg: Msg = try!(protobuf::parse_from_bytes(&bytes));

        // Check that the messages has set `ok: true`
        if msg.get_ok() { Ok(msg) } else { Err(Error::from(msg)) }
    }

    fn send_and_recv_msg(&mut self, msg: Msg) -> Result<Msg> {
        try!(self.send_msg(msg));
        return self.recv_msg();
    }

    pub fn send_event(&mut self, event: Event) -> Result<Msg> {
        let mut msg = proto::Msg::new();
        msg.set_events(protobuf::RepeatedField::from_vec(vec![event]));
        return self.send_and_recv_msg(msg);
    }

    pub fn send_query(&mut self, query: Query) -> Result<Msg> {
        let mut msg = proto::Msg::new();
        msg.set_query(query);
        return self.send_and_recv_msg(msg);
    }
}

impl std::fmt::Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Client {{ addr: {:?} }}", self.stream.peer_addr())
    }
}
