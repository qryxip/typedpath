#![warn(rust_2018_idioms)]

use std::{
    borrow::{Borrow, Cow},
    ffi::OsStr,
    fmt::{self, Debug, Display},
    fs::{Metadata, ReadDir},
    hash::Hash,
    io,
    marker::PhantomData,
    path::StripPrefixError,
    ptr,
};

use path_slash::{CowExt as _, PathExt as _};

use crate::{
    operations::{AsTypedPath, BaseJoin, Push, SeparatorJoin, StrJoinPath},
    sealed::{
        components::Sealed as _,
        iter::Sealed as _,
        path::Sealed as _,
        path_buf::{IntoResult as _, Sealed as _},
        prefix_component::Sealed as _,
    },
};

pub mod operations;
pub mod path_traits;
mod sealed;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct TypedPath<S: Str + ?Sized, B, D> {
    marker: PhantomData<fn() -> (B, D)>,
    inner: S::Path,
}

impl<S: Str + ?Sized, B, D> TypedPath<S, B, D> {
    pub fn as_os_str(&self) -> &OsStr {
        self.inner.__as_std_path().as_os_str()
    }

    pub fn to_path_buf(&self) -> TypedPathBuf<S, B, D> {
        TypedPathBuf::from_inner(self.inner.__to_path_buf())
    }

    pub fn is_absolute(&self) -> bool {
        self.inner.__as_std_path().is_absolute()
    }

    pub fn is_relative(&self) -> bool {
        self.inner.__as_std_path().is_relative()
    }

    pub fn has_root(&self) -> bool {
        self.inner.__as_std_path().has_root()
    }

    pub fn parent(&self) -> Option<&Self> {
        self.inner.__parent().map(Self::from_inner)
    }

    pub fn ancestors(&self) -> TypedAncestors<'_, S, B, D> {
        TypedAncestors::from_inner(self.inner.__ancestors())
    }

    pub fn file_name(&self) -> Option<&S> {
        self.inner.__file_name()
    }

    pub fn strip_prefix<P: AsRef<std::path::Path>>(
        &self,
        base: P,
    ) -> std::result::Result<&TypedPath<S, (), D>, StripPrefixError> {
        let inner = self.inner.__strip_prefix(base)?;
        Ok(TypedPath::from_inner(inner))
    }

    pub fn starts_with<P: AsRef<std::path::Path>>(&self, base: P) -> bool {
        self.inner.__as_std_path().starts_with(base)
    }

    pub fn ends_with<P: AsRef<std::path::Path>>(&self, base: P) -> bool {
        self.inner.__as_std_path().ends_with(base)
    }

    pub fn file_stem(&self) -> Option<&S> {
        self.inner.__file_stem()
    }

    pub fn extension(&self) -> Option<&S> {
        self.inner.__extension()
    }

    // FIXME
    pub fn with_file_name<S_: AsRef<S>>(&self, file_name: S_) -> S::PathBuf {
        self.inner.__with_file_name(file_name)
    }

    // FIXME
    pub fn with_extension<S_: AsRef<S>>(&self, extension: S_) -> S::PathBuf {
        self.inner.__with_extension(extension)
    }

    pub fn components(&self) -> TypedComponents<'_, S, D> {
        TypedComponents::from_inner(self.inner.__components())
    }

    pub fn iter(&self) -> S::Iter<'_> {
        self.inner.__iter()
    }

    pub fn metadata(&self) -> io::Result<Metadata> {
        self.inner.__as_std_path().metadata()
    }

    pub fn symlink_metadata(&self) -> io::Result<Metadata> {
        self.inner.__as_std_path().symlink_metadata()
    }

    pub fn canonicalize(&self) -> io::Result<std::path::PathBuf> {
        self.inner.__as_std_path().canonicalize()
    }

    pub fn read_link(&self) -> io::Result<std::path::PathBuf> {
        self.inner.__as_std_path().read_link()
    }

    pub fn read_dir(&self) -> io::Result<ReadDir> {
        self.inner.__as_std_path().read_dir()
    }

    pub fn exists(&self) -> bool {
        self.inner.__as_std_path().exists()
    }

    pub fn try_exists(&self) -> io::Result<bool> {
        self.inner.__as_std_path().try_exists()
    }

    pub fn is_file(&self) -> bool {
        self.inner.__as_std_path().is_file()
    }

    pub fn is_dir(&self) -> bool {
        self.inner.__as_std_path().is_dir()
    }

    pub fn is_symlink(&self) -> bool {
        self.inner.__as_std_path().is_symlink()
    }

    pub fn into_path_buf(self: Box<Self>) -> TypedPathBuf<S, B, D> {
        TypedPathBuf::from_inner(self.into_inner_owned())
    }

    fn from_inner(inner: &S::Path) -> &Self {
        // # Safety
        //
        // It's `repr` is `transparent`.
        unsafe { &*(ptr::addr_of!(*inner) as *const Self) }
    }

    fn into_inner_owned(self: Box<Self>) -> S::PathBuf {
        let inner = Box::into_raw(self) as *mut S::Path;
        // # Safety
        //
        // It's `repr` is `transparent`.
        unsafe { Box::from_raw(inner) }.__into_path_buf()
    }
}

impl<S: Str + ?Sized, B: Base, D: Separator> TypedPath<S, B, D> {
    pub fn join<P, S2, B2, D2>(&self, path: P) -> TypedPathBuf<S::Output, B::Output, D::Output>
    where
        S: StrJoinPath<S2>,
        B: BaseJoin<B2>,
        D: SeparatorJoin<D2>,
        P: AsTypedPath<Str = S2, Base = B2, Separator = D2>,
        S2: Str + ?Sized,
        B2: Base,
        D2: Separator,
    {
        TypedPathBuf::from_inner(S::__join_path(&self.inner, &path.__as_typed_path().inner))
    }
}

impl<S: Str + ?Sized> TypedPath<S, (), ()> {
    pub fn new<S_: AsRef<S> + ?Sized>(s: &S_) -> &Self {
        Self::from_inner(S::Path::__new(s))
    }
}

impl<S: Str + ?Sized, B: Base> TypedPath<S, B, ()> {
    pub fn from_std_path(path: &std::path::Path) -> crate::Result<&Self> {
        B::__check(path)?;
        let inner =
            S::Path::__from_path(path).ok_or_else(|| crate::Error::new(path, ErrorKind::Utf8))?;
        Ok(Self::from_inner(inner))
    }

    pub fn from_camino_path(path: &camino::Utf8Path) -> crate::Result<&Self> {
        B::__check(path.as_ref())?;
        let inner = S::__from_str(path.as_ref()).__as_ref_path();
        Ok(Self::from_inner(inner))
    }
}

impl<B, D> TypedPath<OsStr, B, D> {
    pub fn to_str(&self) -> Option<&str> {
        self.inner.to_str()
    }

    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        self.inner.to_string_lossy()
    }

    pub fn display(&self) -> std::path::Display<'_> {
        self.inner.display()
    }
}

impl<S: Str + ?Sized, B, D> ToOwned for TypedPath<S, B, D> {
    type Owned = TypedPathBuf<S, B, D>;

    fn to_owned(&self) -> Self::Owned {
        self.to_path_buf()
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypedPathBuf<S: Str + ?Sized, B, D> {
    marker: PhantomData<fn() -> (B, D)>,
    inner: S::PathBuf,
}

impl<S: Str + ?Sized, B, D> TypedPathBuf<S, B, D> {
    fn from_inner(inner: S::PathBuf) -> Self {
        Self {
            marker: PhantomData,
            inner,
        }
    }

    pub fn into_std_path_buf(self) -> std::path::PathBuf {
        self.inner.__into_std_path_buf()
    }

    pub fn as_path(&self) -> &TypedPath<S, B, D> {
        TypedPath::from_inner(self.inner.__as_path())
    }
}

impl<S: Str + ?Sized, B: Base> TypedPathBuf<S, B, ()> {
    pub fn from_std_path_buf(path: std::path::PathBuf) -> crate::Result<Self> {
        B::__check(&path)?;
        let inner = S::PathBuf::__from_path_buf(path)
            .into_result()
            .map_err(|path| crate::Error::new(path, crate::ErrorKind::Utf8))?;
        Ok(Self::from_inner(inner))
    }
}

impl<S: Str + ?Sized, B: Base, D: Separator> TypedPathBuf<S, B, D> {
    pub fn from_camino_path_buf(mut path: camino::Utf8PathBuf) -> crate::Result<Self> {
        B::__check(path.as_ref())?;
        D::__normalize(&mut path);
        Ok(Self::from_inner(S::PathBuf::__from_camino_path_buf(path)))
    }

    pub fn push<P, S2, B2, D2>(&mut self, path: P)
    where
        Self: Push<S2, B2, D2>,
        P: AsTypedPath<Str = S2, Base = B2, Separator = D2>,
        S2: Str + ?Sized,
        B2: Base,
        D2: Separator,
    {
        self.__push(&path.__as_typed_path().inner);
    }
}

impl<S: Str + ?Sized, D> TypedPathBuf<S, (), D> {
    pub fn new() -> Self {
        Self::from_inner(<S::PathBuf as sealed::path_buf::Sealed>::__new())
    }
}

impl<S: Str + ?Sized, D> TypedPathBuf<S, Rel, D> {
    pub fn new() -> Self {
        Self::from_inner(<S::PathBuf as sealed::path_buf::Sealed>::__new())
    }
}

impl<S: Str + ?Sized, D> Default for TypedPathBuf<S, (), D> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: Str + ?Sized, D> Default for TypedPathBuf<S, Rel, D> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: Str + ?Sized, B, D> Borrow<TypedPath<S, B, D>> for TypedPathBuf<S, B, D> {
    fn borrow(&self) -> &TypedPath<S, B, D> {
        self.as_path()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypedPrefix<'a, S: ?Sized> {
    Verbatim(&'a S),
    VerbatimUNC(&'a S, &'a S),
    VerbatimDisk(u8),
    DeviceNS(&'a S),
    UNC(&'a S, &'a S),
    Disk(u8),
}

impl<S: Str + ?Sized> TypedPrefix<'_, S> {
    pub fn is_verbatim(&self) -> bool {
        use TypedPrefix::*;
        matches!(self, Verbatim(_) | VerbatimDisk(_) | VerbatimUNC(..))
    }
}

impl<S: ?Sized> Clone for TypedPrefix<'_, S> {
    fn clone(&self) -> Self {
        match *self {
            Self::Verbatim(x) => Self::Verbatim(x),
            Self::VerbatimUNC(x, y) => Self::VerbatimUNC(x, y),
            Self::VerbatimDisk(x) => Self::VerbatimDisk(x),
            Self::DeviceNS(x) => Self::DeviceNS(x),
            Self::UNC(x, y) => Self::UNC(x, y),
            Self::Disk(x) => Self::Disk(x),
        }
    }
}

impl<S: ?Sized> Copy for TypedPrefix<'_, S> {}

pub struct TypedPrefixComponent<'a, S: Str + ?Sized>(S::PrefixComponent<'a>);

impl<'a, S: Str + ?Sized> TypedPrefixComponent<'a, S> {
    pub fn kind(&self) -> TypedPrefix<'a, S> {
        self.0.__typed_kind()
    }

    pub fn as_os_str(&self) -> &'a OsStr {
        self.0.__as_os_str()
    }
}

pub enum TypedComponent<'a, S: Str + ?Sized> {
    Prefix(TypedPrefixComponent<'a, S>),
    RootDir,
    CurDir,
    ParentDir,
    Normal(&'a S),
}

impl<'a, S: Str + ?Sized> TypedComponent<'a, S> {
    pub fn as_os_str(&self) -> &'a OsStr {
        match self {
            Self::Prefix(p) => p.as_os_str(),
            Self::RootDir => std::path::Component::RootDir.as_os_str(),
            Self::CurDir => std::path::Component::CurDir.as_os_str(),
            Self::ParentDir => std::path::Component::ParentDir.as_os_str(),
            Self::Normal(s) => s.__as_ref_os_str(),
        }
    }
}

pub struct TypedComponents<'a, S: Str + ?Sized, D> {
    marker: PhantomData<fn() -> D>,
    inner: S::Components<'a>,
}

impl<'a, S: Str + ?Sized, D> TypedComponents<'a, S, D> {
    fn from_inner(inner: S::Components<'a>) -> Self {
        Self {
            marker: PhantomData,
            inner,
        }
    }

    pub fn as_path(&self) -> &'a TypedPath<S, (), D> {
        TypedPath::from_inner(self.inner.__as_path())
    }
}

pub struct Iter<'a, S: Str + ?Sized, D> {
    marker: PhantomData<fn() -> D>,
    inner: S::Iter<'a>,
}

impl<'a, S: Str + ?Sized, D> Iter<'a, S, D> {
    pub fn as_path(&self) -> &'a TypedPath<S, (), D> {
        TypedPath::from_inner(self.inner.as_path())
    }
}

#[derive(Clone, Copy)]
pub struct TypedAncestors<'a, S: Str + ?Sized, B, D> {
    marker: PhantomData<fn() -> (B, D)>,
    inner: S::Ancestors<'a>,
}

impl<'a, S: Str + ?Sized, B, D> TypedAncestors<'a, S, B, D> {
    fn from_inner(inner: S::Ancestors<'a>) -> Self {
        Self {
            marker: PhantomData,
            inner,
        }
    }
}

impl<S: Str + ?Sized, B, D> Debug for TypedAncestors<'_, S, B, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.inner, f)
    }
}

pub trait Str:
    'static + sealed::str::Sealed<__Path = Self::Path, __PathBuf = Self::PathBuf>
{
    type Path: path_traits::Path<Str = Self> + ?Sized;
    type PathBuf: path_traits::PathBuf<Str = Self>;
    type Prefix<'a>: path_traits::Prefix<'a>;
    type PrefixComponent<'a>: path_traits::PrefixComponent<'a, Str = Self>;
    type Component<'a>: path_traits::Component<'a>;
    type Components<'a>: path_traits::Components<'a, Str = Self>;
    type Iter<'a>: path_traits::Iter<'a, Str = Self>;
    type Ancestors<'a>: path_traits::Ancestors<'a>;
}

impl Str for OsStr {
    type Path = std::path::Path;
    type PathBuf = std::path::PathBuf;
    type Prefix<'a> = std::path::Prefix<'a>;
    type PrefixComponent<'a> = std::path::PrefixComponent<'a>;
    type Component<'a> = std::path::Component<'a>;
    type Components<'a> = std::path::Components<'a>;
    type Iter<'a> = std::path::Iter<'a>;
    type Ancestors<'a> = std::path::Ancestors<'a>;
}

impl Str for str {
    type Path = camino::Utf8Path;
    type PathBuf = camino::Utf8PathBuf;
    type Prefix<'a> = camino::Utf8Prefix<'a>;
    type PrefixComponent<'a> = camino::Utf8PrefixComponent<'a>;
    type Component<'a> = camino::Utf8Component<'a>;
    type Components<'a> = camino::Utf8Components<'a>;
    type Iter<'a> = camino::Iter<'a>;
    type Ancestors<'a> = camino::Utf8Ancestors<'a>;
}

pub trait Base: Copy + Ord + Hash + Debug + sealed::base::Sealed {}

impl Base for () {}
impl Base for Abs {}
impl Base for Rel {}

impl sealed::base::Sealed for () {
    fn __check(_: &std::path::Path) -> crate::Result<()> {
        Ok(())
    }
}

impl sealed::base::Sealed for Abs {
    fn __check(path: &std::path::Path) -> crate::Result<()> {
        if !path.is_absolute() {
            return Err(crate::Error::new(path, crate::ErrorKind::ExpectedAbs));
        }
        Ok(())
    }
}

impl sealed::base::Sealed for Rel {
    fn __check(path: &std::path::Path) -> crate::Result<()> {
        if !path.is_relative() {
            return Err(crate::Error::new(path, crate::ErrorKind::ExpectedRel));
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Abs {}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Rel {}

pub trait Separator: Copy + Ord + Hash + Debug + sealed::separator::Sealed {}

impl Separator for () {}
impl Separator for MainSeparator {}
impl Separator for Slash {}

impl sealed::separator::Sealed for () {
    fn __normalize(_: &mut camino::Utf8PathBuf) {}
}

impl sealed::separator::Sealed for MainSeparator {
    fn __normalize(path: &mut camino::Utf8PathBuf) {
        if let Cow::Owned(converted) = Cow::from_slash(path.as_ref()) {
            *path = converted.try_into().expect("should be UTF-8");
        }
    }
}

impl sealed::separator::Sealed for Slash {
    fn __normalize(path: &mut camino::Utf8PathBuf) {
        if let Cow::Owned(slash_path) = path.as_std_path().to_slash().expect("this is UTF-8") {
            *path = slash_path.into();
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum MainSeparator {}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Slash {}

pub type Result<T> = std::result::Result<T, Error>;

#[non_exhaustive]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Error {
    pub path: std::path::PathBuf,
    pub kind: ErrorKind,
}

impl Error {
    fn new(path: impl Into<std::path::PathBuf>, kind: ErrorKind) -> Self {
        let path = path.into();
        Self { path, kind }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.path.display())
    }
}

impl std::error::Error for Error {}

#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ErrorKind {
    Utf8,
    ExpectedAbs,
    ExpectedRel,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Utf8 => write!(f, "Expected UTF-8 path"),
            Self::ExpectedAbs => write!(f, "Expected absolute path"),
            Self::ExpectedRel => write!(f, "Expected relative path"),
        }
    }
}
