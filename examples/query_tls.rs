//! Prints all events from the index

use riemann_client::Client;

fn main() {
    let mut client = Client::connect_tls(
        "localhost",
        5554,
        "./test_certs/ca.crt",
        "./test_certs/client.crt",
        "./test_certs/client.key",
    )
    .unwrap();

    let events = client.query("true").unwrap();

    println!(
        "{:<10} {:<10} {:<55} {:<10} {:<10}",
        "HOSTNAME", "TIME", "SERVICE", "METRIC", "STATE"
    );

    for event in events {
        println!(
            "{:<10} {:<10} {:<55} {:<10} {:<10}",
            event.host(),
            event.time(),
            event.service(),
            event.metric_f(),
            event.state()
        );
    }
}
