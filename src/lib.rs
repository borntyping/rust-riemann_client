#![feature(core)]
#![feature(io)]
#![feature(net)]
#![allow(unused_imports)]
#![allow(unused_mut)]

extern crate protobuf;

use std::net::TcpStream;
use std::io::{BufStream,Write};

use protobuf::{Message,CodedOutputStream,CodedInputStream};

pub mod proto;

fn create_event() -> proto::Event {
    let mut event = proto::Event::new();
    event.set_service("rust".to_string());
    event.set_state("warn".to_string());
    event.set_host("centurion".to_string());
    return event;
}

fn send_event(event: proto::Event) -> proto::Msg {
    let mut msg = proto::Msg::new();
    msg.set_events(protobuf::RepeatedField::from_vec(vec![event]));

    let mut stream = TcpStream::connect("127.0.0.1:5555").unwrap();
    let mut stream = BufStream::new(stream);

    {
        println!("--> {{{:?}}}", msg);
        let mut output_stream = CodedOutputStream::new(&mut stream);

        // Riemann expects a big-endian 32 bit unsigned integer describing the
        // size of the message, but the `rust-protobuf` library writes a little-
        // endian 64 bit unsigned integer. This writes the size 'correctly'.
        //
        // See also:
        //  - `protobuf::Message::write_length_delimited_to`
        //  - `protobuf::CodedOutputStream::write_raw_varint32`
        let size = msg.compute_size();
        output_stream.write_raw_byte(((size >> 24) & 0xFF) as u8).unwrap();
        output_stream.write_raw_byte(((size >> 16) & 0xFF) as u8).unwrap();
        output_stream.write_raw_byte(((size >>  8) & 0xFF) as u8).unwrap();
        output_stream.write_raw_byte(((size      ) & 0xFF) as u8).unwrap();

        // Encode and write the message using protobuf
        msg.write_to_with_cached_sizes(&mut output_stream).unwrap();
    }

    stream.flush().unwrap();

    {
        let response: self::proto::Msg = protobuf::parse_length_delimited_from(
            &mut CodedInputStream::new(&mut stream)).unwrap();
        println!("<-- {{{:?}}}", response);
        return response;
    }
}

pub fn event() {
    send_event(create_event());
}
