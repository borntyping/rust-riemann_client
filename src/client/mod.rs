//! Layer three: An abstract client hiding the TCP/Protobuf layers.

use std::net::ToSocketAddrs;
use std::time::Duration;

use super::proto::{Event, Query};
use super::transport::TCPTransport;
use super::Result;

mod hostname;

/// Adds a `set_defaults()` method to `Event`
trait SetDefaults {
    fn set_defaults(&mut self) -> Result<()>;
}

impl SetDefaults for Event {
    /// Sets a host and service for the event if they are not set
    fn set_defaults(&mut self) -> Result<()> {
        if !self.has_host() {
            self.set_host(hostname::hostname()?)
        }
        if !self.has_service() {
            self.set_service("riemann_client".to_string())
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Client {
    pub transport: TCPTransport,
}

impl Client {
    /// Connect to a Riemann server using over TCP.
    pub fn connect<A: ToSocketAddrs + ?Sized>(addr: &A) -> Result<Self> {
        Ok(Client {
            transport: TCPTransport::connect(addr)?,
        })
    }

    /// Set a read and write timeout for the underlying socket
    pub fn set_timeout(&mut self, timeout: Option<Duration>) -> Result<()> {
        self.transport.set_timeout(timeout)
    }

    /// Send multiple events, discarding the response if it is not an error.
    pub fn events(&mut self, mut events: Vec<Event>) -> Result<()> {
        // Modify each event in the vector in place
        for event in events.iter_mut() {
            event.set_defaults()?;
        }

        // Send all events in the same message
        self.transport.send_events(events)?;

        // A successful response is discarded as it contains no useful information
        Ok(())
    }

    /// Wrapper around `.events()` for sending a single `Event`.
    pub fn event(&mut self, event: Event) -> Result<()> {
        self.events(vec![event])
    }

    /// Send a query and return a sorted list of events matching the query.
    pub fn query<T: Into<Query>>(&mut self, query: T) -> Result<Vec<Event>> {
        let response = self.transport.send_query(query.into())?;

        Ok({
            let mut events = Vec::from(response.get_events());
            events.sort_by(|a, b| a.get_service().cmp(b.get_service()));
            events
        })
    }
}

#[cfg(test)]
mod test {
    use super::super::proto::Event;
    use super::hostname::hostname;
    use super::SetDefaults;

    #[test]
    fn event_defaults() {
        let mut event = Event::new();
        event.set_defaults().unwrap();

        assert_eq!(event.get_service(), "riemann_client".to_string());
        assert_eq!(event.get_host(), hostname().unwrap());
    }

    #[test]
    fn event_no_defaults() {
        let mut event = Event::new();
        event.set_service("test".to_string());
        event.set_host("test".to_string());
        event.set_defaults().unwrap();

        assert_eq!(event.get_service(), "test".to_string());
        assert_eq!(event.get_host(), "test".to_string());
    }
}
