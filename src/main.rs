//! A command line interface to [Riemann](http://riemann.io/).
//!
//! Requires the optional dependencies (`docopt` and `rustc_serialize`) that are
//! included by the default feature.

#![cfg(not(test))]
#![cfg(feature = "default")]

extern crate docopt;
extern crate protobuf;
extern crate riemann_client;
extern crate rustc_serialize;

static USAGE: &'static str = "
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
    -N, --host <str>            Event hostname.
    -d, --description <str>     Event description.
    -t, --tag <str>             Event tags (can be used multiple times).
    -l, --ttl <f32>             Event time to live (seconds).
    -a, --attribute <str=str>   Event attributes (key=value, multiple times).
    -i, --metric-sint64 <i64>   Event metric as an integer (metric_sint64).
    -m, --metric-d <f64>        Event metric as a double (metric_d).
    -f, --metric-f <f32>        Event metric as a float (metric_f).
";

#[derive(RustcDecodable, Debug)]
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
    flag_attribute: Vec<String>,
    flag_metric_sint64: Option<i64>,
    flag_metric_d: Option<f64>,
    flag_metric_f: Option<f32>,

    cmd_query: bool,
    arg_query: String,

    flag_version: bool,
}

fn main() {
    let args: Args = docopt::Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        println!("riemann_cli v{}", env!("CARGO_PKG_VERSION"));
        return;
    }

    let addr: (&str, u16) = (&args.flag_server_host, args.flag_server_port);
    let mut client = riemann_client::Client::connect(&addr).unwrap();

    if args.cmd_send {
        let mut event = riemann_client::proto::Event::new();

        if let Some(x) = args.flag_time { event.set_time(x); }
        if let Some(x) = args.flag_state { event.set_state(x); }
        if let Some(x) = args.flag_service { event.set_service(x); }
        if let Some(x) = args.flag_host { event.set_host(x); }
        if let Some(x) = args.flag_description { event.set_description(x); }
        if let Some(x) = args.flag_ttl { event.set_ttl(x); }
        if let Some(x) = args.flag_metric_sint64 { event.set_metric_sint64(x); }
        if let Some(x) = args.flag_metric_d { event.set_metric_d(x); }
        if let Some(x) = args.flag_metric_f { event.set_metric_f(x); }

        if !args.flag_tag.is_empty() {
            event.set_tags(protobuf::RepeatedField::from_vec(args.flag_tag));
        }

        if !args.flag_attribute.is_empty() {
            unimplemented!();
        }

        println!("--> {{ {:?} }}", event);
        let response = client.event(event).unwrap();
        println!("<-- {{ {:?} }}", response);
    } else if args.cmd_query {
        let events = client.query(args.arg_query).unwrap();

        println!(
            "{:<10} {:<10} {:<55} {:<10} {:<10}",
            "HOSTNAME", "TIME", "SERVICE", "METRIC", "STATE");

        for event in events {
            println!("{:<10} {:<10} {:<55} {:<10} {:<10}",
                event.get_host(),
                event.get_time(),
                event.get_service(),
                event.get_metric_f(),
                event.get_state()
            );
        }
    }
}
