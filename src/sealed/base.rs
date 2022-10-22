pub trait Sealed {
    #[doc(hidden)]
    fn __check(path: &std::path::Path) -> crate::Result<()>;
}
