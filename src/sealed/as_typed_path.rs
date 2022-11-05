use crate::{Base, Separator, Str, TypedPath};

pub trait Sealed {
    #[doc(hidden)]
    type __Str: Str + ?Sized;

    #[doc(hidden)]
    type __Base: Base;

    #[doc(hidden)]
    type __Separator: Separator;

    #[doc(hidden)]
    fn __as_typed_path(&self) -> &TypedPath<Self::__Str, Self::__Base, Self::__Separator>;
}
