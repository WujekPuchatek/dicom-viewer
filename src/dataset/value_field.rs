use crate::value_representations::age_string::AgeString;
use crate::value_representations::application_entity::ApplicationEntity;
use crate::value_representations::attribute_tag::AttributeTag;
use crate::value_representations::code_string::CodeString;
use crate::value_representations::date::Date;
use crate::value_representations::date_time::DateTime;
use crate::value_representations::decimal_string::DecimalString;
use crate::value_representations::floating_point_double::FloatingPointDouble;
use crate::value_representations::floating_point_single::FloatingPointSingle;
use crate::value_representations::integer_string::IntegerString;
use crate::value_representations::long_string::LongString;
use crate::value_representations::long_text::LongText;
use crate::value_representations::other_64bit_very_long::Other64bitVeryLong;
use crate::value_representations::other_byte::OtherByte;
use crate::value_representations::other_double::OtherDouble;
use crate::value_representations::other_float::OtherFloat;
use crate::value_representations::other_long::OtherLong;
use crate::value_representations::other_word::OtherWord;
use crate::value_representations::person_name::PersonName;
use crate::value_representations::sequence_of_items::SequenceOfItems;
use crate::value_representations::short_string::ShortString;
use crate::value_representations::short_text::ShortText;
use crate::value_representations::signed_64bit_very_long::Signed64bitVeryLong;
use crate::value_representations::signed_long::SignedLong;
use crate::value_representations::signed_short::SignedShort;
use crate::value_representations::time::Time;
use crate::value_representations::unique_identifier::UniqueIdentifier;
use crate::value_representations::universal_resource_identifier::UniversalResourceIdentifier;
use crate::value_representations::unknown::Unknown;
use crate::value_representations::unlimited_characters::UnlimitedCharacters;
use crate::value_representations::unlimited_text::UnlimitedText;
use crate::value_representations::unsigned_64bit_very_long::Unsigned64bitVeryLong;
use crate::value_representations::unsigned_long::UnsignedLong;
use crate::value_representations::unsigned_short::UnsignedShort;

#[derive(Debug, Clone)]
pub enum ValueField {
    ApplicationEntity(ApplicationEntity),
    AgeString(AgeString),
    AttributeTag(AttributeTag),
    CodeString(CodeString),
    Date(Date),
    DateTime(DateTime),
    DecimalString(DecimalString),
    FloatingPointSingle(FloatingPointSingle),
    FloatingPointDouble(FloatingPointDouble),
    IntegerString(IntegerString),
    LongString(LongString),
    LongText(LongText),
    OtherByte(OtherByte),
    OtherDouble(OtherDouble),
    OtherFloat(OtherFloat),
    OtherLong(OtherLong),
    Other64bitVeryLong(Other64bitVeryLong),
    OtherWord(OtherWord),
    PersonName(PersonName),
    ShortString(ShortString),
    SignedLong(SignedLong),
    SequenceOfItems(SequenceOfItems),
    SignedShort(SignedShort),
    ShortText(ShortText),
    Signed64bitVeryLong(Signed64bitVeryLong),
    Time(Time),
    UnlimitedCharacters(UnlimitedCharacters),
    UniqueIdentifier(UniqueIdentifier),
    UnsignedLong(UnsignedLong),
    Unknown(Unknown),
    UniversalResourceIdentifier(UniversalResourceIdentifier),
    UnsignedShort(UnsignedShort),
    UnlimitedText(UnlimitedText),
    Unsigned64bitVeryLong(Unsigned64bitVeryLong),
}