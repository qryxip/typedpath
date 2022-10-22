pub trait Sealed {
    #[doc(hidden)]
    fn __normalize(path: &mut camino::Utf8PathBuf);
}
