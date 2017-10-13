use bencode::{FromBencode, Bencode, NumFromBencodeError, StringFromBencodeError};
use bencode::util::ByteString;
use decoder_helper;
use decoder_helper::DecodingError;
use crypto::sha1::Sha1;
use crypto::digest::Digest;

#[derive(Clone)]
pub struct FileDesc {
    length: i64,
    md5sum: Option<String>,
    path: Vec<String>,
}

#[derive(Clone)]
pub struct MultiFile {
    files: Vec<FileDesc>,
}

#[derive(Clone)]
pub struct SingleFile {
   pub length: i64,
   pub md5sum: Option<String>,
}

#[derive(Clone)]
pub enum Mode {
    Single(SingleFile),
    Multiple(MultiFile),
}

#[derive(Clone)]
pub struct Info {
   pub mode: Mode,
   pub name: String,
   pub piece_length: i64,
   pub pieces: Vec<u8>,
   pub private: Option<u8>,
}

impl FromBencode for Info {
    type Err = decoder_helper::DecodingError;
    fn from_bencode(bencode: &Bencode) -> Result<Info, DecodingError> {
        use self::DecodingError::*;
        match bencode {
            &Bencode::Dict(ref m) => {
                let pieces =  m.get(&ByteString::from_str("pieces")).expect("pieces not defined");
                match pieces {
                    &Bencode::ByteString(ref vec) =>  {
                        // if it has length it means its single file
                        if let Some(length) = get_optional_field!(m, "length") {
                            let mode = Mode::Single(SingleFile {
                                length: length,
                                md5sum: get_optional_field!(m, "md5sum"),
                            });
                            return Ok(Info {
                                mode: mode,
                                name: get_field!(m, "name").unwrap(),
                                private: get_optional_field!(m, "private"),
                                piece_length: get_field!(m, "piece length").unwrap(),
                                pieces: vec.clone(),
                            });
                        }
                    },
                    _ => panic!("pieces not a byte string"),
                }
                Err(MissingField)
            },
            _ => Err(NotADict),
        }
    }
}

#[derive(Clone)]
pub struct MetaInfo {
  pub info: Info,
  pub hash: String,
  pub announce: String,
  pub created_by: Option<String>,
  pub creation_date: Option<u64>,
  pub comment: Option<String>,
  pub encoding: Option<String>,
}

impl FromBencode for MetaInfo {
    type Err = DecodingError;
    fn from_bencode(bencode: &Bencode) -> Result<MetaInfo, DecodingError> {
        use self::DecodingError::*;
        match bencode {
            &Bencode::Dict(ref m) => {
                let info_dict = m.get(&ByteString::from_str("info")).expect("info field not found");
                let info = Info::from_bencode(info_dict).unwrap();
                let mut hasher = Sha1::new();
                let bytevec = info_dict.to_bytes().unwrap();
                hasher.input(bytevec.as_slice());
                Ok(MetaInfo{
                    info: info,
                    hash: hasher.result_str(),
                    announce: get_field!(m, "announce").unwrap(),
                    created_by: get_optional_field!(m, "created by"),
                    creation_date: get_optional_field!(m, "creation date"),
                    comment: get_optional_field!(m, "comment"),
                    encoding: get_optional_field!(m, "encoding"),
                })
            },
            _ => Err(NotADict),
        }
    }
}
