use crate::dicom_constants::numeric::*;
 pub(crate) struct Validator<'a> {
    data: &'a [u8]
}

#[derive(Debug, PartialEq)]
pub enum ValidationResult {
    Dicom,
    NotDicom
}

impl<'a> Validator<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    pub fn validate(&self) -> ValidationResult {
        if self.data.len() < HEADER_END {
            return ValidationResult::NotDicom;
        }

        if &self.data[HEADER_START..HEADER_END] != HEADER {
            return ValidationResult::NotDicom;
        }

        ValidationResult::Dicom
    }
}

mod tests {
    #[test]
    fn test_new() {
        let data = b"some data";
        let validator = Validator::new(data);
        assert_eq!(validator.data, data);
    }

    #[test]
    fn test_validate_dicom() {
        let mut data = vec![0; 132];
        data[128..132].copy_from_slice(b"DICM");
        let validator = Validator::new(&data);
        assert_eq!(validator.validate(), ValidationResult::Dicom);
    }

    #[test]
    fn test_validate_not_dicom() {
        let data = b"not dicom data";
        let validator = Validator::new(data);
        assert_eq!(validator.validate(), ValidationResult::NotDicom);
    }

    #[test]
    fn test_validate_length_exact_not_dicom() {
        let data = vec![0; 132];
        let validator = Validator::new(&data);
        assert_eq!(validator.validate(), ValidationResult::NotDicom);
    }

    #[test]
    fn test_validate_length_short() {
        let data = vec![0; 131];
        let validator = Validator::new(&data);
        assert_eq!(validator.validate(), ValidationResult::NotDicom);
    }

    #[test]
    fn test_validate_length_exact_wrong_header() {
        let mut data = vec![0; 132];
        data[128..132].copy_from_slice(b"NOTD");
        let validator = Validator::new(&data);
        assert_eq!(validator.validate(), ValidationResult::NotDicom);
    }
}
