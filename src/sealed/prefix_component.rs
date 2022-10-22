use std::ffi::OsStr;

use crate::{Str, TypedPrefix};

pub trait Sealed<'a> {
    #[doc(hidden)]
    type __Str: Str + ?Sized;

    #[doc(hidden)]
    fn __typed_kind(&self) -> TypedPrefix<'a, Self::__Str>;

    #[doc(hidden)]
    fn __as_os_str(&self) -> &'a OsStr;
}

impl<'a> Sealed<'a> for std::path::PrefixComponent<'a> {
    type __Str = OsStr;

    fn __typed_kind(&self) -> TypedPrefix<'a, Self::__Str> {
        match self.kind() {
            std::path::Prefix::Verbatim(x) => TypedPrefix::Verbatim(x),
            std::path::Prefix::VerbatimUNC(x, y) => TypedPrefix::VerbatimUNC(x, y),
            std::path::Prefix::VerbatimDisk(x) => TypedPrefix::VerbatimDisk(x),
            std::path::Prefix::DeviceNS(x) => TypedPrefix::DeviceNS(x),
            std::path::Prefix::UNC(x, y) => TypedPrefix::UNC(x, y),
            std::path::Prefix::Disk(x) => TypedPrefix::Disk(x),
        }
    }

    fn __as_os_str(&self) -> &'a OsStr {
        self.as_os_str()
    }
}
impl<'a> Sealed<'a> for camino::Utf8PrefixComponent<'a> {
    type __Str = str;

    fn __typed_kind(&self) -> TypedPrefix<'a, Self::__Str> {
        match self.kind() {
            camino::Utf8Prefix::Verbatim(x) => TypedPrefix::Verbatim(x),
            camino::Utf8Prefix::VerbatimUNC(x, y) => TypedPrefix::VerbatimUNC(x, y),
            camino::Utf8Prefix::VerbatimDisk(x) => TypedPrefix::VerbatimDisk(x),
            camino::Utf8Prefix::DeviceNS(x) => TypedPrefix::DeviceNS(x),
            camino::Utf8Prefix::UNC(x, y) => TypedPrefix::UNC(x, y),
            camino::Utf8Prefix::Disk(x) => TypedPrefix::Disk(x),
        }
    }

    fn __as_os_str(&self) -> &'a OsStr {
        self.as_os_str()
    }
}
