#![allow(unused_imports)]

extern crate protobuf;
extern crate riemann_client;

use std::net::{TcpListener, TcpStream};
use std::io::{BufReader,BufWriter};
use std::io::BufRead;
use std::thread::Thread;

use protobuf::Message;
use protobuf::CodedInputStream;
use riemann_client::proto::Msg;

fn handle_client(stream: &mut TcpStream) {
    {
        let response: riemann_client::proto::Msg = protobuf::parse_length_delimited_from(
            &mut CodedInputStream::new(stream)).unwrap();
        println!("--> {{{:?}}}", response);
    }

    {
        let mut reply = riemann_client::proto::Msg::new();
        reply.set_ok(true);
        reply.write_length_delimited_to_writer(
            &mut BufWriter::new(stream)).unwrap();
        println!("<-- {{{:?}}}", reply);
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7777").unwrap();

    for stream in listener.incoming() {
        std::thread::spawn(move || {
            handle_client(&mut stream.unwrap())
        });
    }
}
