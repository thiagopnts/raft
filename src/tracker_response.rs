
use bencode::{FromBencode, Bencode, NumFromBencodeError, StringFromBencodeError};
use bencode::util::ByteString;
use decoder_helper;
use decoder_helper::DecodingError;

pub struct TrackerResponse {
    pub interval: i32,
}

impl FromBencode for TrackerResponse {
    type Err = decoder_helper::DecodingError;
    fn from_bencode(bencode: &Bencode) -> Result<TrackerResponse, DecodingError> {
        use decoder_helper::DecodingError::*;
        match bencode {
            &Bencode::Dict(ref m) => {
                let value = m.get(&ByteString::from_str("interval")).expect("tracker_id not defined");
                let interval = FromBencode::from_bencode(value).ok().expect("error decoding ");
                Ok(TrackerResponse {
                    interval: interval,
                })
            },
            _ => Err(NotADict),
        }
    }
}
