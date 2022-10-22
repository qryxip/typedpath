use std::ffi::OsStr;

pub trait Sealed {
    #[doc(hidden)]
    type __Path: ?Sized;

    #[doc(hidden)]
    type __PathBuf;

    #[doc(hidden)]
    fn __from_str(s: &str) -> &Self;

    #[doc(hidden)]
    fn __as_ref_os_str(&self) -> &OsStr;

    #[doc(hidden)]
    fn __as_ref_path(&self) -> &Self::__Path;
}

impl Sealed for OsStr {
    type __Path = std::path::Path;
    type __PathBuf = std::path::PathBuf;

    fn __from_str(s: &str) -> &Self {
        s.as_ref()
    }

    fn __as_ref_os_str(&self) -> &OsStr {
        self
    }

    fn __as_ref_path(&self) -> &Self::__Path {
        self.as_ref()
    }
}

impl Sealed for str {
    type __Path = camino::Utf8Path;
    type __PathBuf = camino::Utf8PathBuf;

    fn __from_str(s: &str) -> &Self {
        s
    }

    fn __as_ref_os_str(&self) -> &OsStr {
        self.as_ref()
    }

    fn __as_ref_path(&self) -> &Self::__Path {
        self.as_ref()
    }
}
