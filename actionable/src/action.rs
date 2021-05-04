use std::{
    borrow::Cow,
    fmt::{Display, Write},
};

use serde::{Deserialize, Serialize};

/// An action that can be allowed or disalled.
pub trait Action {
    /// The full name of this action.
    fn name(&self) -> ActionName;
}

impl Action for () {
    fn name(&self) -> ActionName {
        ActionName::default()
    }
}

/// A unique name of an action.
#[derive(Default, Debug, Serialize, Deserialize)]
#[allow(clippy::module_name_repetitions)] // exported without the module name
pub struct ActionName(pub Vec<Cow<'static, str>>);

impl Display for ActionName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, name) in self.0.iter().enumerate() {
            if index > 0 {
                f.write_char('.')?;
            }

            name.fmt(f)?;
        }
        Ok(())
    }
}

pub use actionable_macros::Action;
