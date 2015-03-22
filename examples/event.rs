extern crate riemann_client;
extern crate simple_logger;

fn main() {
    simple_logger::init();

    let mut client = riemann_client::Client::connect(&("localhost", 5555)).unwrap();

    let mut event = riemann_client::proto::Event::new();
    event.set_service("rust-riemann_client".to_string());
    client.send_event(event).unwrap();

    let mut event = riemann_client::proto::Event::new();
    event.set_service("rust-riemann_client".to_string());
    // event.set_metric_sint64(10000);
    event.set_metric_d(10.0);
    client.send_event(event).unwrap();
}
