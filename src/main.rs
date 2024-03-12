use crate::dicom_file_parser::dicom_file_parser::DicomFileParser;

mod data_reader;
mod dicom_file_parser;
mod dicom_constants;
mod dataset;
mod value_representations;

fn main()  -> std::io::Result<()>
{
    let path = "C:/Users/medapp/Desktop/CT/im001.dcm";

    let parser = DicomFileParser::new()
                 .file_path(path)
                 .read_all_tags()
                 .parse();
    Ok(())
}