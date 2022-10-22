use std::{ffi::OsStr, path::StripPrefixError};

use crate::Str;

pub trait Sealed: 'static {
    #[doc(hidden)]
    type __Str: Str + ?Sized;

    #[doc(hidden)]
    fn __new<S: AsRef<Self::__Str> + ?Sized>(s: &S) -> &Self;

    #[doc(hidden)]
    fn __from_path(path: &std::path::Path) -> Option<&Self>;

    #[doc(hidden)]
    fn __as_std_path(&self) -> &std::path::Path;

    #[doc(hidden)]
    fn __parent(&self) -> Option<&Self>;

    #[doc(hidden)]
    fn __ancestors(&self) -> <Self::__Str as Str>::Ancestors<'_>;

    #[doc(hidden)]
    fn __file_name(&self) -> Option<&Self::__Str>;

    #[doc(hidden)]
    fn __strip_prefix<P>(&self, base: P) -> Result<&Self, StripPrefixError>
    where
        P: AsRef<std::path::Path>;

    #[doc(hidden)]
    fn __file_stem(&self) -> Option<&Self::__Str>;

    #[doc(hidden)]
    fn __extension(&self) -> Option<&Self::__Str>;

    #[doc(hidden)]
    fn __join<P: AsRef<Self>>(&self, path: P) -> <Self::__Str as Str>::PathBuf;

    #[doc(hidden)]
    fn __join_os(&self, path: impl AsRef<std::path::Path>) -> std::path::PathBuf;

    #[doc(hidden)]
    fn __join_utf8(&self, path: impl AsRef<camino::Utf8Path>) -> <Self::__Str as Str>::PathBuf;

    #[doc(hidden)]
    fn __with_file_name<S: AsRef<Self::__Str>>(
        &self,
        file_name: S,
    ) -> <Self::__Str as Str>::PathBuf;

    #[doc(hidden)]
    fn __with_extension<S: AsRef<Self::__Str>>(
        &self,
        extension: S,
    ) -> <Self::__Str as Str>::PathBuf;

    #[doc(hidden)]
    fn __components(&self) -> <Self::__Str as Str>::Components<'_>;

    #[doc(hidden)]
    fn __iter(&self) -> <Self::__Str as Str>::Iter<'_>;

    #[doc(hidden)]
    fn __to_path_buf(&self) -> <Self::__Str as Str>::PathBuf;

    #[doc(hidden)]
    fn __into_path_buf(self: Box<Self>) -> <Self::__Str as Str>::PathBuf;
}

impl Sealed for std::path::Path {
    type __Str = OsStr;

    fn __new<S: AsRef<Self::__Str> + ?Sized>(s: &S) -> &Self {
        Self::new(s)
    }

    fn __from_path(path: &std::path::Path) -> Option<&Self> {
        Some(path)
    }

    fn __as_std_path(&self) -> &std::path::Path {
        self
    }

    fn __parent(&self) -> Option<&Self> {
        self.parent()
    }

    fn __ancestors(&self) -> <Self::__Str as Str>::Ancestors<'_> {
        self.ancestors()
    }

    fn __file_name(&self) -> Option<&Self::__Str> {
        self.file_name()
    }

    fn __strip_prefix<P>(&self, base: P) -> Result<&Self, StripPrefixError>
    where
        P: AsRef<std::path::Path>,
    {
        self.strip_prefix(base)
    }

    fn __file_stem(&self) -> Option<&Self::__Str> {
        self.file_stem()
    }

    fn __extension(&self) -> Option<&Self::__Str> {
        self.extension()
    }

    fn __join<P: AsRef<Self>>(&self, path: P) -> <Self::__Str as Str>::PathBuf {
        self.join(path)
    }

    fn __join_os(&self, path: impl AsRef<std::path::Path>) -> std::path::PathBuf {
        self.join(path)
    }

    fn __join_utf8(&self, path: impl AsRef<camino::Utf8Path>) -> <Self::__Str as Str>::PathBuf {
        self.join(path.as_ref())
    }

    fn __with_file_name<S: AsRef<Self::__Str>>(
        &self,
        file_name: S,
    ) -> <Self::__Str as Str>::PathBuf {
        self.with_file_name(file_name)
    }

    fn __with_extension<S: AsRef<Self::__Str>>(
        &self,
        extension: S,
    ) -> <Self::__Str as Str>::PathBuf {
        self.with_extension(extension)
    }

    fn __components(&self) -> <Self::__Str as Str>::Components<'_> {
        self.components()
    }

    fn __iter(&self) -> <Self::__Str as Str>::Iter<'_> {
        self.iter()
    }

    fn __to_path_buf(&self) -> <Self::__Str as Str>::PathBuf {
        self.to_path_buf()
    }

    fn __into_path_buf(self: Box<Self>) -> <Self::__Str as Str>::PathBuf {
        Self::into_path_buf(self)
    }
}

impl Sealed for camino::Utf8Path {
    type __Str = str;

    fn __new<S: AsRef<Self::__Str> + ?Sized>(s: &S) -> &Self {
        Self::new(s)
    }

    fn __from_path(path: &std::path::Path) -> Option<&Self> {
        Self::from_path(path)
    }

    fn __as_std_path(&self) -> &std::path::Path {
        self.as_std_path()
    }

    fn __parent(&self) -> Option<&Self> {
        self.parent()
    }

    fn __ancestors(&self) -> <Self::__Str as Str>::Ancestors<'_> {
        self.ancestors()
    }

    fn __file_name(&self) -> Option<&Self::__Str> {
        self.file_name()
    }

    fn __strip_prefix<P>(&self, base: P) -> Result<&Self, StripPrefixError>
    where
        P: AsRef<std::path::Path>,
    {
        self.strip_prefix(base)
    }

    fn __file_stem(&self) -> Option<&Self::__Str> {
        self.file_stem()
    }

    fn __extension(&self) -> Option<&Self::__Str> {
        self.extension()
    }

    fn __join<P: AsRef<Self>>(&self, path: P) -> <Self::__Str as Str>::PathBuf {
        self.join(path)
    }

    fn __join_os(&self, path: impl AsRef<std::path::Path>) -> std::path::PathBuf {
        self.join_os(path)
    }

    fn __join_utf8(&self, path: impl AsRef<camino::Utf8Path>) -> <Self::__Str as Str>::PathBuf {
        self.join(path)
    }

    fn __with_file_name<S: AsRef<Self::__Str>>(
        &self,
        file_name: S,
    ) -> <Self::__Str as Str>::PathBuf {
        self.with_file_name(file_name)
    }

    fn __with_extension<S: AsRef<Self::__Str>>(
        &self,
        extension: S,
    ) -> <Self::__Str as Str>::PathBuf {
        self.with_extension(extension)
    }

    fn __components(&self) -> <Self::__Str as Str>::Components<'_> {
        self.components()
    }

    fn __iter(&self) -> <Self::__Str as Str>::Iter<'_> {
        self.iter()
    }

    fn __to_path_buf(&self) -> <Self::__Str as Str>::PathBuf {
        self.to_path_buf()
    }

    fn __into_path_buf(self: Box<Self>) -> <Self::__Str as Str>::PathBuf {
        Self::into_path_buf(self)
    }
}
