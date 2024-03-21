use once_cell::sync::OnceCell;
use crate::dataset::tag::Tag;
use crate::data_reader::data_reader::{DataReader, Whence};
use crate::dataset::data_element::DataElement;
use crate::dataset::data_element_location::DataElementLocation;
use crate::dataset::value_field::ValueField;
use crate::dataset::value_representation::ValueRepresentation;
use crate::utils::submap::Submap;
use crate::value_representations::attribute_tag::AttributeTag;
use crate::value_representations::numeric_type::Numeric;

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

    fn read_value_representation(&self, reader: &mut DataReader) -> Option<ValueRepresentation>  {
        let mut vr = [0; 2];
        reader.read_exact(&mut vr);
        Some(ValueRepresentation {value: vr })
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
            return ValueField::AttributeTag(self.read_attribute_tag(reader, value_length, private::LOCAL));
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
            return ValueField::FloatingPointSingle(
                self.read_numeric_types(
                    || -> f32 { reader.read_f32() },
                    value_length,
                    private::LOCAL));
        }

        if value_representation.value == *b"FD" {
            return ValueField::FloatingPointDouble(
                self.read_numeric_types(
                    || -> f64 { reader.read_f64() },
                    value_length,
                    private::LOCAL));
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
            return ValueField::SignedLong(
                self.read_numeric_types(
                    || -> i32 { reader.read_i32() },
                    value_length,
                    private::LOCAL));
        }

        if value_representation.value == *b"SQ" {
            //return ValueField::SequenceOfItems(self.read_other_bytes(reader, value_length, private::LOCAL));
        }

        if value_representation.value == *b"SS" {
            return ValueField::SignedShort(
                self.read_numeric_types(
                    || -> i16 { reader.read_i16() },
                    value_length,
                    private::LOCAL));
        }

        if value_representation.value == *b"ST" {
            return ValueField::ShortText(self.read_string(reader, value_length, private::LOCAL));
        }

        if value_representation.value == *b"SV" {
            return ValueField::Signed64bitVeryLong(
                self.read_numeric_types(
                    || -> i64 { reader.read_i64() },
                    value_length,
                    private::LOCAL));
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
            return ValueField::UnsignedLong(
                self.read_numeric_types(
                    || -> u32 { reader.read_u32() },
                    value_length,
                    private::LOCAL));
        }

        if value_representation.value == *b"UN" {
            //return ValueField::Unknown(self.read_other_bytes(reader, value_length, private::LOCAL));
        }

        if value_representation.value == *b"UR" {
            return ValueField::UniversalResourceIdentifier(self.read_string(reader, value_length, private::LOCAL));
        }

        if value_representation.value == *b"US" {
            return ValueField::UnsignedShort(
                self.read_numeric_types(
                    || -> u16 { reader.read_u16() },
                    value_length,
                    private::LOCAL));
        }

        if value_representation.value == *b"UT" {
            return ValueField::UnlimitedText(self.read_string(reader, value_length, private::LOCAL));
        }

        if value_representation.value == *b"UV" {
            return ValueField::Unsigned64bitVeryLong(
                self.read_numeric_types(
                    || -> u64 { reader.read_u64() },
                    value_length,
                    private::LOCAL));
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

    fn read_string<VR: From<String> + From<DataElementLocation<String>>>(&self, reader: &mut DataReader, length: u32, _ : private::Local) -> VR {
        let lazy_read = self.get_size_of_lazy_read_element();

        if lazy_read.is_some() && lazy_read.unwrap()  >= length {
            return self.read_lazy_dicom_string::<VR>(reader, length, private::LOCAL);
        }

        let str = reader.read_string(length as usize);
        VR::from(str)
    }

    fn read_attribute_tag(&self, reader: &mut DataReader, length: u32, _ : private::Local) -> AttributeTag {
        let num_of_elems = length as usize / (2 * std::mem::size_of::<u16>());

        let mut val = Vec::with_capacity(num_of_elems);

        for _ in 0..num_of_elems {
            val.push([reader.read_u16(), reader.read_u16()]);
        }
        AttributeTag::new(val)
    }

    fn read_other_bytes<VR: From<Submap>>(&self, reader: &mut DataReader, length: u32, _ : private::Local) -> VR {
        let vr = {
            let desc = reader.get_subreader_desc(length as usize);
            VR::from(desc.submap)
        };

        reader.seek(Whence::Current, length as usize);
        vr
    }

    fn read_numeric_types<VR: Numeric, F: FnMut() -> VR::Type>(
        &self,
        mut read_function: F,
        length: u32, _ : private::Local) -> VR
        where
            VR::Type: Default + Copy
    {
        let length = length as usize / std::mem::size_of::<VR::Type>();
        let mut vec = Vec::<VR::Type>::with_capacity(length as usize);

        for _ in 0..length {
            vec.push(read_function());
        }

        VR::from(vec)
    }

    fn read_lazy_dicom_string<VR: From<DataElementLocation<String>>>(&self, reader: &mut DataReader, length: u32, _ : private::Local) -> VR {
        let vr = {
            let desc = reader.get_subreader_desc(length as usize);
            let reader_clone = reader.clone();

            let read_func = Box::new(move || -> String {
                let value: OnceCell<String> = OnceCell::new();
                let mut reader = DataReader::from_subreader_desc(desc.clone());

                value.get_or_init(|| {
                    reader.read_string(length as usize)
                }).to_string()
            });

            VR::from(DataElementLocation::new(read_func))
        };

        reader.seek(Whence::Current, length as usize);
        vr
    }

    fn read_data_element(&self, tag: &Tag, reader: &mut DataReader) -> DataElement;
    fn skip_data_element(&self, tag: &Tag, reader: &mut DataReader);

    fn set_size_of_lazy_read_element(&mut self, _size: Option<u32>);

    fn get_size_of_lazy_read_element(&self) -> Option<u32>;
}

pub struct ExplicitValueReader {
    size_of_lazy_read_element: Option<u32>
}

impl ExplicitValueReader {
    pub fn new() -> Self {
        ExplicitValueReader { size_of_lazy_read_element: None }
    }
}
impl ValueReaderBase for ExplicitValueReader {
    fn read_data_element(&self, tag: &Tag, reader: &mut DataReader) -> DataElement {
        let tag = *tag;
        let value_representation = self.read_value_representation(reader);
        let value_length = self.read_value_length(&value_representation.unwrap(), reader);
        let value = self.read_value(value_representation.unwrap(), value_length, reader);

        DataElement { tag, value_representation, value_length, value }
    }

    fn skip_data_element(&self, _tag: &Tag, reader: &mut DataReader) {
        let value_representation = self.read_value_representation(reader);
        let value_length = self.read_value_length(&value_representation.unwrap(), reader);
        reader.seek(Whence::Current, value_length as usize);
    }
    fn set_size_of_lazy_read_element(&mut self, size: Option<u32>) {
        self.size_of_lazy_read_element = size;
    }

    fn get_size_of_lazy_read_element(&self) -> Option<u32> {
        self.size_of_lazy_read_element
    }

}

pub struct ImplicitValueReader {
    size_of_lazy_read_element: Option<u32>
}

impl ImplicitValueReader {
    pub fn new() -> Self {
        ImplicitValueReader { size_of_lazy_read_element: None }
    }
}
impl ValueReaderBase for ImplicitValueReader {
    fn read_data_element(&self, _tag: &Tag, reader: &mut DataReader) -> DataElement {
        let tag = self.read_tag(reader);
        let value_length = self.read_value_length(reader);
        let value = self.read_value(value_length, reader);

        DataElement { tag, value_length, value, value_representation: None }
    }

    fn skip_data_element(&self, _tag: &Tag, reader: &mut DataReader) {
        let value_length = self.read_value_length(reader);
        reader.seek(Whence::Current, value_length as usize);
    }
    fn set_size_of_lazy_read_element(&mut self, size: Option<u32>) {
        self.size_of_lazy_read_element = size;
    }

    fn get_size_of_lazy_read_element(&self) -> Option<u32> {
        self.size_of_lazy_read_element
    }
}

impl ImplicitValueReader {
    pub fn read_value(&self,
                      _value_length : u32,
                      _reader: &mut DataReader) -> ValueField {
        if self.find_element_in_dict() {
            panic!("Not implemented")
        }

        panic!("Not implemented")
    }

    pub fn read_value_length(&self, reader: &mut DataReader) -> u32 {
        reader.read_u32()
    }

    pub fn read_value_representation(&self, _reader: &mut DataReader) -> Option<ValueRepresentation> {
        None
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
        ValueReader::Explicit(ExplicitValueReader::new())
    }

    pub fn new_implicit() -> Self {
        ValueReader::Implicit(ImplicitValueReader::new())
    }

    pub fn read_tag(&self, reader: &mut DataReader) -> Tag {
        match self {
            ValueReader::Explicit(explicit_reader) => explicit_reader.read_tag(reader),
            ValueReader::Implicit(implicit_reader) => implicit_reader.read_tag(reader),
        }
    }

    pub fn read_value_representation(&self, reader: &mut DataReader) -> Option<ValueRepresentation> {
        match self {
            ValueReader::Explicit(explicit_reader) => explicit_reader.read_value_representation(reader),
            ValueReader::Implicit(implicit_reader) => implicit_reader.read_value_representation(reader),
        }
    }

    pub fn read_value_length(&self, value_representation: Option<ValueRepresentation>, reader: &mut DataReader) -> u32 {
        match self {
            ValueReader::Explicit(explicit_reader) => explicit_reader.read_value_length(&value_representation.unwrap(), reader),
            ValueReader::Implicit(implicit_reader) => implicit_reader.read_value_length(reader),
        }
    }

    pub fn read_value(&self,
                      value_representation: Option<ValueRepresentation>,
                      value_length : u32,
                      reader: &mut DataReader) -> ValueField {
        match self {
            ValueReader::Explicit(explicit_reader) =>
                explicit_reader.read_value(value_representation.unwrap(), value_length, reader),

            ValueReader::Implicit(implicit_reader) =>
                implicit_reader.read_value(value_length, reader),
        }
    }

    pub fn skip_value(&self, tag: &Tag, reader: &mut DataReader) {
        match self {
            ValueReader::Explicit(explicit_reader) => explicit_reader.skip_data_element(tag, reader),
            ValueReader::Implicit(implicit_reader) => implicit_reader.skip_data_element(tag, reader),
        }
    }

    pub fn read_data_element(&self, tag: &Tag, reader: &mut DataReader) -> DataElement {
        match self {
            ValueReader::Explicit(explicit_reader) => explicit_reader.read_data_element(tag, reader),
            ValueReader::Implicit(implicit_reader) => implicit_reader.read_data_element(tag, reader),
        }
    }

    pub fn set_size_of_lazy_read_element(&mut self, size: Option<u32>) {
        match self {
            ValueReader::Explicit(explicit_reader) => explicit_reader.set_size_of_lazy_read_element(size),
            ValueReader::Implicit(implicit_reader) => implicit_reader.set_size_of_lazy_read_element(size),
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