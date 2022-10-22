use std::ffi::OsStr;

use crate::Str;

pub trait Sealed<'a> {
    #[doc(hidden)]
    type __Str: Str + ?Sized;

    #[doc(hidden)]
    fn __as_path(&self) -> &'a <Self::__Str as Str>::Path;
}

impl<'a> Sealed<'a> for std::path::Components<'a> {
    type __Str = OsStr;

    fn __as_path(&self) -> &'a <Self::__Str as Str>::Path {
        self.as_path()
    }
}

impl<'a> Sealed<'a> for camino::Utf8Components<'a> {
    type __Str = str;

    fn __as_path(&self) -> &'a <Self::__Str as Str>::Path {
        self.as_path()
    }
}
