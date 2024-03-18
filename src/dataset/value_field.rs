
use crate::value_representations::attribute_tag::AttributeTag;
use crate::value_representations::other_64bit_very_long::Other64bitVeryLong;
use crate::value_representations::other_byte::OtherByte;
use crate::value_representations::other_double::OtherDouble;
use crate::value_representations::other_float::OtherFloat;
use crate::value_representations::other_long::OtherLong;
use crate::value_representations::other_word::OtherWord;
use crate::value_representations::sequence_of_items::SequenceOfItems;
use crate::value_representations::dicom_string::DicomString;
use crate::value_representations::numeric_type::NumericType;
use crate::value_representations::unknown::Unknown;

#[derive(Debug)]
pub enum ValueField {
    ApplicationEntity(DicomString),
    AgeString(DicomString),
    AttributeTag(AttributeTag),
    CodeString(DicomString),
    Date(DicomString),
    DateTime(DicomString),
    DecimalString(DicomString),
    FloatingPointSingle(NumericType<f32>),
    FloatingPointDouble(NumericType<f64>),
    IntegerString(DicomString),
    LongString(DicomString),
    LongText(DicomString),
    OtherByte(OtherByte),
    OtherDouble(OtherDouble),
    OtherFloat(OtherFloat),
    OtherLong(OtherLong),
    Other64bitVeryLong(Other64bitVeryLong),
    OtherWord(OtherWord),
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