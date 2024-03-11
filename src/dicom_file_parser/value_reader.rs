use std::mem::size_of;
use crate::dataset::tag::Tag;
use crate::data_reader::data_reader::{DataReader, Whence};
use crate::dataset::value_representation::ValueRepresentation;
use crate::value_representations::string_alike::StringAlike;
use crate::value_representations::other_type::OtherType;

pub trait ValueReaderBase {
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
        if self.value_length_kept_on_2_bytes(value_representation) {
            return reader.read_u16() as u32
        }

        const RESERVED_BYTES: usize = 2;
        reader.seek(Whence::Current, RESERVED_BYTES);
        reader.read_u32()
    }

    fn value_length_kept_on_2_bytes(&self, value_representation: &ValueRepresentation) -> bool {
        let vr = value_representation.value;

        vr == *b"AE" ||
        vr == *b"AS" ||
        vr == *b"AT" ||
        vr == *b"CS" ||
        vr == *b"DA" ||
        vr == *b"DS" ||
        vr == *b"DT" ||
        vr == *b"FL" ||
        vr == *b"FD" ||
        vr == *b"IS" ||
        vr == *b"LT" ||
        vr == *b"PN" ||
        vr == *b"SH" ||
        vr == *b"SL" ||
        vr == *b"SS" ||
        vr == *b"ST" ||
        vr == *b"TM" ||
        vr == *b"UI" ||
        vr == *b"UL" ||
        vr == *b"US"
    }

    fn read_string<VR: StringAlike>(&self, reader: &mut DataReader, length: u32) -> VR {
        let string = reader.read_string(length as usize);
        VR::from_string(string)
    }

    fn cast_to_u8_slice<'a, VR: OtherType>(&self, vec: &mut Vec<VR::Type>) -> &'a[u8] {
        let ptr = vec.as_ptr() as *const u8;
        let len = vec.len() * std::mem::size_of::<VR::Type>();
        unsafe { std::slice::from_raw_parts(ptr, len) }
    }

    fn read_other_bytes<VR: OtherType>(&self, reader: &mut DataReader, length: u32) -> VR
    where
        VR::Type: Default + Copy
    {
        let num_of_values = length / (size_of::<VR::Type>() as u32);

        let mut bytes = vec![VR::Type::default(); length as usize];
        let mut casted = self.cast_to_u8_slice::<VR>(&mut bytes);
        reader.read_exact(&mut casted);
        VR::new(bytes)
    }
}

pub struct  ExplicitValueReader {}
impl ValueReaderBase for ExplicitValueReader {}

pub struct ImplicitValueReader {}
impl ValueReaderBase for ImplicitValueReader {}



pub enum ValueReader
{
    Explicit(ExplicitValueReader),
    Implicit(ImplicitValueReader),
}

impl ValueReader {
    pub fn  read_value_representation(&self, reader: &mut DataReader) -> ValueRepresentation {
        match self {
            ValueReader::Explicit(explicitReader) => explicitReader.read_value_representation(reader),
            ValueReader::Implicit(implicitReader) => implicitReader.read_value_representation(reader),
        }
    }

    pub fn read_value_length(&self, value_representation: &ValueRepresentation, reader: &mut DataReader) -> u32 {
        match self {
            ValueReader::Explicit(explicitReader) => explicitReader.read_value_length(value_representation, reader),
            ValueReader::Implicit(implicitReader) => implicitReader.read_value_length(value_representation, reader),
        }
    }
}
// mod tests {
//     use crate::dicom_file_parser::dicom_file_parser::DicomFileParser;
//     use super::*;
//
//     #[test]
//     fn test_sequence_of_item_special_tag() {
//         let parser = DicomFileParser::new();
//
//         let item_tag = Tag { group: 0xFFFE, element: 0xE000 };
//         assert!(parser.sequence_of_item_special_tag(&item_tag));
//
//         let item_delimitation_tag = Tag { group: 0xFFFE, element: 0xE00D };
//         assert!(parser.sequence_of_item_special_tag(&item_delimitation_tag));
//
//         let sequence_delimitation_tag = Tag { group: 0xFFFE, element: 0xE0DD };
//         assert!(parser.sequence_of_item_special_tag(&sequence_delimitation_tag));
//
//         let non_special_tag = Tag { group: 0x0002, element: 0x0000 };
//         assert!(!parser.sequence_of_item_special_tag(&non_special_tag));
//     }
//
//     #[test]
//     fn test_value_length_kept_on_2_bytes() {
//         let parser = DicomFileParser::new();
//
//         let vr = ValueRepresentation { value: *b"AE" };
//         assert!(parser.value_length_kept_on_2_bytes(&vr));
//
//         let vr = ValueRepresentation { value: *b"AS" };
//         assert!(parser.value_length_kept_on_2_bytes(&vr));
//
//         let vr = ValueRepresentation { value: *b"AT" };
//         assert!(parser.value_length_kept_on_2_bytes(&vr));
//
//         let vr = ValueRepresentation { value: *b"CS" };
//         assert!(parser.value_length_kept_on_2_bytes(&vr));
//
//         let vr = ValueRepresentation { value: *b"DA" };
//         assert!(parser.value_length_kept_on_2_bytes(&vr));
//
//         let vr = ValueRepresentation { value: *b"DS" };
//         assert!(parser.value_length_kept_on_2_bytes(&vr));
//
//         let vr = ValueRepresentation { value: *b"DT" };
//         assert!(parser.value_length_kept_on_2_bytes(&vr));
//
//         let vr = ValueRepresentation { value: *b"FL" };
//         assert!(parser.value_length_kept_on_2_bytes(&vr));
//
//         let vr = ValueRepresentation { value: *b"FD" };
//         assert!(parser.value_length_kept_on_2_bytes(&vr));
//
//         let vr = ValueRepresentation { value: *b"IS" };
//         assert!(parser.value_length_kept_on_2_bytes(&vr));
//
//         let vr = ValueRepresentation { value: *b"LT" };
//         assert!(parser.value_length_kept_on_2_bytes(&vr));
//
//         let vr = ValueRepresentation { value: *b"PN" };
//         assert!(parser.value_length_kept_on_2_bytes(&vr));
//
//         let vr = ValueRepresentation { value: *b"SH" };
//         assert!(parser.value_length_kept_on_2_bytes(&vr));
//
//         let vr = ValueRepresentation { value: *b"SL" };
//         assert!(parser.value_length_kept_on_2_bytes(&vr));
//
//         let vr = ValueRepresentation { value: *b"SS" };
//         assert!(parser.value_length_kept_on_2_bytes(&vr));
//
//         let vr = ValueRepresentation { value: *b"ST" };
//         assert!(parser.value_length_kept_on_2_bytes(&vr));
//
//         let vr = ValueRepresentation { value: *b"TM" };
//         assert!(parser.value_length_kept_on_2_bytes(&vr));
//
//         let vr = ValueRepresentation { value: *b"UI" };
//         assert!(parser.value_length_kept_on_2_bytes(&vr));
//
//         let vr = ValueRepresentation { value: *b"UL" };
//         assert!(parser.value_length_kept_on_2_bytes(&vr));
//
//         let vr = ValueRepresentation { value: *b"US" };
//         assert!(parser.value_length_kept_on_2_bytes(&vr));
//
//         let vr = ValueRepresentation { value: *b"OB" };
//         assert!(!parser.value_length_kept_on_2_bytes(&vr));
//
//         let vr = ValueRepresentation { value: *b"OW" };
//         assert!(!parser.value_length_kept_on_2_bytes(&vr));
//
//         let vr = ValueRepresentation { value: *b"OF" };
//         assert!(!parser.value_length_kept_on_2_bytes(&vr));
//
//         let vr = ValueRepresentation { value: *b"SQ" };
//         assert!(!parser.value_length_kept_on_2_bytes(&vr));
//
//         let vr = ValueRepresentation { value: *b"UN" };
//         assert!(!parser.value_length_kept_on_2_bytes(&vr));
//
//         let vr = ValueRepresentation { value: *b"UT" };
//         assert!(!parser.value_length_kept_on_2_bytes(&vr));
//     }
// }