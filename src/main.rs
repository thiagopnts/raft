#![allow(dead_code)]

extern crate bencode;

use bencode::{Decodable, Decoder, DecodeError};
use std::fs::File;
use std::io::{self, Read, Write};

struct SingleFile {
    length: i64,
    name: String,
    piece_length: usize,
    //pieces: Vec<u8>,
}
impl Decodable for SingleFile {
    fn decode(d: &mut Decoder) -> Result<SingleFile, DecodeError> {
        d.read_struct(|d| {
            Ok(SingleFile{
                length: try!(d.read_field("length")),
                name: try!(d.read_field("name")),
                piece_length: try!(d.read_field("piece length")),
                //pieces: try!(d.read_field("pieces")),
            })
        })
    }
}

struct MetaInfo {
    pub announce: String,
    pub created_by: String,
    pub comment: String,
    pub encoding: String,
    info: SingleFile,
}

fn read_string_with_default(d: &mut Decoder, field_name: &'static str, default: &'static str) -> String {
    match d.read_field(field_name) {
        Ok(field) => field,
        Err(_) => default.to_string(),
    }
}

impl Decodable for MetaInfo {
    fn decode(d: &mut Decoder) -> Result<MetaInfo, DecodeError> {
        d.read_struct(|d| {
            Ok(MetaInfo{
                announce: try!(d.read_field("announce")),
                comment: try!(d.read_field("comment")),
                created_by: read_string_with_default(d, "created by", ""),
                encoding: try!(d.read_field("encoding")),
                info: try!(d.read_field("info")),
            })
        })
    }
}

fn main() {
    let mut file = File::open("harry.torrent").expect("error opening file");
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf);
    let meta_info = bencode::decode::<MetaInfo>(&buf).unwrap();
    println!("GET {}", meta_info.announce);
    println!("name {}", meta_info.info.name);
    println!("total len {}", meta_info.info.length);
    println!("piece len {}", meta_info.info.piece_length);
    println!("created by {}", meta_info.created_by);
    println!("comment {}", meta_info.comment);
    println!("encoding {}", meta_info.encoding);
}
