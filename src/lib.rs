#![feature(core)]
#![feature(net)]

extern crate protobuf;

use std::net::TcpStream;

use protobuf::{Message,CodedOutputStream,CodedInputStream};

pub mod proto;

fn create_event() -> proto::Event {
    let mut event = proto::Event::new();
    event.set_service("rust".to_string());
    event.set_state("warn".to_string());
    event.set_host("centurion".to_string());
    return event;
}

fn send_event(event: proto::Event) {
    let mut msg = proto::Msg::new();
    msg.set_events(protobuf::RepeatedField::from_vec(vec![event]));

    let mut stream = TcpStream::connect("127.0.0.1:5555").unwrap();

    {
        println!("--> {{{:?}}}", msg);
        msg.write_length_delimited_to(
            &mut CodedOutputStream::new(&mut stream)).unwrap();
    }

    {
        let response: self::proto::Msg = protobuf::parse_length_delimited_from(
            &mut CodedInputStream::new(&mut stream)).unwrap();
        assert!(response.get_ok());
        println!("<-- {{{:?}}}", response);
    }
}

pub fn event() {
    send_event(create_event());
}
