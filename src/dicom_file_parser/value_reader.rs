use crate::dataset::tag::Tag;
use crate::data_reader::data_reader::{DataReader, Whence};
use crate::dataset::value_representation::ValueRepresentation;

pub trait ValueReader {
    fn read_tag(&self, reader: &mut DataReader) -> Tag {
        let group = reader.read_u16();
        let element = reader.read_u16();

        Tag { group, element }
    }

    fn read_value_representation(&self, reader: &mut DataReader) -> ValueRepresentation {
        let mut vr = [0; 2];
        reader.read_exact(&mut vr);
        ValueRepresentation {value: vr }
    }

    fn read_value_length(&self, value_representation: &ValueRepresentation, reader: &mut DataReader) -> u32 {
        if (self.value_length_kept_on_2_bytes(value_representation)) {
            reader.read_u16() as u32
        }

        const RESERVED_BYTES: usize = 2;
        reader.seek(Whence::Current, RESERVED_BYTES);
        reader.read_u32()
    }

    fn value_length_kept_on_2_bytes(&self, value_representation: &ValueRepresentation) -> bool {
        let vr = value_representation.value;

        vr == b"AE" ||
            vr == b"AS" ||
            vr == b"AT" ||
            vr == b"CS" ||
            vr == b"DA" ||
            vr == b"DS" ||
            vr == b"DT" ||
            vr == b"FL" ||
            vr == b"FD" ||
            vr == b"IS" ||
            vr == b"LT" ||
            vr == b"PN" ||
            vr == b"SH" ||
            vr == b"SL" ||
            vr == b"SS" ||
            vr == b"ST" ||
            vr == b"TM" ||
            vr == b"UI" ||
            vr == b"UL" ||
            vr == b"US"
    }

    fn read_string(&self, reader: &mut DataReader, length: u32) -> String {
        let mut buffer = vec![0; length as usize];
        reader.read_exact(&mut buffer);
        String::from_utf8(buffer).unwrap()
    }
}
