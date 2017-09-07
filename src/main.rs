#![allow(dead_code)]

extern crate bencode;

use bencode::{Decodable, Decoder, DecodeError};

enum Info {
    Single,
    Multiple,
}

impl Decodable for Info {
    fn decode(_: &mut Decoder) -> Result<Info, DecodeError> {
        Ok(Info::Single)
    }
}

struct MetaInfo {
    info: Info,
}

impl Decodable for MetaInfo {
    fn decode(d: &mut Decoder) -> Result<MetaInfo, DecodeError> {
        d.read_struct(|d| {
            Ok(MetaInfo{
                info: try!(d.read_field("info"))
            })
        })
    }
}

fn main() {
    println!("hello");
}
