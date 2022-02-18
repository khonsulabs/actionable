use std::{
    borrow::Cow,
    collections::HashMap,
    convert::TryFrom,
    fmt::{Display, Formatter, Write},
    hash::Hash,
};

use serde::{Deserialize, Serialize};

use super::{Action, ActionName};

/// A statement of permissions. A statement describes whether one or more
/// `actions` should be `allowed` to be taken against `resources`.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[must_use]
pub struct Statement {
    /// The list of resources this statement applies to.
    pub resources: Vec<ResourceName<'static>>,
    /// The list of actions this statement applies to.
    pub actions: Option<ActionNameList>,
    /// Any configured values for these resources.
    pub configuration: Option<HashMap<String, Configuration>>,
}

impl Statement {
    /// Returns a statement that allows [`ActionNameList::All`] against
    /// [`ResourceName::any()`].
    pub fn allow_all_for_any_resource() -> Self {
        Self::for_any().allowing_all()
    }

    /// Returns an empty statement for a resource named `name`.
    pub fn for_resource(name: impl Into<ResourceName<'static>>) -> Self {
        Self {
            resources: vec![name.into()],
            actions: None,
            configuration: None,
        }
    }

    /// Returns an empty statement for [`ResourceName::any()`].
    pub fn for_any() -> Self {
        Self {
            resources: vec![ResourceName::any()],
            actions: None,
            configuration: None,
        }
    }

    /// Returns an empty statement for a resources named `names`.
    pub fn for_resources<II: IntoIterator<Item = ResourceName<'static>>>(names: II) -> Self {
        Self {
            resources: names.into_iter().collect(),
            actions: None,
            configuration: None,
        }
    }

    /// Allows `action` to be performed.
    pub fn allow<A: Action>(&mut self, action: &A) {
        match &mut self.actions {
            Some(ActionNameList::All) => {}
            Some(ActionNameList::List(names)) => {
                names.push(action.name());
            }
            None => {
                self.actions = Some(ActionNameList::List(vec![action.name()]));
            }
        }
    }

    /// Allows `action` and returns self.
    pub fn allowing<A: Action>(mut self, action: &A) -> Self {
        self.allow(action);
        self
    }

    /// Allows [`ActionNameList::All`].
    pub fn allow_all(&mut self) {
        self.actions = Some(ActionNameList::All);
    }

    /// Allows [`ActionNameList::All`] and returns self.
    pub fn allowing_all(mut self) -> Self {
        self.allow_all();
        self
    }

    /// Sets `configuration` for `key` for the resources in this statement.
    pub fn configure<S: Into<String>, C: Into<Configuration>>(&mut self, key: S, configuration: C) {
        let configurations = self.configuration.get_or_insert_with(HashMap::default);
        configurations.insert(key.into(), configuration.into());
    }

    /// Configures `configuration` for `key` and returns self.
    pub fn with<S: Into<String>, C: Into<Configuration>>(
        mut self,
        key: S,
        configuration: C,
    ) -> Self {
        self.configure(key, configuration);
        self
    }
}

/// A single element of a [`ResourceName`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Identifier<'a> {
    /// When checking for allowed permissions, allow any match where this
    /// identifier is used.
    Any,
    /// An integer identifier.
    Integer(u64),
    /// A string identifier.
    String(Cow<'a, str>),
    /// A binary identifier.
    Bytes(Cow<'a, [u8]>),
}

impl<'a> Hash for Identifier<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // To ensure matching the implementation of Eq, we need to hash
        // everything to bytes in the same way they would be compared.
        match self {
            Identifier::Any => {
                // We get to pick an arbitrary hash for this value. It only
                // needs to be self consistent. A null byte is likely to be
                // unique in terms of produced hash values.
                state.write_u8(0);
            }
            Identifier::Integer(int) => {
                state.write(&int.to_be_bytes());
            }
            Identifier::String(string) => {
                state.write(string.as_bytes());
            }
            Identifier::Bytes(bytes) => {
                state.write(bytes);
            }
        }
    }
}

#[test]
fn identifier_hash_tests() {
    fn hash(identifier: &Identifier<'_>) -> u64 {
        use std::hash::Hasher;
        let mut hasher = std::collections::hash_map::DefaultHasher::default();
        identifier.hash(&mut hasher);
        hasher.finish()
    }

    let integer_a = Identifier::from(u64::from_be_bytes(*b"helloooo"));
    let string_a = Identifier::from("helloooo");
    let bytes_a = Identifier::from(b"helloooo");
    let string_b = Identifier::from("woooorld");

    assert_eq!(hash(&Identifier::Any), hash(&Identifier::Any));
    assert_eq!(hash(&string_a), hash(&string_a));
    assert_eq!(hash(&integer_a), hash(&string_a));
    assert_eq!(hash(&bytes_a), hash(&string_a));
    assert_ne!(hash(&string_a), hash(&string_b));
    assert_ne!(hash(&integer_a), hash(&string_b));
    assert_ne!(hash(&bytes_a), hash(&string_b));
}

impl<'a> Eq for Identifier<'a> {}

impl<'a> PartialEq for Identifier<'a> {
    fn eq(&self, other: &Self) -> bool {
        match other {
            Self::Any => matches!(self, Self::Any),
            Self::Integer(int) => self.eq_int(*int),
            Self::String(string) => self.eq_str(string),
            Self::Bytes(bytes) => self.eq_bytes(bytes),
        }
    }
}

impl<'a> Identifier<'a> {
    /// Convert this identifier to an un-borrowed identifier.
    #[must_use]
    pub fn to_owned(&self) -> Identifier<'static> {
        match self {
            Self::Any => Identifier::Any,
            Self::Integer(value) => Identifier::Integer(*value),
            Self::String(value) => Identifier::String(Cow::Owned(value.to_string())),
            Self::Bytes(value) => Identifier::Bytes(Cow::Owned(value.to_vec())),
        }
    }

    fn eq_int(&self, other: u64) -> bool {
        match self {
            Identifier::Any => false,
            Identifier::Integer(int) => *int == other,
            Identifier::String(string) => {
                let other = other.to_be_bytes();
                string.as_bytes() == other
            }
            Identifier::Bytes(bytes) => {
                let other = other.to_be_bytes();
                **bytes == other
            }
        }
    }

    fn eq_str(&self, other: &str) -> bool {
        match self {
            Identifier::Any => false,
            Identifier::Integer(int) => {
                let int = int.to_be_bytes();
                int == other.as_bytes()
            }
            Identifier::String(string) => string == other,
            Identifier::Bytes(bytes) => &**bytes == other.as_bytes(),
        }
    }

    fn eq_bytes(&self, other: &[u8]) -> bool {
        match self {
            Identifier::Any => false,
            Identifier::Integer(int) => {
                let int = int.to_be_bytes();
                int == other
            }
            Identifier::String(string) => string.as_bytes() == other,
            Identifier::Bytes(bytes) => &**bytes == other,
        }
    }
}

#[test]
fn identifier_equality_tests() {
    let integer_a = Identifier::from(u64::from_be_bytes(*b"helloooo"));
    let integer_b = Identifier::from(u64::from_be_bytes(*b"woooorld"));
    let string_a = Identifier::from("helloooo");
    let string_b = Identifier::from("woooorld");
    let bytes_a = Identifier::from(b"helloooo");
    let bytes_b = Identifier::from(b"woooorld");

    // Integer on left
    assert_ne!(integer_a, Identifier::Any);
    assert_eq!(integer_a, integer_a);
    assert_eq!(integer_a, string_a);
    assert_eq!(integer_a, bytes_a);
    assert_ne!(integer_a, integer_b);
    assert_ne!(integer_a, string_b);
    assert_ne!(integer_a, bytes_b);

    // String on left
    assert_ne!(string_a, Identifier::Any);
    assert_eq!(string_a, integer_a);
    assert_eq!(string_a, string_a);
    assert_eq!(string_a, bytes_a);
    assert_ne!(string_a, integer_b);
    assert_ne!(string_a, string_b);
    assert_ne!(string_a, bytes_b);

    // Bytes on left
    assert_ne!(bytes_a, Identifier::Any);
    assert_eq!(bytes_a, integer_a);
    assert_eq!(bytes_a, string_a);
    assert_eq!(bytes_a, bytes_a);
    assert_ne!(bytes_a, integer_b);
    assert_ne!(bytes_a, string_b);
    assert_ne!(bytes_a, bytes_b);

    // Any on left
    assert_eq!(Identifier::Any, Identifier::Any);
    assert_ne!(Identifier::Any, integer_a);
    assert_ne!(Identifier::Any, string_a);
    assert_ne!(Identifier::Any, bytes_a);
}

impl<'a> Display for Identifier<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Any => f.write_char('*'),
            Self::Integer(integer) => integer.fmt(f),
            Self::String(string) => string.fmt(f),
            Self::Bytes(bytes) => {
                f.write_char('$')?;
                for byte in bytes.iter() {
                    write!(f, "{:02x}", byte)?;
                }
                Ok(())
            }
        }
    }
}

#[test]
fn identifier_display_tests() {
    assert_eq!(Identifier::Any.to_string(), "*");
    assert_eq!(Identifier::from(1).to_string(), "1");
    assert_eq!(Identifier::from("string").to_string(), "string");
    assert_eq!(Identifier::from(b"bytes").to_string(), "$6279746573");
}

impl<'a> From<u64> for Identifier<'a> {
    fn from(id: u64) -> Self {
        Self::Integer(id)
    }
}

impl<'a> From<&'a str> for Identifier<'a> {
    fn from(id: &'a str) -> Self {
        Self::String(Cow::Borrowed(id))
    }
}

impl<'a> From<&'a String> for Identifier<'a> {
    fn from(id: &'a String) -> Self {
        Self::from(id.as_str())
    }
}

impl<'a> From<String> for Identifier<'a> {
    fn from(id: String) -> Self {
        Self::String(Cow::Owned(id))
    }
}

impl<'a, const N: usize> From<&'a [u8; N]> for Identifier<'a> {
    fn from(id: &'a [u8; N]) -> Self {
        Self::from(&id[..])
    }
}

impl<'a, const N: usize> From<[u8; N]> for Identifier<'a> {
    fn from(id: [u8; N]) -> Self {
        Self::from(id.to_vec())
    }
}

impl<'a> From<&'a [u8]> for Identifier<'a> {
    fn from(id: &'a [u8]) -> Self {
        Self::Bytes(Cow::Borrowed(id))
    }
}

impl<'a> From<&'a Vec<u8>> for Identifier<'a> {
    fn from(id: &'a Vec<u8>) -> Self {
        Self::from(id.clone())
    }
}

impl<'a> From<Vec<u8>> for Identifier<'a> {
    fn from(id: Vec<u8>) -> Self {
        Self::Bytes(Cow::Owned(id))
    }
}

#[test]
fn identifier_from_tests() {
    assert_eq!(Identifier::from(1).to_string(), "1");
    assert_eq!(Identifier::from("string").to_string(), "string");
    assert_eq!(
        Identifier::from(&String::from("string")).to_string(),
        "string"
    );
    assert_eq!(
        Identifier::from(String::from("string")).to_string(),
        "string"
    );
    // This calls through to from(&[u8])
    assert_eq!(Identifier::from(b"bytes").to_string(), "$6279746573");
    // This calls through to from(Vec<u8>)
    assert_eq!(Identifier::from(*b"bytes").to_string(), "$6279746573");
    assert_eq!(
        Identifier::from(&b"bytes".to_vec()).to_string(),
        "$6279746573"
    );
}

/// A list of [`ActionName`]s.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ActionNameList {
    /// A specific list of names.
    List(Vec<ActionName>),
    /// All actions.
    All,
}

impl<T> From<T> for ActionNameList
where
    T: Action,
{
    fn from(action: T) -> Self {
        Self::List(vec![action.name()])
    }
}

impl<T> From<Vec<T>> for ActionNameList
where
    T: Action,
{
    fn from(actions: Vec<T>) -> Self {
        Self::List(actions.into_iter().map(|action| action.name()).collect())
    }
}

/// A configured value for a resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Configuration {
    /// An unsigned integer configuration value.
    Unsigned(u64),
    /// A signed integer configuration value.
    Signed(i64),
    /// A string configuration value.
    String(String),
}

impl Configuration {
    /// Evaluates the contents of this configuration as a signed integer.
    /// Returns None if unable to convert safely.
    #[must_use]
    pub fn to_signed(&self) -> Option<i64> {
        match self {
            Configuration::Unsigned(unsigned) => i64::try_from(*unsigned).ok(),
            Configuration::Signed(signed) => Some(*signed),
            Configuration::String(string) => string.parse().ok(),
        }
    }

    /// Evaluates the contents of this configuration as an unsigned integer.
    /// Returns None if unable to convert safely.
    #[must_use]
    pub fn to_unsigned(&self) -> Option<u64> {
        match self {
            Configuration::Unsigned(unsigned) => Some(*unsigned),
            Configuration::Signed(signed) => u64::try_from(*signed).ok(),
            Configuration::String(string) => string.parse().ok(),
        }
    }
}

impl Display for Configuration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Configuration::Unsigned(unsigned) => unsigned.fmt(f),
            Configuration::Signed(signed) => signed.fmt(f),
            Configuration::String(string) => string.fmt(f),
        }
    }
}

impl From<u64> for Configuration {
    fn from(value: u64) -> Self {
        Self::Unsigned(value)
    }
}

impl From<i64> for Configuration {
    fn from(value: i64) -> Self {
        Self::Signed(value)
    }
}

impl From<String> for Configuration {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl<'a> From<&'a str> for Configuration {
    fn from(value: &'a str) -> Self {
        Self::String(value.to_string())
    }
}

/// A unique name/identifier of a resource.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ResourceName<'a>(Vec<Identifier<'a>>);

impl<'a> ResourceName<'a> {
    /// Convert a borrowed name to an un-borrwed name.
    #[must_use]
    pub fn to_owned(&self) -> ResourceName<'static> {
        ResourceName(self.0.iter().map(Identifier::to_owned).collect())
    }
}

impl<'a> Display for ResourceName<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (index, identifier) in self.0.iter().enumerate() {
            if index > 0 {
                f.write_char('.')?;
            }

            identifier.fmt(f)?;
        }

        Ok(())
    }
}

impl<'a> ResourceName<'a> {
    /// Creates a `ResourceName` that matches any identifier.
    #[must_use]
    pub fn any() -> Self {
        Self::named(Identifier::Any)
    }

    /// Creates a `ResourceName` with `name`.
    #[must_use]
    pub fn named<I: Into<Identifier<'a>>>(name: I) -> Self {
        Self(vec![name.into()])
    }

    /// Adds another name segment.
    #[must_use]
    pub fn and<I: Into<Identifier<'a>>>(mut self, name: I) -> Self {
        self.0.push(name.into());
        self
    }
}

impl<'a> AsRef<[Identifier<'a>]> for ResourceName<'a> {
    fn as_ref(&self) -> &[Identifier<'a>] {
        &self.0
    }
}

impl<'a> IntoIterator for ResourceName<'a> {
    type IntoIter = std::vec::IntoIter<Identifier<'a>>;
    type Item = Identifier<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'b, 'a> From<&'b [Identifier<'a>]> for ResourceName<'a> {
    fn from(parts: &'b [Identifier<'a>]) -> Self {
        Self(parts.to_vec())
    }
}

impl<'a> From<&'a str> for ResourceName<'a> {
    fn from(name: &'a str) -> Self {
        Self(vec![Identifier::from(name)])
    }
}

impl<'a> From<u64> for ResourceName<'a> {
    fn from(name: u64) -> Self {
        Self(vec![Identifier::from(name)])
    }
}
