extern crate riemann_client;

use riemann_client::Client;
use riemann_client::proto::Event;

fn main() {
    let mut client = Client::connect(&("localhost", 5555)).unwrap();

    client.event({
        let mut event = Event::new();
        event.set_service("rust-riemann_client".to_string());
        event.set_state("ok".to_string());
        event.set_metric_d(128.128);
        event
    }).unwrap();

    // client.event(riemann_client::Event {
    //     service: "rust-riemann_client",
    //     state: "ok",
    //     metric_d: 128.128
    //     ..Event::new()
    // }).unwrap()
}
