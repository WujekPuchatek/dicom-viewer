use crate::dataset::tag::Tag;

pub const FILE_META_INFO_GROUP_LENGTH_TAG: Tag = Tag { group: 0x0002, element: 0x0000 };
pub const FILE_META_INFO_VERSION: Tag = Tag { group: 0x0002, element: 0x0001 };
pub const MEDIA_STORAGE_SOP_CLASS_UID: Tag = Tag { group: 0x0002, element: 0x0002 };
pub const MEDIA_STORAGE_SOP_INSTANCE_UID: Tag = Tag { group: 0x0002, element: 0x0003 };
pub const TRANSFER_SYNTAX_UID: Tag = Tag { group: 0x0002, element: 0x0010 };
pub const IMPLEMENTATION_CLASS_UID: Tag = Tag { group: 0x0002, element: 0x0012 };
pub const IMPLEMENTATION_VERSION_NAME: Tag = Tag { group: 0x0002, element: 0x0013 };
pub const TRANSFER_SYNTAX_UID_TAG: Tag = Tag { group: 0x0002, element: 0x0010 };
pub const IMPLEMENTATION_CLASS_UID_TAG: Tag = Tag { group: 0x0002, element: 0x0012 };
pub const IMPLEMENTATION_VERSION_NAME_TAG: Tag = Tag { group: 0x0002, element: 0x0013 };