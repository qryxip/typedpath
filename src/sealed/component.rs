pub trait Sealed<'a> {}

impl<'a> Sealed<'a> for std::path::Component<'a> {}
impl<'a> Sealed<'a> for camino::Utf8Component<'a> {}
