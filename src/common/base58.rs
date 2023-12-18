pub type Error = bs58::decode::Error;

pub fn encode<T: AsRef<[u8]>>(bytes: T) -> Vec<u8> {
    bs58::encode(bytes).into_vec()
}

pub fn to_string<T: AsRef<[u8]>>(bytes: T) -> String {
    String::from_utf8(encode(bytes)).unwrap()
}

pub fn decode<T: AsRef<[u8]>>(value: T) -> Result<Vec<u8>, Error> {
    bs58::decode(value).into_vec()
}
