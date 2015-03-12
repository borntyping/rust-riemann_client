//! Prints all events from the index

#![feature(collections)]

extern crate riemann_client;

fn main() {
    let addr = ("localhost", 5555);
    let mut client = riemann_client::Client::connect(&addr).unwrap();
    let mut query = riemann_client::proto::Query::new();
    query.set_string("true".to_string());

    let response = client.send_query(query).unwrap();

    // Create a sorted list of events
    let mut events = Vec::new();
    events.push_all(response.get_events());
    events.sort_by(|a, b| {
        a.get_service().cmp(b.get_service())
    });

    for event in events {
        println!("{} - {} - {:<55} {:<10} {:<10}",
            event.get_host(),
            event.get_time(),
            event.get_service(),
            event.get_metric_f(),
            event.get_state()
        );
    }
}
