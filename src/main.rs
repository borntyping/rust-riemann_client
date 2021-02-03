//! A command line interface to [Riemann](http://riemann.io/).
//!
//! Requires the optional dependencies (`docopt` and `rustc_serialize`) that are
//! included by the default feature.

#![cfg(not(test))]
#![cfg(feature = "default")]

use docopt::Docopt;
use serde::Deserialize;

static USAGE: &str = "
Usage: riemann_cli [-HP] send [options]
       riemann_cli [-HP] query <query>
       riemann_cli --help | --version

Server options:
    -H, --server-host <host>    Riemann server hostname [default: localhost].
    -P, --server-port <port>    Riemann server port [default: 5555].

Event options:
    -T, --time <i64>            Event timestamp (unix format).
    -S, --state <str>           Event state.
    -s, --service <str>         Event service [default: riemann-cli].
    -N, --host <str>            Event hostname (defaults to current hostname).
    -d, --description <str>     Event description.
    -t, --tag <str>             Event tags (can be used multiple times).
    -l, --ttl <f32>             Event time to live (in seconds).
    -a, --attribute <str=str>   Event attributes (key=value pairs, can be used multiple times).
    -i, --metric-sint64 <i64>   Event metric as an integer (using the metric_sint64 field).
    -m, --metric-d <f64>        Event metric as a double (using the metric_d field).
    -f, --metric-f <f32>        Event metric as a float (using the metric_f field).
";

#[derive(Deserialize, Debug)]
struct Args {
    flag_server_host: String,
    flag_server_port: u16,

    cmd_send: bool,
    flag_time: Option<i64>,
    flag_state: Option<String>,
    flag_service: Option<String>,
    flag_host: Option<String>,
    flag_description: Option<String>,
    flag_tag: Vec<String>,
    flag_ttl: Option<f32>,
    flag_attribute: Option<String>,
    flag_metric_sint64: Option<i64>,
    flag_metric_d: Option<f64>,
    flag_metric_f: Option<f32>,

    cmd_query: bool,
    arg_query: String,

    flag_version: bool,
}

fn main() {
    use riemann_client::proto::Attribute;

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        println!("riemann_cli v{}", env!("CARGO_PKG_VERSION"));
        return;
    }

    let addr: (&str, u16) = (&args.flag_server_host, args.flag_server_port);
    let mut client = riemann_client::Client::connect(&addr).unwrap();

    if args.cmd_send {
        let mut event = riemann_client::proto::Event::new();

        if let Some(x) = args.flag_time {
            event.set_time(x);
        }
        if let Some(x) = args.flag_state {
            event.set_state(x);
        }
        if let Some(x) = args.flag_service {
            event.set_service(x);
        }
        if let Some(x) = args.flag_host {
            event.set_host(x);
        }
        if let Some(x) = args.flag_description {
            event.set_description(x);
        }
        if let Some(x) = args.flag_ttl {
            event.set_ttl(x);
        }
        if let Some(x) = args.flag_metric_sint64 {
            event.set_metric_sint64(x);
        }
        if let Some(x) = args.flag_metric_d {
            event.set_metric_d(x);
        }
        if let Some(x) = args.flag_metric_f {
            event.set_metric_f(x);
        }

        if !args.flag_tag.is_empty() {
            event.set_tags(protobuf::RepeatedField::from_vec(args.flag_tag));
        }

        if let Some(x) = args.flag_attribute {
            let mut vec_attr: Vec<Attribute> = Vec::new();
            let args_attr = x;
            let args_attr_split = args_attr.split(',');
            for attr in args_attr_split {
                let mut res: Vec<String> = attr.split('=').map(|s| s.to_string()).collect();
                let mut at = Attribute::new();
                if let Some(x) = res.pop() {
                    at.set_value(x)
                };
                if let Some(x) = res.pop() {
                    at.set_key(x)
                };
                vec_attr.push(at);
            }
            event.set_attributes(protobuf::RepeatedField::from_vec(vec_attr));
        }

        println!("--> {{ {:?} }}", event);
        let response = client.event(event).unwrap();
        println!("<-- {{ {:?} }}", response);
    } else if args.cmd_query {
        let events = client.query(args.arg_query).unwrap();

        println!(
            "{:<10} {:<10} {:<55} {:<10} {:<10}",
            "HOSTNAME", "TIME", "SERVICE", "METRIC", "STATE"
        );

        for event in events {
            println!(
                "{:<10} {:<10} {:<55} {:<10} {:<10}",
                event.get_host(),
                event.get_time(),
                event.get_service(),
                event.get_metric_f(),
                event.get_state()
            );
        }
    }
}
