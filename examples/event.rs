extern crate riemann_client;

fn main() {
    let mut client = riemann_client::Client::connect(&("localhost", 5555)).unwrap();
    let mut event = riemann_client::proto::Event::new();
    event.set_service("rust-riemann_client".to_string());
    event.set_state("ok".to_string());
    event.set_metric_d(128.128);
    client.send_event(event).unwrap();
}
