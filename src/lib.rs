//! A [Riemann](http://riemann.io/) client library

extern crate libc;
#[macro_use]
extern crate log;
extern crate protobuf;

use std::fmt::Debug;
use std::io::Write;
use std::iter::{IntoIterator,FromIterator};
use std::net::{TcpStream,ToSocketAddrs};

use protobuf::{Message,CodedInputStream};

use self::error::{Error,Result};
use self::hostname::hostname;
use self::proto::{Event,Msg,Query};

mod error;
pub mod proto;
pub mod hostname;

pub struct TCPTransport {
    stream: TcpStream
}

impl TCPTransport {
    pub fn connect<A: ToSocketAddrs + ?Sized>(addr: &A) -> Result<Self> {
        Ok(TCPTransport { stream: try!(TcpStream::connect(addr)) })
    }
}

impl TCPTransport {
    fn send_msg(&mut self, msg: Msg) -> Result<Msg> {
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
        let msg: Msg = try!(protobuf::parse_from_bytes(&bytes));

        // If the message has set `ok: false`, transform it into an `Err`
        if msg.get_ok() {
            Ok(msg)
        } else {
            Err(Error::from(msg))
        }
    }

    fn send_events<E>(&mut self, events: E) -> Result<Msg> where E: IntoIterator<Item=Event> {
        self.send_msg({
            let mut msg = proto::Msg::new();
            msg.set_events(protobuf::RepeatedField::from_iter(events));
            msg
        })
    }

    fn send_query(&mut self, query: Query) -> Result<Msg> {
        self.send_msg({
            let mut msg = proto::Msg::new();
            msg.set_query(query);
            msg
        })
    }
}

impl std::fmt::Debug for TCPTransport {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TCPTransport {{ addr: {:?} }}", self.stream.peer_addr())
    }
}

/// A wrapper providing a nicer interface to TCPTransport.
#[derive(Debug)]
pub struct Client {
    transport: TCPTransport
}

impl Client {
    /// Connect to a Riemann server using over TCP.
    pub fn connect<A: ToSocketAddrs + ?Sized>(addr: &A) -> Result<Self> {
        Ok(Client { transport: try!(TCPTransport::connect(addr)) })
    }

    /// Send multiple events, discarding the response if it is not an error.
    pub fn events<E>(&mut self, events: E) -> Result<()> where E: IntoIterator<Item=Event> {
        self.transport.send_events(events).and_then(|_| Ok(()))
    }

    /// Wrapper around `.events()` for sending a single `Event`.
    pub fn event(&mut self, event: Event) -> Result<()> {
        self.events(vec![event])
    }

    /// Send a query and return a sorted list of events matching the query.
    pub fn query(&mut self, query: String) -> Result<Vec<Event>> {
        let response = try!(self.transport.send_query({
            let mut query_msg = Query::new();
            query_msg.set_string(query);
            query_msg
        }));

        Ok({
            let mut events = Vec::from(response.get_events());
            events.sort_by(|a, b| { a.get_service().cmp(b.get_service()) });
            events
        })
    }
}
