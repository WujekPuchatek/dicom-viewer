use std::rc::Rc;
use memmap2::Mmap;
use crate::dataset::tag::Tag;
use crate::data_reader::data_reader::{DataReader, Endianness, Whence};
use crate::dataset::data_element::DataElement;
use crate::dataset::value_field::ValueField;
use crate::dicom_constants::numeric::HEADER_END;
use crate::dicom_constants::tags::{ITEM, ITEM_DELIMITATION, SEQUENCE_DELIMITATION};
use crate::dicom_file_parser::value_reader::{ExplicitValueReader, ValueReader};
use crate::value_representations::numeric_type::Numeric;
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

    pub fn parse(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let content = self.open_file()?;

        if Validator::new(&content).validate() == ValidationResult::NotDicom {
            return Err("Not a DICOM file".into());
        }

        let mut reader = DataReader::new(&content, Endianness::Little);
        reader.seek(Whence::Start, HEADER_END);

        let old_value_read_all_tags = self.read_all_tags;
        self.read_all_tags = true;

        let meta_data = self.read_meta_data(&mut reader);

        self.read_all_tags = old_value_read_all_tags;

        if let Err(e) = meta_data {
            return Err(e);
        }

        while reader.unconsumed() > 0 {
            let tag = self.dicom_dataset_reader.read_tag(&mut reader);
            let data_element = self.read_data_element(&tag, &mut reader);

            if let Some(data_element) = data_element {
                println!("{:?}", data_element);
            }
        }



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

    fn read_data_element(&self, tag: &Tag, reader: &mut DataReader) -> Option<DataElement> {
        if self.sequence_of_item_special_tag(tag) {
            reader.seek(Whence::Current, 4);

        }

        if self.read_all_tags || self.tags_to_read.contains(tag)
        {
            return Some(self.dicom_dataset_reader.read_data_element(&tag, reader));
        }
        else
        {
            self.dicom_dataset_reader.skip_value(&tag, reader);
            return None;
        }

    }

    fn read_meta_data(&self, reader: &mut DataReader) -> Result<std::vec::Vec<DataElement>,
                                                                Box<dyn std::error::Error>>
    {
        let tag = self.dicom_dataset_reader.read_tag(reader);
        let file_meta_information_group_length = self.dicom_dataset_reader.read_data_element(&tag, reader);

        let filemeta_length = match file_meta_information_group_length.value {
            ValueField::UnsignedLong(u32) => u32.Value().first().copied(),
            _ => return Err("File meta information group length should be kept as unsigned long".into())
        };

        if let None = filemeta_length {
            return Err("Cannot read file meta information group length".into());
        }

        const EXPECTED_MAX_NUM_OF_ELEMENTS: usize = 20;
        let mut elems = Vec::with_capacity(EXPECTED_MAX_NUM_OF_ELEMENTS);
        let end_of_file_meta = reader.unconsumed() - filemeta_length.unwrap() as isize;

        while reader.unconsumed() > end_of_file_meta {
            let tag = self.dicom_dataset_reader.read_tag(reader);

            elems.push(
                self.read_data_element(&tag, reader).unwrap());
        }

        Ok(elems)
    }
}