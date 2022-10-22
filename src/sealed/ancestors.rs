pub trait Sealed<'a> {}

impl<'a> Sealed<'a> for std::path::Ancestors<'a> {}
impl<'a> Sealed<'a> for camino::Utf8Ancestors<'a> {}
