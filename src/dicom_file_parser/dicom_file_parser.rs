use std::rc::Rc;
use memmap2::Mmap;
use crate::dataset::tag::Tag;
use crate::data_reader::data_reader::{DataReader, Endianness, Whence};
use crate::dicom_constants::numeric::HEADER_END;
use crate::dicom_constants::tags::{ITEM, ITEM_DELIMITATION, SEQUENCE_DELIMITATION};
use crate::dicom_file_parser::value_reader::{ExplicitValueReader, ValueReader};
use super::validator::{Validator, ValidationResult};


pub struct DicomFileParser {
    file_path: String,
    tags_to_read : std::collections::HashSet<Tag>,
    read_all_tags : bool,
    dicom_dataset_reader: ValueReader
}

impl DicomFileParser {
    pub fn new() -> Self {
        Self { file_path: "".parse().unwrap(),
               tags_to_read: std::collections::HashSet::new(),
               read_all_tags: false,
               dicom_dataset_reader: ValueReader::Explicit(ExplicitValueReader{}) }
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

        let meta_data = self.read_meta_data(&mut reader);


        Ok(())
    }

    fn open_file(&self) -> Result<Rc<Mmap>, std::io::Error> {
        let file = std::fs::File::open(&self.file_path)?;
        let mapped_file = unsafe { Mmap::map(&file)? };
        Ok(Rc::new(mapped_file))
    }

    fn sequence_of_item_special_tag(&self, tag: &Tag) -> bool {
        tag == &ITEM || tag == &ITEM_DELIMITATION || tag == &SEQUENCE_DELIMITATION
    }

    fn read_data_element_with_explicit_vr(&self, tag: &Tag, reader: &mut DataReader) -> Result<(), Box<dyn std::error::Error>> {
        if self.sequence_of_item_special_tag(tag) {
            reader.seek(Whence::Current, 4);
            return Ok(())
        }

        if self.read_all_tags || self.tags_to_read.contains(tag) {
        }
        else {
        }

        Ok(())
    }

    fn read_meta_data(&self, reader: &mut DataReader) {
        let tag = self.dicom_dataset_reader.read_tag(reader);
        let file_meta_information_group_length = self.dicom_dataset_reader.read_data_element(&tag, reader);
    }
}