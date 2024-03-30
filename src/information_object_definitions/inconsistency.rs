pub enum DicomFileInconsistency {
    MissingAttribute(&'static str),
}