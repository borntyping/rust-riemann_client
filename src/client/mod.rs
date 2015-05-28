//! Layer three: An abstract client hiding the TCP/Protobuf layers.

use std::iter::IntoIterator;
use std::net::ToSocketAddrs;

use super::Result;
use super::proto::{Event, Query};
use super::transport::TCPTransport;

// TODO: Enable and use this
// mod hostname;

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
    pub fn events<E>(&mut self, events: E) -> Result<()>
        where E: IntoIterator<Item = Event>
    {
        self.transport.send_events(events).and_then(|_| Ok(()))
    }

    /// Wrapper around `.events()` for sending a single `Event`.
    pub fn event(&mut self, event: Event) -> Result<()> {
        self.events(vec![event])
    }

    /// Send a query and return a sorted list of events matching the query.
    pub fn query<T: Into<Query>>(&mut self, query: T) -> Result<Vec<Event>> {
        let response = try!(self.transport.send_query(query.into()));

        Ok({
            let mut events = Vec::from(response.get_events());
            events.sort_by(|a, b| { a.get_service().cmp(b.get_service()) });
            events
        })
    }
}
