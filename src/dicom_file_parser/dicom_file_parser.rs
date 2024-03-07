use std::io::Read;
use std::rc::Rc;
use memmap2::Mmap;
use crate::dataset::tag::Tag;
use crate::data_reader::data_reader::{DataReader, Endianness, Whence};
use crate::dataset::value_representation::ValueRepresentation;
use crate::dicom_constants::numeric::HEADER_END;
use super::validator::{Validator, ValidationResult};
pub struct DicomFileParser {
    file_path: String,

}

impl DicomFileParser {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }

    fn open_file(&self) -> Result<Rc<Mmap>, std::io::Error> {
        let file = std::fs::File::open(&self.file_path)?;
        let mapped_file = unsafe { memmap2::Mmap::map(&file)? };
        Ok(Rc::new(mapped_file))
    }

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

    fn read_value_length(&self, reader: &mut DataReader) -> u32 {
        reader.read_u32()
    }

    fn read_data_element_with_explicit_vr(&self, reader: &mut DataReader) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn read_meta_data(&self, reader: &mut DataReader) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
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
}