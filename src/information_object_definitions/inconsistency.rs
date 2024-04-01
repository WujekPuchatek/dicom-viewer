use crate::Traits::cast::{CastArrayError, CastError};

#[derive(Clone)]
pub enum DicomFileInconsistency {
    MissingAttribute(&'static str),
    UnexpectedValueRepresentation(String),
    CastError(String),
}

impl<T: std::fmt::Debug> From<CastError<T>> for DicomFileInconsistency {
    fn from(error: CastError<T>) -> Self {
        DicomFileInconsistency::CastError(format!("{:?}", error))
    }
}

impl<T: std::fmt::Debug, const N: usize> From<CastArrayError<T, N>> for DicomFileInconsistency {
    fn from(error : CastArrayError<T, N>) -> Self {
        DicomFileInconsistency::CastError(format!("{:?}", error))
    }
}