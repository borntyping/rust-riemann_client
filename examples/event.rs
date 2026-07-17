use riemann_client::proto::Event;
use riemann_client::Client;

fn main() {
    let mut client = Client::connect(&("localhost", 5555)).unwrap();

    client
        .event({
            let mut event = Event::new();
            event.service = Some("rust-riemann_client".to_string());
            event.state = Some("ok".to_string());
            event.metric_d = Some(128.128);
            event
        })
        .unwrap();

    // client.event(riemann_client::Event {
    //     service: "rust-riemann_client",
    //     state: "ok",
    //     metric_d: 128.128
    //     ..Event::new()
    // }).unwrap()
}
