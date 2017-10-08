#![allow(dead_code)]

extern crate bencode;

use bencode::{FromBencode, Bencode, NumFromBencodeError, StringFromBencodeError};
use bencode::util::ByteString;
use std::fs::File;
use std::io::{self, Read, Write};


#[derive(Debug)]
enum DecodingError {
    MissingField(String),
    NotADict,
    NotANumber(NumFromBencodeError),
    NotAString(StringFromBencodeError)
}

struct FileDesc {
    length: i64,
    md5sum: Option<String>,
    path: Vec<String>,
}

struct MultiFile {
    files: Vec<FileDesc>,
}

struct SingleFile {
    length: i64,
    md5sum: Option<String>,
}

enum Mode {
    Single(SingleFile),
    Multiple(MultiFile),
}

struct Info {
    mode: Mode,
    name: String,
    piece_length: i64,
    pieces: Vec<u8>,
    private: Option<u8>,
}

impl FromBencode for Info {
    type Err = DecodingError;
    fn from_bencode(bencode: &Bencode) -> Result<Info, DecodingError> {
        use DecodingError::*;
        match bencode {
            &Bencode::Dict(ref m) => {
                let mut private = None;
                let length = m.get(&ByteString::from_str("piece length")).expect("piece length not defined");
                let mut value = m.get(&ByteString::from_str("name")).expect("file name not defined");
                let name = FromBencode::from_bencode(value).ok().expect("error decoding name");
                value =  m.get(&ByteString::from_str("pieces")).expect("pieces not defined");
                match value {
                    &Bencode::ByteString(ref vec) =>  {
                        if let Some(value) = m.get(&ByteString::from_str("private")) {
                            private = FromBencode::from_bencode(value).ok();
                        }
                        // if it has length it means its single file
                        if let Some(value) = m.get(&ByteString::from_str("length")) {
                            let mut md5sum = None;
                            let length = FromBencode::from_bencode(value).ok().expect("file size not defined");
                            if let Some(value) = m.get(&ByteString::from_str("md5sum")) {
                                md5sum = FromBencode::from_bencode(value).ok();
                            }

                            let mode = Mode::Single(SingleFile {
                                length: length,
                                md5sum: md5sum,
                            });
                            return Ok(Info {
                                mode: mode,
                                name: name,
                                piece_length: length,
                                private: private,
                                pieces: vec.clone(),
                            });
                        }
                    },
                    _ => panic!("pieces not a byte string"),
                }
                Err(MissingField("multifile support not implemented".to_string()))
            },
            _ => Err(NotADict),
        }
    }
}

struct MetaInfo {
    info: Info,
    announce: String,
    created_by: Option<String>,
    creation_date: Option<u64>,
    comment: Option<String>,
    encoding: Option<String>,
}

impl FromBencode for MetaInfo {
    type Err = DecodingError;
    fn from_bencode(bencode: &Bencode) -> Result<MetaInfo, DecodingError> {
        use DecodingError::*;
        match bencode {
            &Bencode::Dict(ref m) => {
                let mut announce_option = None;
                let mut created_by = None;
                let mut creation_date = None;
                let mut comment = None;
                let mut encoding = None;
                if let Some(value) = m.get(&ByteString::from_str("announce")) {
                    announce_option = FromBencode::from_bencode(value).ok();
                }
                if let Some(value) = m.get(&ByteString::from_str("created by")) {
                    created_by = FromBencode::from_bencode(value).ok();
                }
                if let Some(value) = m.get(&ByteString::from_str("creation date")) {
                    creation_date = FromBencode::from_bencode(value).ok();
                }
                if let Some(value) = m.get(&ByteString::from_str("comment")) {
                    comment = FromBencode::from_bencode(value).ok();
                }
                if let Some(value) = m.get(&ByteString::from_str("encoding")) {
                    encoding = FromBencode::from_bencode(value).ok();
                }
                if announce_option.is_none() {
                    return Err(MissingField("missing announce field".to_string()))
                }
                let info_dict = m.get(&ByteString::from_str("info")).expect("info field not found");
                let info = Info::from_bencode(info_dict).unwrap();
                Ok(MetaInfo{
                    info: info,
                    announce: announce_option.expect("no announce url found"),
                    created_by: created_by,
                    creation_date: creation_date,
                    comment: comment,
                    encoding: encoding,
                })
            },
            _ => Err(NotADict),
        }
    }
}

fn main() {
    let mut file = File::open("harry.torrent").expect("error opening file");
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf);
    let bencode: bencode::Bencode = bencode::from_vec(buf).unwrap();
    let meta_info: MetaInfo = FromBencode::from_bencode(&bencode).unwrap();
    println!("GET {}", meta_info.announce);
    println!("name {}", meta_info.info.name);
    println!("piece len {}", meta_info.info.piece_length);
    println!("pieces len {}", meta_info.info.pieces.len());
    println!("created by {:?}", meta_info.created_by);
    println!("creation date {:?}", meta_info.creation_date);
    println!("encoding {:?}", meta_info.encoding);
    match meta_info.info.mode {
        Mode::Single(single) => {
            println!("md5sum {:?}", single.md5sum);
            println!("single file length {:?}", single.length);
        }
        _ => println!("nope"),
    }
}
