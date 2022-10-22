use std::{
    collections::TryReserveError,
    ffi::{OsStr, OsString},
};

use crate::Str;

pub trait Sealed {
    #[doc(hidden)]
    type __Str: Str + ?Sized;

    #[doc(hidden)]
    type __FromStdPathBuf: IntoResult<Ok = Self>;

    #[doc(hidden)]
    fn __new() -> Self;

    #[doc(hidden)]
    fn __from_camino_path_buf(path: camino::Utf8PathBuf) -> Self;

    #[doc(hidden)]
    fn __from_path_buf(path: std::path::PathBuf) -> Self::__FromStdPathBuf;

    #[doc(hidden)]
    fn __into_std_path_buf(self) -> std::path::PathBuf;

    #[doc(hidden)]
    fn __with_capacity(capacity: usize) -> Self;

    #[doc(hidden)]
    fn __as_path(&self) -> &<Self::__Str as Str>::Path;

    #[doc(hidden)]
    fn __push<P: AsRef<<Self::__Str as Str>::Path>>(&mut self, path: P);

    #[doc(hidden)]
    fn __pop(&mut self) -> bool;

    #[doc(hidden)]
    fn __set_file_name<S: AsRef<Self::__Str>>(&mut self, file_name: S);

    #[doc(hidden)]
    fn __set_extension<S: AsRef<Self::__Str>>(&mut self, extension: S) -> bool;

    #[doc(hidden)]
    fn __into_os_string(self) -> OsString;

    #[doc(hidden)]
    fn __into_boxed_path(self) -> Box<<Self::__Str as Str>::Path>;

    #[doc(hidden)]
    fn __capacity(&self) -> usize;

    #[doc(hidden)]
    fn __clear(&mut self);

    #[doc(hidden)]
    fn __reserve(&mut self, additional: usize);

    #[doc(hidden)]
    fn __try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError>;

    #[doc(hidden)]
    fn __reserve_exact(&mut self, additional: usize);

    #[doc(hidden)]
    fn __try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError>;

    #[doc(hidden)]
    fn __shrink_to_fit(&mut self);

    #[doc(hidden)]
    fn __shrink_to(&mut self, min_capacity: usize);
}

impl Sealed for std::path::PathBuf {
    type __Str = OsStr;
    type __FromStdPathBuf = Self;

    fn __new() -> Self {
        Self::new()
    }

    fn __from_camino_path_buf(path: camino::Utf8PathBuf) -> Self {
        path.into()
    }

    fn __from_path_buf(path: std::path::PathBuf) -> Self::__FromStdPathBuf {
        path
    }

    fn __into_std_path_buf(self) -> std::path::PathBuf {
        self
    }

    fn __with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn __as_path(&self) -> &<Self::__Str as Str>::Path {
        self.as_path()
    }

    fn __push<P: AsRef<<Self::__Str as Str>::Path>>(&mut self, path: P) {
        self.push(path);
    }

    fn __pop(&mut self) -> bool {
        self.pop()
    }

    fn __set_file_name<S: AsRef<Self::__Str>>(&mut self, file_name: S) {
        self.set_file_name(file_name);
    }

    fn __set_extension<S: AsRef<Self::__Str>>(&mut self, extension: S) -> bool {
        self.set_extension(extension)
    }

    fn __into_os_string(self) -> OsString {
        self.into_os_string()
    }

    fn __into_boxed_path(self) -> Box<<Self::__Str as Str>::Path> {
        self.into_boxed_path()
    }

    fn __capacity(&self) -> usize {
        self.capacity()
    }

    fn __clear(&mut self) {
        self.clear();
    }

    fn __reserve(&mut self, additional: usize) {
        self.reserve(additional);
    }

    fn __try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.try_reserve(additional)
    }

    fn __reserve_exact(&mut self, additional: usize) {
        self.reserve_exact(additional);
    }

    fn __try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.try_reserve_exact(additional)
    }

    fn __shrink_to_fit(&mut self) {
        self.shrink_to_fit();
    }

    fn __shrink_to(&mut self, min_capacity: usize) {
        self.shrink_to(min_capacity);
    }
}

impl Sealed for camino::Utf8PathBuf {
    type __Str = str;
    type __FromStdPathBuf = Result<Self, std::path::PathBuf>;

    fn __new() -> Self {
        Self::new()
    }

    fn __from_path_buf(path: std::path::PathBuf) -> Self::__FromStdPathBuf {
        Self::from_path_buf(path)
    }

    fn __from_camino_path_buf(path: camino::Utf8PathBuf) -> Self {
        path
    }

    fn __into_std_path_buf(self) -> std::path::PathBuf {
        self.into_std_path_buf()
    }

    fn __with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn __as_path(&self) -> &<Self::__Str as Str>::Path {
        self.as_path()
    }

    fn __push<P: AsRef<<Self::__Str as Str>::Path>>(&mut self, path: P) {
        self.push(path);
    }

    fn __pop(&mut self) -> bool {
        self.pop()
    }

    fn __set_file_name<S: AsRef<Self::__Str>>(&mut self, file_name: S) {
        self.set_file_name(file_name);
    }

    fn __set_extension<S: AsRef<Self::__Str>>(&mut self, extension: S) -> bool {
        self.set_extension(extension)
    }

    fn __into_os_string(self) -> OsString {
        self.into_os_string()
    }

    fn __into_boxed_path(self) -> Box<<Self::__Str as Str>::Path> {
        self.into_boxed_path()
    }

    fn __capacity(&self) -> usize {
        self.capacity()
    }

    fn __clear(&mut self) {
        self.clear()
    }

    fn __reserve(&mut self, additional: usize) {
        self.reserve(additional);
    }

    fn __try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.try_reserve(additional)
    }

    fn __reserve_exact(&mut self, additional: usize) {
        self.reserve_exact(additional);
    }

    fn __try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.try_reserve_exact(additional)
    }

    fn __shrink_to_fit(&mut self) {
        self.shrink_to_fit();
    }

    fn __shrink_to(&mut self, min_capacity: usize) {
        self.shrink_to(min_capacity);
    }
}

pub trait IntoResult {
    type Ok;
    fn into_result(self) -> Result<Self::Ok, std::path::PathBuf>;
}

impl IntoResult for std::path::PathBuf {
    type Ok = std::path::PathBuf;

    fn into_result(self) -> Result<Self::Ok, std::path::PathBuf> {
        Ok(self)
    }
}

impl IntoResult for Result<camino::Utf8PathBuf, std::path::PathBuf> {
    type Ok = camino::Utf8PathBuf;

    fn into_result(self) -> Result<<Self as IntoResult>::Ok, std::path::PathBuf> {
        self
    }
}
