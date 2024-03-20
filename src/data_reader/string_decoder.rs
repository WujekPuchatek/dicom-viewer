use encoding::all::*;
use encoding::{Encoding, EncodingRef, DecoderTrap};

#[derive(Clone)]
pub struct StringDecoder {
    decoder: EncodingRef,
}

impl StringDecoder {
    pub fn new() -> Self {
        Self { decoder: UTF_8 }
    }
    pub fn decode(&self, value: Vec<u8>) -> String {
        self.decoder.decode(&*value, DecoderTrap::Ignore).unwrap()
    }

    pub fn change_decoder(&mut self, character_set : String) {
        if character_set == "ISO_IR 100" || character_set == "ISO 2022 IR 100" {
            self.decoder = ISO_8859_1;
            return;
        }

        if character_set == "ISO_IR 101" || character_set == "ISO 2022 IR 101" {
            self.decoder = ISO_8859_2;
            return;
        }

        if character_set == "ISO_IR 109" || character_set == "ISO 2022 IR 109" {
            self.decoder = ISO_8859_3;
            return;
        }

        if character_set == "ISO_IR 110" || character_set == "ISO 2022 IR 110" {
            self.decoder = ISO_8859_4;
            return;
        }

        if character_set == "ISO_IR 126" || character_set == "ISO 2022 IR 126" {
            self.decoder = ISO_8859_7;
            return;
        }

        if character_set == "ISO_IR 127" || character_set == "ISO 2022 IR 127" {
            self.decoder = ISO_8859_6;
            return;
        }

        if character_set == "ISO_IR 138" || character_set == "ISO 2022 IR 138" {
            self.decoder = ISO_8859_8;
            return;
        }

        if character_set == "ISO_IR 144" || character_set == "ISO 2022 IR 144" {
            self.decoder = ISO_8859_5;
            return;
        }

        if character_set == "ISO_IR 148" || character_set == "ISO 2022 IR 148" {
            // self.decoder = ISO_8859_9;
        }

        if character_set == "ISO_IR 166" || character_set == "ISO 2022 IR 166" {
            self.decoder = ISO_8859_10;
            return;
        }

        if character_set == "ISO_IR 192" || character_set == "ISO 2022 IR 192" {
            self.decoder = UTF_8;
            return;
        }

        if character_set == "GBK" || character_set == "GB18030" {
            self.decoder = GBK;
            return;
        }

        panic!("Unsupported character set: {}", character_set)
    }
}