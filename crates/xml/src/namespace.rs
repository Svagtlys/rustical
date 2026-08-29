use quick_xml::name::Namespace;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct NamespaceOwned(pub String);

impl<'a> From<Namespace<'a>> for NamespaceOwned {
    fn from(value: Namespace<'a>) -> Self {
        Self(value.0.to_owned())
    }
}

impl From<String> for NamespaceOwned {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for NamespaceOwned {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl<'a> From<&'a Namespace<'a>> for NamespaceOwned {
    fn from(value: &'a Namespace<'a>) -> Self {
        Self(value.0.to_owned())
    }
}

impl NamespaceOwned {
    #[must_use]
    pub fn as_ref(&self) -> Namespace<'_> {
        Namespace(&self.0)
    }
}
