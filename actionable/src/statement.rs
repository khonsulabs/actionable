use std::{
    borrow::Cow,
    collections::HashMap,
    convert::TryFrom,
    fmt::{Display, Formatter, Write},
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
#[derive(Debug, Hash, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum Identifier<'a> {
    /// When checking for allowed permissions, allow any match where this
    /// identifier is used.
    Any,
    /// An integer identifier.
    Integer(u64),
    /// A string identifier.
    String(Cow<'a, str>),
}

impl<'a> Identifier<'a> {
    /// Convert this identifier to an un-borrowed identifier.
    #[must_use]
    pub fn to_owned(&self) -> Identifier<'static> {
        match self {
            Self::Any => Identifier::Any,
            Self::Integer(value) => Identifier::Integer(*value),
            Self::String(value) => Identifier::String(Cow::Owned(value.to_string())),
        }
    }
}

impl<'a> Display for Identifier<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Any => f.write_char('*'),
            Self::Integer(integer) => integer.fmt(f),
            Self::String(string) => string.fmt(f),
        }
    }
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
