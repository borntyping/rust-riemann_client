//! Layer three: An abstract client hiding the TCP/Protobuf layers.

use std::net::ToSocketAddrs;
use std::time::Duration;

use super::proto::{Event, Msg, Query};
use super::transport::TCPTransport;
use super::Result;

/// Adds a `set_defaults()` method to `Event`
trait SetDefaults {
    fn set_defaults(&mut self) -> Result<()>;
}

impl SetDefaults for Event {
    /// Sets a host and service for the event if they are not set
    fn set_defaults(&mut self) -> Result<()> {
        if !self.has_host() {
            self.set_host(gethostname::gethostname().into_string()?);
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
    /// Connect to a Riemann server using raw TCP.
    pub fn connect<A: ToSocketAddrs + ?Sized>(addr: &A) -> Result<Self> {
        Ok(Client {
            transport: TCPTransport::connect(addr)?,
        })
    }

    /// Connect to a Riemann server using mTLS over TCP.
    pub fn connect_tls(
        hostname: &str,
        port: u16,
        ca_file: &str,
        cert_file: &str,
        key_file: &str,
    ) -> Result<Self> {
        Ok(Client {
            transport: TCPTransport::connect_tls(
                hostname.to_owned(),
                port,
                ca_file,
                cert_file,
                key_file,
            )?,
        })
    }

    /// Set a read and write timeout for the underlying socket
    pub fn set_timeout(&mut self, timeout: Option<Duration>) -> Result<()> {
        self.transport.set_timeout(timeout)
    }

    /// Send multiple events, discarding the response if it is not an error.
    pub fn events(&mut self, mut events: Vec<Event>) -> Result<Msg> {
        // Modify each event in the vector in place
        for event in events.iter_mut() {
            event.set_defaults()?;
        }

        // Send all events in the same message
        let msg = self.transport.send_events(events)?;

        // A successful response is discarded as it contains no useful information
        Ok(msg)
    }

    /// Wrapper around `.events()` for sending a single `Event`.
    pub fn event(&mut self, event: Event) -> Result<Msg> {
        self.events(vec![event])
    }

    /// Send a query and return a sorted list of events matching the query.
    pub fn query<T: Into<Query>>(&mut self, query: T) -> Result<Vec<Event>> {
        let response = self.transport.send_query(query.into())?;

        Ok({
            let mut events = response.events;
            events.sort_by(|a, b| a.service.cmp(&b.service));
            events
        })
    }
}

#[cfg(test)]
mod test {
    use super::super::proto::Event;
    use super::SetDefaults;

    #[test]
    fn event_defaults() {
        let mut event = Event::new();
        event.set_defaults().unwrap();

        assert_eq!(event.service, Some("riemann_client".to_string()));
        assert_eq!(
            event.host,
            Some(gethostname::gethostname().into_string().unwrap())
        );
    }

    #[test]
    fn event_no_defaults() {
        let mut event = Event::new();
        event.service = Some("test".to_string());
        event.host = Some("test".to_string());

        assert_eq!(event.service, Some("test".to_string()));
        assert_eq!(event.host, Some("test".to_string()));
    }
}
