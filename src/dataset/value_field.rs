
use crate::value_representations::attribute_tag::AttributeTag;
use crate::value_representations::extended_string::ExtendedString;
use crate::value_representations::floating_point_double::FloatingPointDouble;
use crate::value_representations::floating_point_single::FloatingPointSingle;
use crate::value_representations::other_64bit_very_long::Other64bitVeryLong;
use crate::value_representations::other_byte::OtherByte;
use crate::value_representations::other_double::OtherDouble;
use crate::value_representations::other_float::OtherFloat;
use crate::value_representations::other_long::OtherLong;
use crate::value_representations::other_word::OtherWord;
use crate::value_representations::sequence_of_items::SequenceOfItems;
use crate::value_representations::signed_64bit_very_long::Signed64bitVeryLong;
use crate::value_representations::signed_long::SignedLong;
use crate::value_representations::signed_short::SignedShort;
use crate::value_representations::standard_string::StandardString;
use crate::value_representations::unknown::Unknown;
use crate::value_representations::unsigned_64bit_very_long::Unsigned64bitVeryLong;
use crate::value_representations::unsigned_long::UnsignedLong;
use crate::value_representations::unsigned_short::UnsignedShort;

#[derive(Debug, Clone)]
pub enum ValueField {
    ApplicationEntity(StandardString),
    AgeString(StandardString),
    AttributeTag(AttributeTag),
    CodeString(StandardString),
    Date(StandardString),
    DateTime(StandardString),
    DecimalString(StandardString),
    FloatingPointSingle(FloatingPointSingle),
    FloatingPointDouble(FloatingPointDouble),
    IntegerString(StandardString),
    LongString(ExtendedString),
    LongText(ExtendedString),
    OtherByte(OtherByte),
    OtherDouble(OtherDouble),
    OtherFloat(OtherFloat),
    OtherLong(OtherLong),
    Other64bitVeryLong(Other64bitVeryLong),
    OtherWord(OtherWord),
    PersonName(ExtendedString),
    ShortString(ExtendedString),
    SignedLong(SignedLong),
    SequenceOfItems(SequenceOfItems),
    SignedShort(SignedShort),
    ShortText(ExtendedString),
    Signed64bitVeryLong(Signed64bitVeryLong),
    Time(StandardString),
    UnlimitedCharacters(ExtendedString),
    UniqueIdentifier(StandardString),
    UnsignedLong(UnsignedLong),
    Unknown(Unknown),
    UniversalResourceIdentifier(StandardString),
    UnsignedShort(UnsignedShort),
    UnlimitedText(ExtendedString),
    Unsigned64bitVeryLong(Unsigned64bitVeryLong),
}