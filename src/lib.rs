//! A [Riemann](http://riemann.io/) client library

#![feature(core)]
#![feature(io)]
#![feature(net)]

#[macro_use]
extern crate log;
extern crate protobuf;

use std::error::FromError;
use std::io::{BufStream,Write};
use std::io::Error as IoError;
use std::net::{TcpStream,ToSocketAddrs};
use std::net::SocketAddr;

use protobuf::{Message,CodedOutputStream,CodedInputStream};
use protobuf::error::ProtobufError;

use self::proto::{Event,Msg,Query};

pub mod proto;

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

    /// This function might panic, but probably will not
    fn addr(&self) -> SocketAddr {
        self.stream.get_ref().peer_addr().unwrap()
    }

    fn send_msg(&mut self, msg: Msg) -> Result<(), ClientError> {
        debug!("{:?} <-- msg {{{:?}}}", self.addr(), msg);

        {
            let mut output_stream = CodedOutputStream::new(&mut self.stream);

            // Riemann expects a big-endian 32 bit unsigned integer describing
            // the size of the message, but the `rust-protobuf` library writes a
            // little-endian 32 bit unsigned integer.
            //
            // See also:
            //  - `protobuf::Message::write_length_delimited_to`
            //  - `protobuf::CodedOutputStream::write_raw_varint32`
            let size = msg.compute_size();
            try!(output_stream.write_raw_byte(((size >> 24) & 0xFF) as u8));
            try!(output_stream.write_raw_byte(((size >> 16) & 0xFF) as u8));
            try!(output_stream.write_raw_byte(((size >>  8) & 0xFF) as u8));
            try!(output_stream.write_raw_byte(((size >>  0) & 0xFF) as u8));

            // Encode and write the message using protobuf
            try!(msg.write_to_with_cached_sizes(&mut output_stream));
        }

        try!(self.stream.flush());
        return Ok(());
    }

    fn recv_msg(&mut self) -> Result<Msg, ClientError> {
        let msg = {
            let mut input_stream = CodedInputStream::new(&mut self.stream);

            // Read a big-endian 32 bit unsigned integer describing the size of the
            // message in bytes. Further explanation the comments in `send_msg`.
            let mut size: u32 = 0;
            size += (try!(input_stream.read_raw_byte()) as u32) << 24;
            size += (try!(input_stream.read_raw_byte()) as u32) << 16;
            size += (try!(input_stream.read_raw_byte()) as u32) <<  8;
            size += (try!(input_stream.read_raw_byte()) as u32) <<  0;

            // Read the expected bytes and parse them as a message
            let bytes = try!(input_stream.read_raw_bytes(size));
            try!(protobuf::parse_from_bytes(bytes.as_slice()))
        };

        debug!("{:?} --> msg {{{:?}}}", self.addr(), msg);
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
        write!(f, "Client {{ addr: {:?} }}", self.addr())
    }
}
