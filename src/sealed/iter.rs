use std::ffi::OsStr;

use crate::Str;

pub trait Sealed<'a> {
    type __Str: Str + ?Sized;
    fn as_path(&self) -> &'a <Self::__Str as Str>::Path;
}

impl<'a> Sealed<'a> for std::path::Iter<'a> {
    type __Str = OsStr;

    fn as_path(&self) -> &'a <Self::__Str as Str>::Path {
        self.as_path()
    }
}

impl<'a> Sealed<'a> for camino::Iter<'a> {
    type __Str = str;

    fn as_path(&self) -> &'a <Self::__Str as Str>::Path {
        self.as_path()
    }
}
