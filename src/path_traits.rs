use std::{
    ffi::OsStr,
    fmt::Debug,
    hash::Hash,
    panic::{RefUnwindSafe, UnwindSafe},
};

use crate::{sealed, Str};

pub trait Path:
    'static
    + Ord
    + Hash
    + Debug
    + Send
    + Sync
    + Unpin
    + UnwindSafe
    + RefUnwindSafe
    + sealed::path::Sealed<__Str = Self::Str>
{
    type Str: Str<Path = Self> + ?Sized;
}

impl Path for std::path::Path {
    type Str = OsStr;
}

impl Path for camino::Utf8Path {
    type Str = str;
}

pub trait PathBuf:
    'static
    + Clone
    + Ord
    + Hash
    + Debug
    + Send
    + Sync
    + Unpin
    + UnwindSafe
    + RefUnwindSafe
    + sealed::path_buf::Sealed<__Str = Self::Str>
{
    type Str: Str<PathBuf = Self> + ?Sized;
}

impl PathBuf for std::path::PathBuf {
    type Str = OsStr;
}

impl PathBuf for camino::Utf8PathBuf {
    type Str = str;
}

pub trait Prefix<'a>: sealed::prefix::Sealed<'a> {}

impl<'a> Prefix<'a> for std::path::Prefix<'a> {}
impl<'a> Prefix<'a> for camino::Utf8Prefix<'a> {}

pub trait PrefixComponent<'a>: sealed::prefix_component::Sealed<'a, __Str = Self::Str> {
    type Str: Str + ?Sized;
}

impl<'a> PrefixComponent<'a> for std::path::PrefixComponent<'a> {
    type Str = OsStr;
}

impl<'a> PrefixComponent<'a> for camino::Utf8PrefixComponent<'a> {
    type Str = str;
}

pub trait Component<'a>: sealed::component::Sealed<'a> {}

impl<'a> Component<'a> for std::path::Component<'a> {}
impl<'a> Component<'a> for camino::Utf8Component<'a> {}

pub trait Components<'a>: sealed::components::Sealed<'a, __Str = Self::Str> {
    type Str: Str + ?Sized;
}

impl<'a> Components<'a> for std::path::Components<'a> {
    type Str = OsStr;
}

impl<'a> Components<'a> for camino::Utf8Components<'a> {
    type Str = str;
}

pub trait Iter<'a>: sealed::iter::Sealed<'a, __Str = Self::Str> {
    type Str: Str<Iter<'a> = Self> + ?Sized;
}

impl<'a> Iter<'a> for std::path::Iter<'a> {
    type Str = OsStr;
}

impl<'a> Iter<'a> for camino::Iter<'a> {
    type Str = str;
}

pub trait Ancestors<'a>: Copy + Debug + sealed::ancestors::Sealed<'a> {}

impl<'a> Ancestors<'a> for std::path::Ancestors<'a> {}
impl<'a> Ancestors<'a> for camino::Utf8Ancestors<'a> {}
