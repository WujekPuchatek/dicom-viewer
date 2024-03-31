
use crate::value_representations::attribute_tag::AttributeTag;
use crate::value_representations::sequence_of_items::SequenceOfItems;
use crate::value_representations::dicom_string::DicomString;
use crate::value_representations::numeric_string::NumericString;
use crate::value_representations::numeric_type::NumericType;
use crate::value_representations::other_type::OtherType;
use crate::value_representations::unknown::Unknown;

#[derive(Debug)]
pub enum ValueField {
    ApplicationEntity(DicomString),
    AgeString(DicomString),
    AttributeTag(AttributeTag),
    CodeString(DicomString),
    Date(DicomString),
    DateTime(DicomString),
    DecimalString(NumericString),
    FloatingPointSingle(NumericType<f32>),
    FloatingPointDouble(NumericType<f64>),
    IntegerString(NumericString),
    LongString(DicomString),
    LongText(DicomString),
    OtherByte(OtherType<u8>),
    OtherDouble(OtherType<f64>),
    OtherFloat(OtherType<f32>),
    OtherLong(OtherType<i32>),
    Other64bitVeryLong(OtherType<i64>),
    OtherWord(OtherType<u16>),
    PersonName(DicomString),
    ShortString(DicomString),
    SignedLong(NumericType<i32>),
    SequenceOfItems(SequenceOfItems),
    SignedShort(NumericType<i16>),
    ShortText(DicomString),
    Signed64bitVeryLong(NumericType<i64>),
    Time(DicomString),
    UnlimitedCharacters(DicomString),
    UniqueIdentifier(DicomString),
    UnsignedLong(NumericType<u32>),
    Unknown(Unknown),
    UniversalResourceIdentifier(DicomString),
    UnsignedShort(NumericType<u16>),
    UnlimitedText(DicomString),
    Unsigned64bitVeryLong(NumericType<u64>),
}