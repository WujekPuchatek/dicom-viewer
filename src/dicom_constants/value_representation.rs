use crate::dataset::value_representation::ValueRepresentation;

fn str_to_u8(s: &str) -> [u8; 2] {
    let bytes = s.as_bytes();
    [bytes[0], bytes[1]]
}

pub const APPLICATION_ENTITY : ValueRepresentation = ValueRepresentation { value: str_to_u8("AE") };
pub const AGE_STRING : ValueRepresentation = ValueRepresentation { value: str_to_u8("AS") };
pub const ATTRIBUTE_TAG : ValueRepresentation = ValueRepresentation { value: str_to_u8("AT") };
pub const CODE_STRING : ValueRepresentation = ValueRepresentation { value: str_to_u8("CS") };
pub const DATE : ValueRepresentation = ValueRepresentation { value: str_to_u8("DA") };
pub const DECIMAL_STRING : ValueRepresentation = ValueRepresentation { value: str_to_u8("DS") };
pub const DATE_TIME : ValueRepresentation = ValueRepresentation { value: str_to_u8("DT") };
pub const FLOATING_POINT_SINGLE : ValueRepresentation = ValueRepresentation { value: str_to_u8("FL") };
pub const FLOATING_POINT_DOUBLE : ValueRepresentation = ValueRepresentation { value: str_to_u8("FD") };
pub const INTEGER_STRING : ValueRepresentation = ValueRepresentation { value: str_to_u8("IS") };
pub const LONG_STRING : ValueRepresentation = ValueRepresentation { value: str_to_u8("LO") };
pub const LONG_TEXT : ValueRepresentation = ValueRepresentation { value: str_to_u8("LT") };
pub const OTHER_BYTE : ValueRepresentation = ValueRepresentation { value: str_to_u8("OB") };
pub const OTHER_DOUBLE : ValueRepresentation = ValueRepresentation { value: str_to_u8("OD") };
pub const OTHER_FLOAT : ValueRepresentation = ValueRepresentation { value: str_to_u8("OF") };
pub const OTHER_LONG : ValueRepresentation = ValueRepresentation { value: str_to_u8("OL") };
pub const OTHER_64_BIT_VERY_LONG : ValueRepresentation = ValueRepresentation { value: str_to_u8("OV") };
pub const OTHER_WORD : ValueRepresentation = ValueRepresentation { value: str_to_u8("OW") };
pub const PERSON_NAME : ValueRepresentation = ValueRepresentation { value: str_to_u8("PN") };
pub const SHORT_STRING : ValueRepresentation = ValueRepresentation { value: str_to_u8("SH") };
pub const SIGNED_LONG : ValueRepresentation = ValueRepresentation { value: str_to_u8("SL") };
pub const SEQUENCE_OF_ITEMS : ValueRepresentation = ValueRepresentation { value: str_to_u8("SQ") };
pub const SIGNED_SHORT : ValueRepresentation = ValueRepresentation { value: str_to_u8("SS") };
pub const SHORT_TEXT : ValueRepresentation = ValueRepresentation { value: str_to_u8("ST") };
pub const SIGNED_64_BIT_VERY_LONG : ValueRepresentation = ValueRepresentation { value: str_to_u8("SV") };
pub const TIME : ValueRepresentation = ValueRepresentation { value: str_to_u8("TM") };
pub const UNLIMITED_CHARACTER : ValueRepresentation = ValueRepresentation { value: str_to_u8("UC") };
pub const UNIQUE_IDENTIFIER_UID : ValueRepresentation = ValueRepresentation { value: str_to_u8("UI") };
pub const UNSIGNED_LONG : ValueRepresentation = ValueRepresentation { value: str_to_u8("UL") };
pub const UNKNOWN : ValueRepresentation = ValueRepresentation { value: str_to_u8("UN") };

pub const UNIVERSAL_RESOURCE : ValueRepresentation = ValueRepresentation { value: str_to_u8("UR") };
pub const UNSIGNED_SHORT : ValueRepresentation = ValueRepresentation { value: str_to_u8("US") };
pub const UNLIMITED_TEXT : ValueRepresentation = ValueRepresentation { value: str_to_u8("UT") };
pub const UNSIGNED_64_BIT_VERY_LONG : ValueRepresentation = ValueRepresentation { value: str_to_u8("UV") };