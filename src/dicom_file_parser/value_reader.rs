use crate::dataset::tag::Tag;
use crate::data_reader::data_reader::{DataReader, Whence};
use crate::dataset::value_field::ValueField;
use crate::dataset::value_representation::ValueRepresentation;
use crate::value_representations::other_type::OtherType;
use crate::value_representations::string_alike::StringAlike;

mod private {
    pub struct Local {}

    pub const LOCAL : Local = Local{};
}

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
        if self.value_length_kept_on_2_bytes(value_representation, private::LOCAL) {
            return reader.read_u16() as u32
        }

        const RESERVED_BYTES: usize = 2;
        reader.seek(Whence::Current, RESERVED_BYTES);
        reader.read_u32()
    }

    fn read_value(&self,
                  value_representation: ValueRepresentation,
                  value_length : u32,
                  reader: &mut DataReader) -> ValueField {
        if value_representation.value == *b"AE" {
            return ValueField::ApplicationEntity(self.read_string(reader, value_length, private::LOCAL));
        }

        if value_representation.value == *b"AS" {
            return ValueField::AgeString(self.read_string(reader, value_length, private::LOCAL));
        }

        if value_representation.value == *b"AT" {
            return ValueField::AttributeTag(self.read_string(reader, value_length, private::LOCAL));
        }

        if value_representation.value == *b"CS" {
            return ValueField::CodeString(self.read_string(reader, value_length, private::LOCAL));
        }

        if value_representation.value == *b"DA" {
            return ValueField::Date(self.read_string(reader, value_length, private::LOCAL));
        }

        if value_representation.value == *b"DS" {
            return ValueField::DecimalString(self.read_string(reader, value_length, private::LOCAL));
        }

        if value_representation.value == *b"DT" {
            return ValueField::DateTime(self.read_string(reader, value_length, private::LOCAL));
        }

        if value_representation.value == *b"FL" {
            // return ValueField::FloatingPointSingle(self.read_other_bytes(reader, value_length, private::LOCAL));
        }

        if value_representation.value == *b"FD" {
            // return ValueField::FloatingPointDouble(self.read_other_bytes(reader, value_length, private::LOCAL));
        }

        if value_representation.value == *b"IS" {
            return ValueField::IntegerString(self.read_string(reader, value_length, private::LOCAL));
        }

        if value_representation.value == *b"LO" {
            return ValueField::LongString(self.read_string(reader, value_length, private::LOCAL));
        }

        if value_representation.value == *b"LT" {
            return ValueField::LongText(self.read_string(reader, value_length, private::LOCAL));
        }

        if value_representation.value == *b"OB" {
            return ValueField::OtherByte(self.read_other_bytes(reader, value_length, private::LOCAL));
        }

        if value_representation.value == *b"OD" {
            return ValueField::OtherDouble(self.read_other_bytes(reader, value_length, private::LOCAL));
        }

        if value_representation.value == *b"OF" {
            return ValueField::OtherFloat(self.read_other_bytes(reader, value_length, private::LOCAL));
        }

        if value_representation.value == *b"OL" {
            return ValueField::OtherLong(self.read_other_bytes(reader, value_length, private::LOCAL));
        }

        if value_representation.value == *b"OV" {
            return ValueField::Other64bitVeryLong(self.read_other_bytes(reader, value_length, private::LOCAL));
        }

        if value_representation.value == *b"OW" {
            return ValueField::OtherWord(self.read_other_bytes(reader, value_length, private::LOCAL));
        }

        if value_representation.value == *b"PN" {
            return ValueField::PersonName(self.read_string(reader, value_length, private::LOCAL));
        }

        if value_representation.value == *b"SH" {
            return ValueField::ShortString(self.read_string(reader, value_length, private::LOCAL));
        }

        if value_representation.value == *b"SL" {
            //return ValueField::SignedLong(self.read_other_bytes(reader, value_length, private::LOCAL));
        }

        if value_representation.value == *b"SQ" {
            //return ValueField::SequenceOfItems(self.read_other_bytes(reader, value_length, private::LOCAL));
        }

        if value_representation.value == *b"SS" {
            //return ValueField::SignedShort(self.read_other_bytes(reader, value_length, private::LOCAL));
        }

        if value_representation.value == *b"ST" {
            return ValueField::ShortText(self.read_string(reader, value_length, private::LOCAL));
        }

        if value_representation.value == *b"SV" {
            //return ValueField::Signed64bitVeryLong(self.read_other_bytes(reader, value_length, private::LOCAL));
        }

        if value_representation.value == *b"TM" {
            return ValueField::Time(self.read_string(reader, value_length, private::LOCAL));
        }

        if value_representation.value == *b"UC" {
            return ValueField::UnlimitedCharacters(self.read_string(reader, value_length, private::LOCAL));
        }

        if value_representation.value == *b"UI" {
            return ValueField::UniqueIdentifier(self.read_string(reader, value_length, private::LOCAL));
        }

        if value_representation.value == *b"UL" {
            //return ValueField::UnsignedLong(self.read_other_bytes(reader, value_length, private::LOCAL));
        }

        if value_representation.value == *b"UN" {
            //return ValueField::Unknown(self.read_other_bytes(reader, value_length, private::LOCAL));
        }

        if value_representation.value == *b"UR" {
            return ValueField::UniversalResourceIdentifier(self.read_string(reader, value_length, private::LOCAL));
        }

        if value_representation.value == *b"US" {
            //return ValueField::UnsignedShort(self.read_other_bytes(reader, value_length, private::LOCAL));
        }

        if value_representation.value == *b"UT" {
            return ValueField::UnlimitedText(self.read_string(reader, value_length, private::LOCAL));
        }

        if value_representation.value == *b"UV" {
            //return ValueField::Unsigned64bitVeryLong(self.read_other_bytes(reader, value_length, private::LOCAL));
        }

        panic!("Unknown value representation: {:?}", value_representation);
    }
    fn value_length_kept_on_2_bytes(&self, value_representation: &ValueRepresentation, _ : private::Local) -> bool {
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
        vr == *b"LO" ||
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

    fn read_string<VR: StringAlike>(&self, reader: &mut DataReader, length: u32, _ : private::Local) -> VR {
        let str = reader.read_string(length as usize);
        VR::from_string(str)
    }

    fn read_other_bytes<VR: OtherType>(&self, reader: &mut DataReader, length: u32, _ : private::Local) -> VR
        where
            VR::Type: Default + Copy
    {
        let length = length as usize / std::mem::size_of::<VR::Type>();
        let mut vec = vec![VR::Type::default(); length as usize];

        let slice = unsafe {
            std::slice::from_raw_parts_mut(vec.as_mut_ptr() as *mut u8,
                                           vec.len() * std::mem::size_of::<VR::Type>())
        };

        reader.read_exact(slice);
        VR::new(vec)
    }

    fn cast_to_u8_slice<'a, VR: OtherType>(&self, vec: &mut Vec<VR::Type>, _ : private::Local) -> &'a [u8] {
        unsafe {
            std::slice::from_raw_parts(vec.as_ptr() as *const u8, vec.len() * std::mem::size_of::<VR::Type>())
        }
    }
}

pub struct ExplicitValueReader {}
impl ValueReaderBase for ExplicitValueReader {}

pub struct ImplicitValueReader {}
impl ValueReaderBase for ImplicitValueReader {}

impl ImplicitValueReader {
    pub fn read_value(&self,
                  value_representation: Option<ValueRepresentation>,
                  value_length : u32,
                  reader: &mut DataReader) -> ValueField {
        if self.find_element_in_dict() {
            panic!("Not implemented")
        }

        panic!("Not implemented")
    }

    fn find_element_in_dict(&self) -> bool {
        false
    }
}



pub enum ValueReader
{
    Explicit(ExplicitValueReader),
    Implicit(ImplicitValueReader),
}

impl ValueReader {
    pub fn new_explicit() -> Self {
        ValueReader::Explicit(ExplicitValueReader{})
    }

    pub fn new_implicit() -> Self {
        ValueReader::Implicit(ImplicitValueReader{})
    }

    pub fn read_tag(&self, reader: &mut DataReader) -> Tag {
        match self {
            ValueReader::Explicit(explicit_reader) => explicit_reader.read_tag(reader),
            ValueReader::Implicit(implicit_reader) => implicit_reader.read_tag(reader),
        }
    }

    pub fn  read_value_representation(&self, reader: &mut DataReader) -> ValueRepresentation {
        match self {
            ValueReader::Explicit(explicit_reader) => explicit_reader.read_value_representation(reader),
            ValueReader::Implicit(implicit_reader) => implicit_reader.read_value_representation(reader),
        }
    }

    pub fn read_value_length(&self, value_representation: &ValueRepresentation, reader: &mut DataReader) -> u32 {
        match self {
            ValueReader::Explicit(explicit_reader) => explicit_reader.read_value_length(value_representation, reader),
            ValueReader::Implicit(implicit_reader) => implicit_reader.read_value_length(value_representation, reader),
        }
    }

    pub fn read_value(&self,
                      value_representation: Option<ValueRepresentation>,
                      value_length : u32,
                      reader: &mut DataReader) -> ValueField {
        match self {
            ValueReader::Explicit(explicitReader) =>
                explicitReader.read_value(value_representation.unwrap(), value_length, reader),

            ValueReader::Implicit(implicitReader) =>
                implicitReader.read_value(value_representation, value_length, reader),
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