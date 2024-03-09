mod data_reader;
mod dicom_file_parser;
mod dicom_constants;
mod dataset;

fn main()  -> std::io::Result<()>
{
    let path = "C:/Users/medapp/Desktop/CT/im001.dcm";

    let parser = dicom_file_parser::DicomFileParser::new()
                 .file_path(path)
                 .read_all_tags()
                 .parse()?;
    Ok(())
}