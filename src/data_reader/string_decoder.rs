use encoding::all::*;
use encoding::{Encoding, EncodingRef, DecoderTrap};

pub struct StringDecoder {
    decoder: EncodingRef,
}

impl StringDecoder {
    pub fn new() -> Self {
        Self { decoder: UTF_8 }
    }
    pub fn change_decoder(&mut self, decoder: EncodingRef) {
        self.decoder = decoder;
    }

    pub fn decode(&self, value: Vec<u8>) -> String {
        self.decoder.decode(&*value, DecoderTrap::Ignore).unwrap()
    }
}