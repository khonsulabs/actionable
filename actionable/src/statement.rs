use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    fmt::{Display, Formatter, Write},
};

use super::{Action, ActionName};

/// A statement of permissions. A statement describes whether one or more
/// `actions` should be `allowed` to be taken against `resources`.
#[derive(Debug, Serialize, Deserialize)]
pub struct Statement {
    /// The list of resources this statement applies to.
    pub resources: Vec<ResourceName<'static>>,
    /// The list of actions this statement applies to.
    pub actions: ActionNameList,
}

impl Statement {
    /// Returns a statement that allows [`ActionNameList::All`] against
    /// [`ResourceName::any()`].
    #[must_use]
    pub fn allow_all() -> Self {
        Self {
            resources: vec![ResourceName::any()],
            actions: ActionNameList::All,
        }
    }
}

/// A single element of a [`ResourceName`]
#[derive(Debug, Hash, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum Identifier<'a> {
    /// When checking for allowed permissions, allow any match where this identifier is used.
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
#[derive(Debug, Serialize, Deserialize)]
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

impl From<ActionName> for ActionNameList {
    fn from(name: ActionName) -> Self {
        Self::List(vec![name])
    }
}

impl From<Vec<ActionName>> for ActionNameList {
    fn from(names: Vec<ActionName>) -> Self {
        Self::List(names)
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
    type Item = Identifier<'a>;

    type IntoIter = std::vec::IntoIter<Identifier<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
