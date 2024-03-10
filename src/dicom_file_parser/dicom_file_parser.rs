use std::io::Read;
use std::rc::Rc;
use memmap2::Mmap;
use crate::dataset::tag::Tag;
use crate::data_reader::data_reader::{DataReader, Endianness, Whence};
use crate::dataset::value_representation::ValueRepresentation;
use crate::dicom_constants::numeric::HEADER_END;
use crate::dicom_constants::tags::{ITEM, ITEM_DELIMITATION, SEQUENCE_DELIMITATION};
use super::validator::{Validator, ValidationResult};
pub struct DicomFileParser {
    file_path: String,
    tags_to_read : std::collections::HashSet<Tag>,
    read_all_tags : bool
}

impl DicomFileParser {
    pub fn new() -> Self {
        Self { file_path: "".parse().unwrap(),
               tags_to_read: std::collections::HashSet::new(),
               read_all_tags: false }
    }

    pub fn read_all_tags(mut self) -> Self {
        self.read_all_tags = true;
        self
    }

    pub fn read_tags(mut self, tags: &[Tag]) -> Self {
        self.tags_to_read = tags.into_iter().cloned().collect();
        self
    }

    pub fn file_path(mut self, file_path: &str) -> Self {
        self.file_path = file_path.parse().unwrap();
        self
    }

    pub fn parse(&self) -> Result<(), Box<dyn std::error::Error>> {
        let content = self.open_file()?;

        if Validator::new(&content).validate() == ValidationResult::NotDicom {
            return Err("Not a DICOM file".into());
        }

        let mut reader = DataReader::new(&content, Endianness::Little);
        reader.seek(Whence::Start, HEADER_END);

        let meta_data = self.read_meta_data(&mut reader)?;


        Ok(())
    }

    fn open_file(&self) -> Result<Rc<Mmap>, std::io::Error> {
        let file = std::fs::File::open(&self.file_path)?;
        let mapped_file = unsafe { Mmap::map(&file)? };
        Ok(Rc::new(mapped_file))
    }

    fn sequence_of_item_special_tag(&self, tag: &Tag) -> bool {
        tag == ITEM || tag == ITEM_DELIMITATION || tag == SEQUENCE_DELIMITATION
    }

    fn read_data_element_with_explicit_vr(&self, tag: &Tag, reader: &mut DataReader) -> Result<(), Box<dyn std::error::Error>> {
        if self.sequence_of_item_special_tag(tag) {
            reader.seek(Whence::Current, 4);
            return


                Ok(())
        }

        let value_representation = self.read_value_representation(reader);
        let value_length = self.read_value_length(&value_representation, reader);

        if self.read_all_tags || self.tags_to_read.contains(tag) {
            let data = reader.read_bytes(value_length as usize);
        }
        else {
            reader.seek(Whence::Current, value_length as usize);
        }

        Ok(())
    }

    fn read_meta_data(&self, reader: &mut DataReader) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_sequence_of_item_special_tag() {
        let parser = DicomFileParser::new();

        let item_tag = Tag { group: 0xFFFE, element: 0xE000 };
        assert!(parser.sequence_of_item_special_tag(&item_tag));

        let item_delimitation_tag = Tag { group: 0xFFFE, element: 0xE00D };
        assert!(parser.sequence_of_item_special_tag(&item_delimitation_tag));

        let sequence_delimitation_tag = Tag { group: 0xFFFE, element: 0xE0DD };
        assert!(parser.sequence_of_item_special_tag(&sequence_delimitation_tag));

        let non_special_tag = Tag { group: 0x0002, element: 0x0000 };
        assert!(!parser.sequence_of_item_special_tag(&non_special_tag));
    }

    #[test]
    fn test_value_length_kept_on_2_bytes() {
        let parser = DicomFileParser::new();

        let vr = ValueRepresentation { value: *b"AE" };
        assert!(parser.value_length_kept_on_2_bytes(&vr));

        let vr = ValueRepresentation { value: *b"AS" };
        assert!(parser.value_length_kept_on_2_bytes(&vr));

        let vr = ValueRepresentation { value: *b"AT" };
        assert!(parser.value_length_kept_on_2_bytes(&vr));

        let vr = ValueRepresentation { value: *b"CS" };
        assert!(parser.value_length_kept_on_2_bytes(&vr));

        let vr = ValueRepresentation { value: *b"DA" };
        assert!(parser.value_length_kept_on_2_bytes(&vr));

        let vr = ValueRepresentation { value: *b"DS" };
        assert!(parser.value_length_kept_on_2_bytes(&vr));

        let vr = ValueRepresentation { value: *b"DT" };
        assert!(parser.value_length_kept_on_2_bytes(&vr));

        let vr = ValueRepresentation { value: *b"FL" };
        assert!(parser.value_length_kept_on_2_bytes(&vr));

        let vr = ValueRepresentation { value: *b"FD" };
        assert!(parser.value_length_kept_on_2_bytes(&vr));

        let vr = ValueRepresentation { value: *b"IS" };
        assert!(parser.value_length_kept_on_2_bytes(&vr));

        let vr = ValueRepresentation { value: *b"LT" };
        assert!(parser.value_length_kept_on_2_bytes(&vr));

        let vr = ValueRepresentation { value: *b"PN" };
        assert!(parser.value_length_kept_on_2_bytes(&vr));

        let vr = ValueRepresentation { value: *b"SH" };
        assert!(parser.value_length_kept_on_2_bytes(&vr));

        let vr = ValueRepresentation { value: *b"SL" };
        assert!(parser.value_length_kept_on_2_bytes(&vr));

        let vr = ValueRepresentation { value: *b"SS" };
        assert!(parser.value_length_kept_on_2_bytes(&vr));

        let vr = ValueRepresentation { value: *b"ST" };
        assert!(parser.value_length_kept_on_2_bytes(&vr));

        let vr = ValueRepresentation { value: *b"TM" };
        assert!(parser.value_length_kept_on_2_bytes(&vr));

        let vr = ValueRepresentation { value: *b"UI" };
        assert!(parser.value_length_kept_on_2_bytes(&vr));

        let vr = ValueRepresentation { value: *b"UL" };
        assert!(parser.value_length_kept_on_2_bytes(&vr));

        let vr = ValueRepresentation { value: *b"US" };
        assert!(parser.value_length_kept_on_2_bytes(&vr));

        let vr = ValueRepresentation { value: *b"OB" };
        assert!(!parser.value_length_kept_on_2_bytes(&vr));

        let vr = ValueRepresentation { value: *b"OW" };
        assert!(!parser.value_length_kept_on_2_bytes(&vr));

        let vr = ValueRepresentation { value: *b"OF" };
        assert!(!parser.value_length_kept_on_2_bytes(&vr));

        let vr = ValueRepresentation { value: *b"SQ" };
        assert!(!parser.value_length_kept_on_2_bytes(&vr));

        let vr = ValueRepresentation { value: *b"UN" };
        assert!(!parser.value_length_kept_on_2_bytes(&vr));

        let vr = ValueRepresentation { value: *b"UT" };
        assert!(!parser.value_length_kept_on_2_bytes(&vr));
    }


}