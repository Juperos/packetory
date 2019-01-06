#![allow(non_camel_case_types)]

#[macro_use]
extern crate serde_derive;

use bincode::deserialize;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Read;
use std::net::TcpListener;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct PACKET_CA_REQ_HASH {
    header: [u8; 2],
}

#[derive(Debug)]
enum Packets {
    PACKET_CA_REQ_HASH = 0x1db,
    PACKET_UNKNOWN = 0xffff,
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6900").unwrap();

    println!("Listening on 6900!");

    for stream in listener.incoming() {
        match stream {
            Ok(mut packet) => {
                packet.set_nodelay(true).expect("Failed to set_nodelay");

                // TODO: Generics... (and use it instead of byteorder?)
                // Peek here because we don't want to remove it from the queue.
                let mut buf = [0; 2];
                packet.peek(&mut buf).expect("Failed to peek");
                let decoded: PACKET_CA_REQ_HASH = deserialize(&buf[..]).unwrap();

                println!("{:?}", decoded);

                // OR use this
                let header: u16 = packet.read_u16::<LittleEndian>().unwrap_or(0);

                println!("Packet Header => 0x{:x?}", header);

                let packet_type = match header {
                    0x1db => Packets::PACKET_CA_REQ_HASH,
                    _ => Packets::PACKET_UNKNOWN,
                };

                println!("Packet Type => {:?}", packet_type);

                // TODO: Match on packet struct from enum(?)
            }
            Err(e) => panic!("encountered IO error: {}", e),
        }
    }
}
