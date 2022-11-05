use std::{
    borrow::Cow,
    convert,
    ffi::{OsStr, OsString},
};

use crate::{
    sealed::{self, separator::Sealed as _},
    Abs, Base, MainSeparator, Rel, Separator, Slash, Str, TypedPath, TypedPathBuf,
};

pub trait StrJoinPath<Other: Str + ?Sized>: Str {
    type Output: Str + ?Sized;

    #[doc(hidden)]
    fn __join_path(this: &Self::Path, other: &Other::Path) -> <Self::Output as Str>::PathBuf;
}

macro_rules! impl_str_join_path {
    ($(
        ($this:ty, $other:ty) -> $output:ty { $f:expr }
    )*) => {
        $(
            impl StrJoinPath<$other> for $this {
                type Output = $output;

                fn __join_path(
                    this: &Self::Path,
                    other: &<$other as Str>::Path,
                ) -> <Self::Output as Str>::PathBuf {
                    apply2($f, this, other)
                }
            }
        )*
    };
}

impl_str_join_path! {
    (OsStr, OsStr) -> OsStr { std::path::Path::join     }
    (OsStr, str  ) -> OsStr { std::path::Path::join     }
    (str  , OsStr) -> OsStr { camino::Utf8Path::join_os }
    (str  , str  ) -> str   { camino::Utf8Path::join    }
}

pub trait BaseJoin<Other: Base>: Base {
    type Output: Base;
}

macro_rules! impl_base_join {
    ($(
        ($this:ty, $other:ty) -> $output:ty;
    )*) => {
        $(
            impl BaseJoin<$other> for $this {
                type Output = $output;
            }
        )*
    };
}

impl_base_join! {
    (() , () ) -> () ;
    (() , Rel) -> () ;
    (() , Abs) -> Abs;
    (Rel, () ) -> () ;
    (Rel, Rel) -> Rel;
    (Rel, Abs) -> Abs;
    (Abs, () ) -> Abs;
    (Abs, Rel) -> Abs;
    (Abs, Abs) -> Abs;
}

pub trait SeparatorJoin<Other: Separator>: Separator {
    type Output: Separator;
}

macro_rules! impl_separator_join {
    ($(
        ($this:ty, $other:ty) -> $output:ty;
    )*) => {
        $(
            impl SeparatorJoin<$other> for $this {
                type Output = $output;
            }
        )*
    };
}

impl_separator_join! {
    (()           , ()           ) -> ()           ;
    (()           , MainSeparator) -> ()           ;
    (()           , Slash        ) -> ()           ;
    (MainSeparator, ()           ) -> ()           ;
    (MainSeparator, MainSeparator) -> MainSeparator;
    (MainSeparator, Slash        ) -> ()           ;
    (Slash        , ()           ) -> ()           ;
    (Slash        , MainSeparator) -> ()           ;
    (Slash        , Slash        ) -> Slash        ;
}

pub trait AsTypedPath:
    sealed::as_typed_path::Sealed<__Str = Self::Str, __Base = Self::Base, __Separator = Self::Separator>
{
    type Str: Str + ?Sized;
    type Base: Base;
    type Separator: Separator;
}

macro_rules! impl_as_typed_path {
    ($(
        for<$($generic_param_name:ident $(: {$($generic_param_bounds:tt)*})?),*> (
            $self:ty
        ) -> &_<$str:ty, $base:ty, $separator:ty> {
            $f:expr
        }
    )*) => {
        $(
            impl<$($generic_param_name$(: $($generic_param_bounds)*)?),*> AsTypedPath for $self {
                type Str = $str;
                type Base = $base;
                type Separator = $separator;
            }

            impl<$($generic_param_name$(: $($generic_param_bounds)*)?),*>
                sealed::as_typed_path::Sealed for $self
            {
                type __Str = $str;
                type __Base = $base;
                type __Separator = $separator;

                fn __as_typed_path(
                    &self,
                ) -> &TypedPath<Self::__Str, Self::__Base, Self::__Separator> {
                    apply1($f, self)
                }
            }
        )*
    };
}

impl_as_typed_path! {
    for<P: {AsTypedPath + ?Sized}                   > (&'_ P                      ) -> &_<P::Str, P::Base, P::Separator> { |p| (**p).__as_typed_path() }
    for<P: {AsTypedPath + ?Sized}                   > (&'_ mut P                  ) -> &_<P::Str, P::Base, P::Separator> { |p| (**p).__as_typed_path() }
    for<S: {Str + ?Sized}, B: {Base}, D: {Separator}> (TypedPath<S, B, D>         ) -> &_<S     , B      , D           > { convert::identity           }
    for<S: {Str + ?Sized}, B: {Base}, D: {Separator}> (Box<TypedPath<S, B, D>>    ) -> &_<S     , B      , D           > { convert::identity           }
    for<S: {Str + ?Sized}, B: {Base}, D: {Separator}> (Cow<'_, TypedPath<S, B, D>>) -> &_<S     , B      , D           > { convert::identity           }
    for<S: {Str + ?Sized}, B: {Base}, D: {Separator}> (TypedPathBuf<S, B, D>      ) -> &_<S     , B      , D           > { TypedPathBuf::as_path       }
    for<                                            > (OsStr                      ) -> &_<OsStr , ()     , ()          > { TypedPath::new              }
    for<                                            > (Box<OsStr>                 ) -> &_<OsStr , ()     , ()          > { TypedPath::new              }
    for<                                            > (Cow<'_, OsStr>             ) -> &_<OsStr , ()     , ()          > { TypedPath::new              }
    for<                                            > (OsString                   ) -> &_<OsStr , ()     , ()          > { TypedPath::new              }
    for<                                            > (std::path::Path            ) -> &_<OsStr , ()     , ()          > { TypedPath::new              }
    for<                                            > (Box<std::path::Path>       ) -> &_<OsStr , ()     , ()          > { |p| TypedPath::new(&**p)    }
    for<                                            > (Cow<'_, std::path::Path>   ) -> &_<OsStr , ()     , ()          > { |p| TypedPath::new(&**p)    }
    for<                                            > (std::path::PathBuf         ) -> &_<OsStr , ()     , ()          > { TypedPath::new              }
    for<                                            > (str                        ) -> &_<str   , ()     , ()          > { TypedPath::new              }
    for<                                            > (Box<str>                   ) -> &_<str   , ()     , ()          > { TypedPath::new              }
    for<                                            > (Cow<'_, str>               ) -> &_<str   , ()     , ()          > { TypedPath::new              }
    for<                                            > (String                     ) -> &_<str   , ()     , ()          > { TypedPath::new              }
    for<                                            > (camino::Utf8Path           ) -> &_<str   , ()     , ()          > { TypedPath::new              }
    for<                                            > (Box<camino::Utf8Path>      ) -> &_<str   , ()     , ()          > { |p| TypedPath::new(&**p)    }
    for<                                            > (Cow<'_, camino::Utf8Path>  ) -> &_<str   , ()     , ()          > { |p| TypedPath::new(&**p)    }
    for<                                            > (camino::Utf8PathBuf        ) -> &_<str   , ()     , ()          > { TypedPath::new              }
}

pub trait Push<S: Str + ?Sized, B: Base, D: Separator>: sealed::push::Sealed {
    #[doc(hidden)]
    fn __push(&mut self, path: &S::Path);
}

macro_rules! impl_push {
    ($(
        for<$($generic_param_name:ident $(: {$($generic_param_bounds:tt)*})?),*> (
            &mut _<$str1:ty, $base1:ty, $separator1:ty>,
            &_<$str2:ty, $base2:ty, $separator2:ty>
        ) {
            $f:expr
        }
    )*) => {
        $(
            impl<$($generic_param_name$(: $($generic_param_bounds)*)?),*>
                Push<$str2, $base2, $separator2> for TypedPathBuf<$str1, $base1, $separator1>
            {
                fn __push(&mut self, path: &<$str2 as Str>::Path) {
                    apply2($f, &mut self.inner, path);
                }
            }

        )*
    };
}

impl_push! {
    for<S: {Str + ?Sized}, B: {Base}, D: {Separator}> (&mut _<OsStr, () , ()           >, &_<S  , B  , D            >) { |b, p| b.push(sealed::path::Sealed::__as_std_path(p)) }
    for<S: {Str + ?Sized}, B: {Base}                > (&mut _<OsStr, () , MainSeparator>, &_<S  , B  , MainSeparator>) { |b, p| b.push(sealed::path::Sealed::__as_std_path(p)) }
    for<S: {Str + ?Sized}           , D: {Separator}> (&mut _<OsStr, Rel, ()           >, &_<S  , Rel, D            >) { |b, p| b.push(sealed::path::Sealed::__as_std_path(p)) }
    for<S: {Str + ?Sized}                           > (&mut _<OsStr, Rel, MainSeparator>, &_<S  , Rel, MainSeparator>) { |b, p| b.push(sealed::path::Sealed::__as_std_path(p)) }
    for<S: {Str + ?Sized}, B: {Base}, D: {Separator}> (&mut _<OsStr, Abs, ()           >, &_<S  , B  , D            >) { |b, p| b.push(sealed::path::Sealed::__as_std_path(p)) }
    for<S: {Str + ?Sized}, B: {Base}                > (&mut _<OsStr, Abs, MainSeparator>, &_<S  , B  , MainSeparator>) { |b, p| b.push(sealed::path::Sealed::__as_std_path(p)) }
    for<                   B: {Base}, D: {Separator}> (&mut _<str  , () , ()           >, &_<str, B  , D            >) { camino::Utf8PathBuf::push                             }
    for<                   B: {Base}, D: {Separator}> (&mut _<str  , () , MainSeparator>, &_<str, B  , D            >) { |b, p| { b.push(p); MainSeparator::__normalize(b); }  }
    for<                   B: {Base}, D: {Separator}> (&mut _<str  , () , Slash        >, &_<str, B  , D            >) { |b, p| { b.push(p); Slash::__normalize(b); }          }
    for<                              D: {Separator}> (&mut _<str  , Rel, ()           >, &_<str, Rel, D            >) { camino::Utf8PathBuf::push                             }
    for<                              D: {Separator}> (&mut _<str  , Rel, MainSeparator>, &_<str, Rel, D            >) { |b, p| { b.push(p); MainSeparator::__normalize(b); }  }
    for<                              D: {Separator}> (&mut _<str  , Rel, Slash        >, &_<str, Rel, D            >) { |b, p| { b.push(p); Slash::__normalize(b); }          }
    for<                   B: {Base}, D: {Separator}> (&mut _<str  , Abs, ()           >, &_<str, B  , D            >) { camino::Utf8PathBuf::push                             }
    for<                   B: {Base}, D: {Separator}> (&mut _<str  , Abs, MainSeparator>, &_<str, B  , D            >) { |b, p| { b.push(p); MainSeparator::__normalize(b); }  }
    for<                   B: {Base}, D: {Separator}> (&mut _<str  , Abs, Slash        >, &_<str, B  , D            >) { |b, p| { b.push(p); Slash::__normalize(b); }          }
}

impl<S: Str + ?Sized, B, D> sealed::push::Sealed for TypedPathBuf<S, B, D> {}

fn apply1<F: FnOnce(X) -> O, X, O>(f: F, x: X) -> O {
    f(x)
}

fn apply2<F: FnOnce(X, Y) -> O, X, Y, O>(f: F, x: X, y: Y) -> O {
    f(x, y)
}
