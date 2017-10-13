#![allow(dead_code)]

extern crate bencode;
extern crate crypto;
extern crate hyper;
extern crate reqwest;

use bencode::{FromBencode, Bencode, NumFromBencodeError, StringFromBencodeError};
use bencode::util::ByteString;
use std::fs::File;
use std::io::{self, Read, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use crypto::sha1::Sha1;
use crypto::digest::Digest;
use hyper::Url;

mod decoder_helper;
mod meta_info;
mod tracker_response;

use meta_info::{ MetaInfo, Mode };
use tracker_response::TrackerResponse;

fn tracker(meta_info: MetaInfo) {
    let time = SystemTime::now().duration_since(UNIX_EPOCH).expect("time went backwards");
    let mut hasher = Sha1::new();
    hasher.input_str(&time.as_secs().to_string());
    let port = 6881;
    let uploaded = 0;
    let downloaded = 0;
    let left = meta_info.info.piece_length;
    let compact = 0;
    let no_peer_id = 0;
    let event = "started";
    let mut url = Url::parse(&meta_info.announce).expect("invalid announce url");
    let result = hasher.result_str();
    let hashed_id = format!("TR-2-92-{}", result);
    let prefixed_peer_id = hashed_id.get(..20).expect("error");
    let mut query = vec![
        ("info_hash".to_string(), meta_info.hash),
        ("peer_id".to_string(), prefixed_peer_id.to_string()),
        ("port".to_string(), port.to_string()),
        ("uploaded".to_string(), uploaded.to_string()),
        ("downloaded".to_string(), downloaded.to_string()),
        ("left".to_string(), left.to_string()),
        ("compact".to_string(), compact.to_string()),
        ("no_peer_id".to_string(), no_peer_id.to_string()),
        ("event".to_string(), event.to_string()),
    ];
    if let Some(queries) = url.query_pairs() {
        query.extend(queries.iter().cloned());
    }
    url.set_query_from_pairs(query);
    let full_url = url.serialize();
    println!("making request to tracker: {}", full_url);
    let mut response = reqwest::get(&full_url).unwrap();
    let mut buf = Vec::new();
    let _ = response.read_to_end(&mut buf);
    println!("response: {}", buf.len());
    let resp = bencode::from_vec(buf).unwrap();
    let tracker_response: TrackerResponse = FromBencode::from_bencode(&resp).unwrap();
    println!("interval yay {}", tracker_response.interval);
}


fn main() {
    let mut file = File::open("got.torrent").expect("error opening file");
    //let mut file = File::open("harry.torrent").expect("error opening file");
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf);
    let bencode: bencode::Bencode = bencode::from_vec(buf).expect("kill me");
    let meta_info: MetaInfo = FromBencode::from_bencode(&bencode).unwrap();
    println!("GET {}", meta_info.announce);
    println!("name {}", meta_info.info.name);
    println!("piece len {}", meta_info.info.piece_length);
    println!("pieces len {}", meta_info.info.pieces.len());
    println!("created by {:?}", meta_info.created_by);
    println!("creation date {:?}", meta_info.creation_date);
    println!("encoding {:?}", meta_info.encoding);
    println!("private {:?}", meta_info.info.private);
    tracker(meta_info.clone());
    match meta_info.info.mode {
        Mode::Single(single) => {
            println!("md5sum {:?}", single.md5sum);
            println!("single file length {:?}", single.length);
        }
        _ => println!("nope"),
    }
}
