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

    let mut query = proto::Query::new();
    query.set_string("true".to_string());
    msg.set_query(query);

    println!("--> {{{:?}}}", msg);

    let mut stream = TcpStream::connect("127.0.0.1:5555").unwrap();
    let mut stream = BufStream::new(stream);

    {
        let mut output_stream = CodedOutputStream::new(&mut stream);

        // Riemann expects a big-endian 32 bit unsigned integer describing the
        // size of the message, but the `rust-protobuf` library writes a little-
        // endian 32 bit unsigned integer. This writes the size 'correctly'.
        //
        // See also:
        //  - `protobuf::Message::write_length_delimited_to`
        //  - `protobuf::CodedOutputStream::write_raw_varint32`
        let size = msg.compute_size();
        output_stream.write_raw_byte(((size >> 24) & 0xFF) as u8).unwrap();
        output_stream.write_raw_byte(((size >> 16) & 0xFF) as u8).unwrap();
        output_stream.write_raw_byte(((size >>  8) & 0xFF) as u8).unwrap();
        output_stream.write_raw_byte(((size >>  0) & 0xFF) as u8).unwrap();

        // Encode and write the message using protobuf
        msg.write_to_with_cached_sizes(&mut output_stream).unwrap();
    }

    // Flush the buffered stream's output
    stream.flush().unwrap();

    let response: self::proto::Msg = {
        let mut input_stream = CodedInputStream::new(&mut stream);

        // Read a big-endian 32 bit unsigned integer describing the size of the
        // message in bytes.
        let mut size: u32 = 0;
        size += (input_stream.read_raw_byte().unwrap() as u32) << 24;
        size += (input_stream.read_raw_byte().unwrap() as u32) << 16;
        size += (input_stream.read_raw_byte().unwrap() as u32) <<  8;
        size += (input_stream.read_raw_byte().unwrap() as u32) <<  0;

        let bytes = input_stream.read_raw_bytes(size).unwrap();
        protobuf::parse_from_bytes(bytes.as_slice()).unwrap()
    };

    println!("<-- {{{:?}}}", response);
    return response;
}

pub fn event() {
    send_event(create_event());
}
