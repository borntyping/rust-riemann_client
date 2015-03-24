//! A [Riemann](http://riemann.io/) client library

#![feature(core)]

extern crate libc;
#[macro_use]
extern crate log;
extern crate protobuf;

use std::error::FromError;
use std::io::{BufStream,Write};
use std::io::Error as IoError;
use std::net::{TcpStream,ToSocketAddrs};
use std::net::SocketAddr;

use protobuf::{Message,CodedInputStream};
use protobuf::error::ProtobufError;

use self::proto::{Event,Msg,Query};

pub mod proto;
pub mod utils;

#[derive(Debug)]
pub enum ClientError {
    Io(IoError),
    ProtoBuf(ProtobufError)
}

impl FromError<IoError> for ClientError {
    fn from_error(err: IoError) -> Self {
        ClientError::Io(err)
    }
}

impl FromError<ProtobufError> for ClientError {
    fn from_error(err: ProtobufError) -> Self {
        ClientError::ProtoBuf(err)
    }
}

pub struct Client {
    stream: BufStream<TcpStream>
}

impl Client {
    pub fn connect<A: ToSocketAddrs + ?Sized>(addr: &A) -> Result<Self, ClientError> {
        Ok(Client { stream: BufStream::new(try!(TcpStream::connect(addr))) })
    }

    fn send_msg(&mut self, msg: Msg) -> Result<(), ClientError> {
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
        try!(self.stream.write_all(bytes.as_slice()));
        try!(self.stream.flush());

        return Ok(());
    }

    fn recv_msg(&mut self) -> Result<Msg, ClientError> {
        let mut input_stream = CodedInputStream::new(&mut self.stream);

        // Read the message size as a big-endian 32 bit unsigned integer.
        let mut size: u32 = 0;
        size += (try!(input_stream.read_raw_byte()) as u32) << 24;
        size += (try!(input_stream.read_raw_byte()) as u32) << 16;
        size += (try!(input_stream.read_raw_byte()) as u32) <<  8;
        size += (try!(input_stream.read_raw_byte()) as u32) <<  0;

        // Read the expected bytes and parse them as a message.
        let bytes = try!(input_stream.read_raw_bytes(size));
        let msg = try!(protobuf::parse_from_bytes(bytes.as_slice()));

        return Ok(msg);
    }

    fn send_and_recv_msg(&mut self, msg: Msg) -> Result<Msg, ClientError> {
        try!(self.send_msg(msg));
        return self.recv_msg();
    }

    pub fn send_event(&mut self, event: Event) -> Result<Msg, ClientError> {
        let mut msg = proto::Msg::new();
        msg.set_events(protobuf::RepeatedField::from_vec(vec![event]));
        return self.send_and_recv_msg(msg);
    }

    pub fn send_query(&mut self, query: Query) -> Result<Msg, ClientError> {
        let mut msg = proto::Msg::new();
        msg.set_query(query);
        return self.send_and_recv_msg(msg);
    }
}

impl std::fmt::Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Client {{ addr: {:?} }}", self.stream.get_ref().peer_addr())
    }
}
