use std::ffi::OsStr;

use crate::{Str, TypedPrefix};

pub trait Sealed<'a> {
    #[doc(hidden)]
    type __Str: Str + ?Sized;

    #[doc(hidden)]
    fn __into_typed_prefix(self) -> TypedPrefix<'a, Self::__Str>;
}

impl<'a> Sealed<'a> for std::path::Prefix<'a> {
    type __Str = OsStr;

    fn __into_typed_prefix(self) -> TypedPrefix<'a, Self::__Str> {
        match self {
            Self::Verbatim(x) => TypedPrefix::Verbatim(x),
            Self::VerbatimUNC(x, y) => TypedPrefix::VerbatimUNC(x, y),
            Self::VerbatimDisk(x) => TypedPrefix::VerbatimDisk(x),
            Self::DeviceNS(x) => TypedPrefix::DeviceNS(x),
            Self::UNC(x, y) => TypedPrefix::UNC(x, y),
            Self::Disk(x) => TypedPrefix::Disk(x),
        }
    }
}

impl<'a> Sealed<'a> for camino::Utf8Prefix<'a> {
    type __Str = str;

    fn __into_typed_prefix(self) -> TypedPrefix<'a, Self::__Str> {
        match self {
            Self::Verbatim(x) => TypedPrefix::Verbatim(x),
            Self::VerbatimUNC(x, y) => TypedPrefix::VerbatimUNC(x, y),
            Self::VerbatimDisk(x) => TypedPrefix::VerbatimDisk(x),
            Self::DeviceNS(x) => TypedPrefix::DeviceNS(x),
            Self::UNC(x, y) => TypedPrefix::UNC(x, y),
            Self::Disk(x) => TypedPrefix::Disk(x),
        }
    }
}
