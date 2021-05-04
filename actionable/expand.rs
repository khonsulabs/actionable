#![feature(prelude_import)]
//! An enum-based async framework for building permission-driven APIs
#![forbid(unsafe_code)]
#![warn(
    clippy::cargo,
    missing_docs,
    clippy::nursery,
    clippy::pedantic,
    future_incompatible,
    rust_2018_idioms
)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
mod action {
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
    #[allow(clippy::module_name_repetitions)]
    pub struct ActionName(pub Vec<Cow<'static, str>>);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    #[allow(clippy::module_name_repetitions)]
    impl ::core::default::Default for ActionName {
        #[inline]
        fn default() -> ActionName {
            ActionName(::core::default::Default::default())
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    #[allow(clippy::module_name_repetitions)]
    impl ::core::fmt::Debug for ActionName {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                ActionName(ref __self_0_0) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "ActionName");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ActionName {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                _serde::Serializer::serialize_newtype_struct(__serializer, "ActionName", &self.0)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ActionName {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<ActionName>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ActionName;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "tuple struct ActionName",
                        )
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(
                        self,
                        __e: __E,
                    ) -> _serde::__private::Result<Self::Value, __E::Error>
                    where
                        __E: _serde::Deserializer<'de>,
                    {
                        let __field0: Vec<Cow<'static, str>> =
                            match <Vec<Cow<'static, str>> as _serde::Deserialize>::deserialize(__e)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                        _serde::__private::Ok(ActionName(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            Vec<Cow<'static, str>>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"tuple struct ActionName with 1 element",
                                ));
                            }
                        };
                        _serde::__private::Ok(ActionName(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(
                    __deserializer,
                    "ActionName",
                    __Visitor {
                        marker: _serde::__private::PhantomData::<ActionName>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
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
}
mod permissions {
    use std::collections::HashMap;
    use crate::{Action, ActionNameList, Identifier, Statement};
    /// A collection of allowed permissions.
    pub struct Permissions {
        children: Option<HashMap<Identifier<'static>, Permissions>>,
        allowed: AllowedActions,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for Permissions {
        #[inline]
        fn default() -> Permissions {
            Permissions {
                children: ::core::default::Default::default(),
                allowed: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Permissions {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Permissions {
                    children: ref __self_0_0,
                    allowed: ref __self_0_1,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "Permissions");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "children",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "allowed",
                        &&(*__self_0_1),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    impl Permissions {
        /// Evaluate whether the `action` is allowed to be taken upon
        /// `resource_name`. Returns true if the action should be allowed. If no
        /// statements that match `resource_name` allow `action`, false will be
        /// returned.
        pub fn allowed_to<R: AsRef<[Identifier<'static>]>, P: Action>(
            &self,
            resource_name: R,
            action: &P,
        ) -> bool {
            let resource_name = resource_name.as_ref();
            if let Some(resource) = resource_name.first() {
                if let Some(children) = &self.children {
                    let remaining_resource = &resource_name[1..resource_name.len()];
                    if let Some(permissions) = children.get(resource) {
                        {
                            ::std::io::_print(::core::fmt::Arguments::new_v1(
                                &["Checking allowed in ", "\n"],
                                &match (&resource,) {
                                    (arg0,) => [::core::fmt::ArgumentV1::new(
                                        arg0,
                                        ::core::fmt::Debug::fmt,
                                    )],
                                },
                            ));
                        };
                        if permissions.allowed_to(remaining_resource, action) {
                            return match true {
                                tmp => {
                                    {
                                        :: std :: io :: _eprint (:: core :: fmt :: Arguments :: new_v1_formatted (& ["[" , ":" , "] " , " = " , "\n"] , & match (& "actionable/src/permissions.rs" , & 34u32 , & "true" , & & tmp) { (arg0 , arg1 , arg2 , arg3) => [:: core :: fmt :: ArgumentV1 :: new (arg0 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (arg1 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (arg2 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (arg3 , :: core :: fmt :: Debug :: fmt)] , } , & [:: core :: fmt :: rt :: v1 :: Argument { position : 0usize , format : :: core :: fmt :: rt :: v1 :: FormatSpec { fill : ' ' , align : :: core :: fmt :: rt :: v1 :: Alignment :: Unknown , flags : 0u32 , precision : :: core :: fmt :: rt :: v1 :: Count :: Implied , width : :: core :: fmt :: rt :: v1 :: Count :: Implied , } , } , :: core :: fmt :: rt :: v1 :: Argument { position : 1usize , format : :: core :: fmt :: rt :: v1 :: FormatSpec { fill : ' ' , align : :: core :: fmt :: rt :: v1 :: Alignment :: Unknown , flags : 0u32 , precision : :: core :: fmt :: rt :: v1 :: Count :: Implied , width : :: core :: fmt :: rt :: v1 :: Count :: Implied , } , } , :: core :: fmt :: rt :: v1 :: Argument { position : 2usize , format : :: core :: fmt :: rt :: v1 :: FormatSpec { fill : ' ' , align : :: core :: fmt :: rt :: v1 :: Alignment :: Unknown , flags : 0u32 , precision : :: core :: fmt :: rt :: v1 :: Count :: Implied , width : :: core :: fmt :: rt :: v1 :: Count :: Implied , } , } , :: core :: fmt :: rt :: v1 :: Argument { position : 3usize , format : :: core :: fmt :: rt :: v1 :: FormatSpec { fill : ' ' , align : :: core :: fmt :: rt :: v1 :: Alignment :: Unknown , flags : 4u32 , precision : :: core :: fmt :: rt :: v1 :: Count :: Implied , width : :: core :: fmt :: rt :: v1 :: Count :: Implied , } , }])) ;
                                    };
                                    tmp
                                }
                            };
                        }
                        {
                            ::std::io::_print(::core::fmt::Arguments::new_v1(
                                &["Nope\n"],
                                &match () {
                                    () => [],
                                },
                            ));
                        };
                    }
                    if let Some(permissions) = children.get(&Identifier::Any) {
                        {
                            ::std::io::_print(::core::fmt::Arguments::new_v1(
                                &["Checking allowed in Any\n"],
                                &match () {
                                    () => [],
                                },
                            ));
                        };
                        if permissions.allowed_to(remaining_resource, action) {
                            return match true {
                                tmp => {
                                    {
                                        :: std :: io :: _eprint (:: core :: fmt :: Arguments :: new_v1_formatted (& ["[" , ":" , "] " , " = " , "\n"] , & match (& "actionable/src/permissions.rs" , & 43u32 , & "true" , & & tmp) { (arg0 , arg1 , arg2 , arg3) => [:: core :: fmt :: ArgumentV1 :: new (arg0 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (arg1 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (arg2 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (arg3 , :: core :: fmt :: Debug :: fmt)] , } , & [:: core :: fmt :: rt :: v1 :: Argument { position : 0usize , format : :: core :: fmt :: rt :: v1 :: FormatSpec { fill : ' ' , align : :: core :: fmt :: rt :: v1 :: Alignment :: Unknown , flags : 0u32 , precision : :: core :: fmt :: rt :: v1 :: Count :: Implied , width : :: core :: fmt :: rt :: v1 :: Count :: Implied , } , } , :: core :: fmt :: rt :: v1 :: Argument { position : 1usize , format : :: core :: fmt :: rt :: v1 :: FormatSpec { fill : ' ' , align : :: core :: fmt :: rt :: v1 :: Alignment :: Unknown , flags : 0u32 , precision : :: core :: fmt :: rt :: v1 :: Count :: Implied , width : :: core :: fmt :: rt :: v1 :: Count :: Implied , } , } , :: core :: fmt :: rt :: v1 :: Argument { position : 2usize , format : :: core :: fmt :: rt :: v1 :: FormatSpec { fill : ' ' , align : :: core :: fmt :: rt :: v1 :: Alignment :: Unknown , flags : 0u32 , precision : :: core :: fmt :: rt :: v1 :: Count :: Implied , width : :: core :: fmt :: rt :: v1 :: Count :: Implied , } , } , :: core :: fmt :: rt :: v1 :: Argument { position : 3usize , format : :: core :: fmt :: rt :: v1 :: FormatSpec { fill : ' ' , align : :: core :: fmt :: rt :: v1 :: Alignment :: Unknown , flags : 4u32 , precision : :: core :: fmt :: rt :: v1 :: Count :: Implied , width : :: core :: fmt :: rt :: v1 :: Count :: Implied , } , }])) ;
                                    };
                                    tmp
                                }
                            };
                        }
                        {
                            ::std::io::_print(::core::fmt::Arguments::new_v1(
                                &["Nope\n"],
                                &match () {
                                    () => [],
                                },
                            ));
                        };
                    }
                }
            }
            let mut allowed = &self.allowed;
            for name in action.name().0 {
                {
                    ::std::io::_print(::core::fmt::Arguments::new_v1(
                        &["checking permissions for ", " in ", "\n"],
                        &match (&name, &allowed) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                            ],
                        },
                    ));
                };
                allowed = match allowed {
                    AllowedActions::None => {
                        return match false {
                            tmp => {
                                {
                                    ::std::io::_eprint(::core::fmt::Arguments::new_v1_formatted(
                                        &["[", ":", "] ", " = ", "\n"],
                                        &match (
                                            &"actionable/src/permissions.rs",
                                            &60u32,
                                            &"false",
                                            &&tmp,
                                        ) {
                                            (arg0, arg1, arg2, arg3) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg3,
                                                    ::core::fmt::Debug::fmt,
                                                ),
                                            ],
                                        },
                                        &[
                                            ::core::fmt::rt::v1::Argument {
                                                position: 0usize,
                                                format: ::core::fmt::rt::v1::FormatSpec {
                                                    fill: ' ',
                                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                    flags: 0u32,
                                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                                    width: ::core::fmt::rt::v1::Count::Implied,
                                                },
                                            },
                                            ::core::fmt::rt::v1::Argument {
                                                position: 1usize,
                                                format: ::core::fmt::rt::v1::FormatSpec {
                                                    fill: ' ',
                                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                    flags: 0u32,
                                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                                    width: ::core::fmt::rt::v1::Count::Implied,
                                                },
                                            },
                                            ::core::fmt::rt::v1::Argument {
                                                position: 2usize,
                                                format: ::core::fmt::rt::v1::FormatSpec {
                                                    fill: ' ',
                                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                    flags: 0u32,
                                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                                    width: ::core::fmt::rt::v1::Count::Implied,
                                                },
                                            },
                                            ::core::fmt::rt::v1::Argument {
                                                position: 3usize,
                                                format: ::core::fmt::rt::v1::FormatSpec {
                                                    fill: ' ',
                                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                    flags: 4u32,
                                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                                    width: ::core::fmt::rt::v1::Count::Implied,
                                                },
                                            },
                                        ],
                                    ));
                                };
                                tmp
                            }
                        }
                    }
                    AllowedActions::All => {
                        return match true {
                            tmp => {
                                {
                                    ::std::io::_eprint(::core::fmt::Arguments::new_v1_formatted(
                                        &["[", ":", "] ", " = ", "\n"],
                                        &match (
                                            &"actionable/src/permissions.rs",
                                            &61u32,
                                            &"true",
                                            &&tmp,
                                        ) {
                                            (arg0, arg1, arg2, arg3) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg3,
                                                    ::core::fmt::Debug::fmt,
                                                ),
                                            ],
                                        },
                                        &[
                                            ::core::fmt::rt::v1::Argument {
                                                position: 0usize,
                                                format: ::core::fmt::rt::v1::FormatSpec {
                                                    fill: ' ',
                                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                    flags: 0u32,
                                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                                    width: ::core::fmt::rt::v1::Count::Implied,
                                                },
                                            },
                                            ::core::fmt::rt::v1::Argument {
                                                position: 1usize,
                                                format: ::core::fmt::rt::v1::FormatSpec {
                                                    fill: ' ',
                                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                    flags: 0u32,
                                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                                    width: ::core::fmt::rt::v1::Count::Implied,
                                                },
                                            },
                                            ::core::fmt::rt::v1::Argument {
                                                position: 2usize,
                                                format: ::core::fmt::rt::v1::FormatSpec {
                                                    fill: ' ',
                                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                    flags: 0u32,
                                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                                    width: ::core::fmt::rt::v1::Count::Implied,
                                                },
                                            },
                                            ::core::fmt::rt::v1::Argument {
                                                position: 3usize,
                                                format: ::core::fmt::rt::v1::FormatSpec {
                                                    fill: ' ',
                                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                    flags: 4u32,
                                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                                    width: ::core::fmt::rt::v1::Count::Implied,
                                                },
                                            },
                                        ],
                                    ));
                                };
                                tmp
                            }
                        }
                    }
                    AllowedActions::Some(actions) => {
                        if let Some(children_allowed) = actions.get(name.as_ref()) {
                            children_allowed
                        } else {
                            {
                                ::std::io::_print(::core::fmt::Arguments::new_v1(
                                    &["Didn\'t find child: ", "\n"],
                                    &match (&name,) {
                                        (arg0,) => [::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Debug::fmt,
                                        )],
                                    },
                                ));
                            };
                            return false;
                        }
                    }
                };
            }
            match match allowed {
                AllowedActions::All => true,
                _ => false,
            } {
                tmp => {
                    {
                        ::std::io::_eprint(::core::fmt::Arguments::new_v1_formatted(
                            &["[", ":", "] ", " = ", "\n"],
                            &match (
                                &"actionable/src/permissions.rs",
                                &72u32,
                                &"matches!(allowed, AllowedActions :: All)",
                                &&tmp,
                            ) {
                                (arg0, arg1, arg2, arg3) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg2, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg3, ::core::fmt::Debug::fmt),
                                ],
                            },
                            &[
                                ::core::fmt::rt::v1::Argument {
                                    position: 0usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 0u32,
                                        precision: ::core::fmt::rt::v1::Count::Implied,
                                        width: ::core::fmt::rt::v1::Count::Implied,
                                    },
                                },
                                ::core::fmt::rt::v1::Argument {
                                    position: 1usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 0u32,
                                        precision: ::core::fmt::rt::v1::Count::Implied,
                                        width: ::core::fmt::rt::v1::Count::Implied,
                                    },
                                },
                                ::core::fmt::rt::v1::Argument {
                                    position: 2usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 0u32,
                                        precision: ::core::fmt::rt::v1::Count::Implied,
                                        width: ::core::fmt::rt::v1::Count::Implied,
                                    },
                                },
                                ::core::fmt::rt::v1::Argument {
                                    position: 3usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 4u32,
                                        precision: ::core::fmt::rt::v1::Count::Implied,
                                        width: ::core::fmt::rt::v1::Count::Implied,
                                    },
                                },
                            ],
                        ));
                    };
                    tmp
                }
            }
        }
    }
    impl From<Vec<Statement>> for Permissions {
        fn from(statements: Vec<Statement>) -> Self {
            let mut permissions = Self::default();
            for statement in statements {
                for resource in statement.resources {
                    let mut current_permissions = &mut permissions;
                    for name in resource {
                        let permissions = current_permissions
                            .children
                            .get_or_insert_with(HashMap::default);
                        current_permissions = permissions.entry(name).or_default();
                    }
                    let mut allowed = &mut current_permissions.allowed;
                    match &statement.actions {
                        ActionNameList::List(actions) => {
                            for action in actions {
                                for name in &action.0 {
                                    let action_map = match allowed {
                                        AllowedActions::All | AllowedActions::None => {
                                            *allowed = {
                                                let mut action_map = HashMap::new();
                                                action_map
                                                    .insert(name.to_string(), AllowedActions::None);
                                                AllowedActions::Some(action_map)
                                            };
                                            if let AllowedActions::Some(action_map) = allowed {
                                                action_map
                                            } else {
                                                {
                                                    ::core::panicking::panic(
                                                        "internal error: entered unreachable code",
                                                    )
                                                }
                                            }
                                        }
                                        AllowedActions::Some(action_map) => action_map,
                                    };
                                    allowed = action_map.entry(name.to_string()).or_default();
                                }
                            }
                        }
                        ActionNameList::All => {}
                    }
                    if statement.allowed {
                        *allowed = AllowedActions::All
                    } else {
                        *allowed = AllowedActions::None
                    }
                }
            }
            permissions
        }
    }
    enum AllowedActions {
        None,
        Some(HashMap<String, AllowedActions>),
        All,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for AllowedActions {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&AllowedActions::None,) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "None");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&AllowedActions::Some(ref __self_0),) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Some");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&AllowedActions::All,) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "All");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    impl Default for AllowedActions {
        fn default() -> Self {
            Self::None
        }
    }
}
mod statement {
    use serde::{Deserialize, Serialize};
    use std::{
        borrow::Cow,
        fmt::{Display, Formatter, Write},
    };
    use super::{Action, ActionName};
    /// A statement of permissions. A statement describes whether one or more
    /// `actions` should be `allowed` to be taken against `resources`.
    pub struct Statement {
        /// The list of resources this statement applies to.
        pub resources: Vec<ResourceName>,
        /// The list of actions this statement applies to.
        pub actions: ActionNameList,
        /// Whether the `actions` should be allowed or disallowed.
        pub allowed: bool,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Statement {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Statement {
                    resources: ref __self_0_0,
                    actions: ref __self_0_1,
                    allowed: ref __self_0_2,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "Statement");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "resources",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "actions",
                        &&(*__self_0_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "allowed",
                        &&(*__self_0_2),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Statement {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Statement",
                    false as usize + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "resources",
                    &self.resources,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "actions",
                    &self.actions,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "allowed",
                    &self.allowed,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Statement {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "resources" => _serde::__private::Ok(__Field::__field0),
                            "actions" => _serde::__private::Ok(__Field::__field1),
                            "allowed" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"resources" => _serde::__private::Ok(__Field::__field0),
                            b"actions" => _serde::__private::Ok(__Field::__field1),
                            b"allowed" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Statement>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Statement;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct Statement")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            Vec<ResourceName>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct Statement with 3 elements",
                                ));
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            ActionNameList,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct Statement with 3 elements",
                                ));
                            }
                        };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Statement with 3 elements",
                                        ),
                                    );
                                }
                            };
                        _serde::__private::Ok(Statement {
                            resources: __field0,
                            actions: __field1,
                            allowed: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Vec<ResourceName>> =
                            _serde::__private::None;
                        let mut __field1: _serde::__private::Option<ActionNameList> =
                            _serde::__private::None;
                        let mut __field2: _serde::__private::Option<bool> = _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "resources",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Vec<ResourceName>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "actions",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<ActionNameList>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "allowed",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<bool>(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("resources") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("actions") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("allowed") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(Statement {
                            resources: __field0,
                            actions: __field1,
                            allowed: __field2,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["resources", "actions", "allowed"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Statement",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Statement>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    /// A single element of a [`ResourceName`]
    pub enum Identifier<'a> {
        /// When checking for allowed permissions, allow any match where this identifier is used.
        Any,
        /// An integer identifier.
        Integer(u64),
        /// A string identifier.
        String(Cow<'a, str>),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'a> ::core::fmt::Debug for Identifier<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&Identifier::Any,) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Any");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Identifier::Integer(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "Integer");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Identifier::String(ref __self_0),) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "String");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'a> ::core::hash::Hash for Identifier<'a> {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match (&*self,) {
                (&Identifier::Integer(ref __self_0),) => {
                    ::core::hash::Hash::hash(&::core::intrinsics::discriminant_value(self), state);
                    ::core::hash::Hash::hash(&(*__self_0), state)
                }
                (&Identifier::String(ref __self_0),) => {
                    ::core::hash::Hash::hash(&::core::intrinsics::discriminant_value(self), state);
                    ::core::hash::Hash::hash(&(*__self_0), state)
                }
                _ => ::core::hash::Hash::hash(&::core::intrinsics::discriminant_value(self), state),
            }
        }
    }
    impl<'a> ::core::marker::StructuralEq for Identifier<'a> {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'a> ::core::cmp::Eq for Identifier<'a> {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<u64>;
                let _: ::core::cmp::AssertParamIsEq<Cow<'a, str>>;
            }
        }
    }
    impl<'a> ::core::marker::StructuralPartialEq for Identifier<'a> {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'a> ::core::cmp::PartialEq for Identifier<'a> {
        #[inline]
        fn eq(&self, other: &Identifier<'a>) -> bool {
            {
                let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (
                            &Identifier::Integer(ref __self_0),
                            &Identifier::Integer(ref __arg_1_0),
                        ) => (*__self_0) == (*__arg_1_0),
                        (&Identifier::String(ref __self_0), &Identifier::String(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
        #[inline]
        fn ne(&self, other: &Identifier<'a>) -> bool {
            {
                let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (
                            &Identifier::Integer(ref __self_0),
                            &Identifier::Integer(ref __arg_1_0),
                        ) => (*__self_0) != (*__arg_1_0),
                        (&Identifier::String(ref __self_0), &Identifier::String(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        _ => false,
                    }
                } else {
                    true
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'a> ::core::clone::Clone for Identifier<'a> {
        #[inline]
        fn clone(&self) -> Identifier<'a> {
            match (&*self,) {
                (&Identifier::Any,) => Identifier::Any,
                (&Identifier::Integer(ref __self_0),) => {
                    Identifier::Integer(::core::clone::Clone::clone(&(*__self_0)))
                }
                (&Identifier::String(ref __self_0),) => {
                    Identifier::String(::core::clone::Clone::clone(&(*__self_0)))
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'a> _serde::Serialize for Identifier<'a> {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    Identifier::Any => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "Identifier",
                        0u32,
                        "Any",
                    ),
                    Identifier::Integer(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "Identifier",
                            1u32,
                            "Integer",
                            __field0,
                        )
                    }
                    Identifier::String(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "Identifier",
                            2u32,
                            "String",
                            __field0,
                        )
                    }
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de, 'a> _serde::Deserialize<'de> for Identifier<'a> {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "variant identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"variant index 0 <= i < 3",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "Any" => _serde::__private::Ok(__Field::__field0),
                            "Integer" => _serde::__private::Ok(__Field::__field1),
                            "String" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            )),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"Any" => _serde::__private::Ok(__Field::__field0),
                            b"Integer" => _serde::__private::Ok(__Field::__field1),
                            b"String" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(_serde::de::Error::unknown_variant(
                                    __value, VARIANTS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de, 'a> {
                    marker: _serde::__private::PhantomData<Identifier<'a>>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de, 'a> _serde::de::Visitor<'de> for __Visitor<'de, 'a> {
                    type Value = Identifier<'a>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "enum Identifier")
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match match _serde::de::EnumAccess::variant(__data) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            (__Field::__field0, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(Identifier::Any)
                            }
                            (__Field::__field1, __variant) => _serde::__private::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<u64>(__variant),
                                Identifier::Integer,
                            ),
                            (__Field::__field2, __variant) => _serde::__private::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<Cow<'a, str>>(
                                    __variant,
                                ),
                                Identifier::String,
                            ),
                        }
                    }
                }
                const VARIANTS: &'static [&'static str] = &["Any", "Integer", "String"];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "Identifier",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Identifier<'a>>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
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
    impl<'a> From<String> for Identifier<'a> {
        fn from(id: String) -> Self {
            Self::String(Cow::Owned(id))
        }
    }
    /// A list of [`ActionName`]s.
    pub enum ActionNameList {
        /// A specific list of names.
        List(Vec<ActionName>),
        /// All actions.
        All,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for ActionNameList {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&ActionNameList::List(ref __self_0),) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "List");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&ActionNameList::All,) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "All");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ActionNameList {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    ActionNameList::List(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "ActionNameList",
                            0u32,
                            "List",
                            __field0,
                        )
                    }
                    ActionNameList::All => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "ActionNameList",
                        1u32,
                        "All",
                    ),
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ActionNameList {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "variant identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"variant index 0 <= i < 2",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "List" => _serde::__private::Ok(__Field::__field0),
                            "All" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            )),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"List" => _serde::__private::Ok(__Field::__field0),
                            b"All" => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(_serde::de::Error::unknown_variant(
                                    __value, VARIANTS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<ActionNameList>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ActionNameList;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "enum ActionNameList")
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match match _serde::de::EnumAccess::variant(__data) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            (__Field::__field0, __variant) => _serde::__private::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<Vec<ActionName>>(
                                    __variant,
                                ),
                                ActionNameList::List,
                            ),
                            (__Field::__field1, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(ActionNameList::All)
                            }
                        }
                    }
                }
                const VARIANTS: &'static [&'static str] = &["List", "All"];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "ActionNameList",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<ActionNameList>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl<T> From<T> for ActionNameList
    where
        T: Action,
    {
        fn from(action: T) -> Self {
            Self::List(<[_]>::into_vec(box [action.name()]))
        }
    }
    impl From<ActionName> for ActionNameList {
        fn from(name: ActionName) -> Self {
            Self::List(<[_]>::into_vec(box [name]))
        }
    }
    impl From<Vec<ActionName>> for ActionNameList {
        fn from(names: Vec<ActionName>) -> Self {
            Self::List(names)
        }
    }
    /// A unique name/identifier of a resource.
    pub struct ResourceName(Vec<Identifier<'static>>);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for ResourceName {
        #[inline]
        fn default() -> ResourceName {
            ResourceName(::core::default::Default::default())
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for ResourceName {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                ResourceName(ref __self_0_0) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "ResourceName");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ResourceName {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                _serde::Serializer::serialize_newtype_struct(__serializer, "ResourceName", &self.0)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ResourceName {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<ResourceName>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ResourceName;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "tuple struct ResourceName",
                        )
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(
                        self,
                        __e: __E,
                    ) -> _serde::__private::Result<Self::Value, __E::Error>
                    where
                        __E: _serde::Deserializer<'de>,
                    {
                        let __field0: Vec<Identifier<'static>> =
                            match <Vec<Identifier<'static>> as _serde::Deserialize>::deserialize(
                                __e,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                        _serde::__private::Ok(ResourceName(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            Vec<Identifier<'static>>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"tuple struct ResourceName with 1 element",
                                ));
                            }
                        };
                        _serde::__private::Ok(ResourceName(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(
                    __deserializer,
                    "ResourceName",
                    __Visitor {
                        marker: _serde::__private::PhantomData::<ResourceName>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Display for ResourceName {
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
    impl ResourceName {
        /// Creates a `ResourceName` that matches any identifier.
        #[must_use]
        pub fn any() -> Self {
            Self::named(Identifier::Any)
        }
        /// Creates a `ResourceName` with `name`.
        #[must_use]
        pub fn named<I: Into<Identifier<'static>>>(name: I) -> Self {
            Self(<[_]>::into_vec(box [name.into()]))
        }
        /// Adds another name segment.
        #[must_use]
        pub fn and<I: Into<Identifier<'static>>>(mut self, name: I) -> Self {
            self.0.push(name.into());
            self
        }
    }
    impl AsRef<[Identifier<'static>]> for ResourceName {
        fn as_ref(&self) -> &[Identifier<'static>] {
            &self.0
        }
    }
    impl IntoIterator for ResourceName {
        type Item = Identifier<'static>;
        type IntoIter = std::vec::IntoIter<Identifier<'static>>;
        fn into_iter(self) -> Self::IntoIter {
            self.0.into_iter()
        }
    }
}
pub use self::{
    action::{Action, ActionName},
    permissions::Permissions,
    statement::{ActionNameList, Identifier, ResourceName, Statement},
};
pub use actionable_macros::Actionable;
#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use crate::{Action, ActionName, ActionNameList, Actionable, Permissions, ResourceName, Statement};
    enum TestActions {
        DoSomething,
        Post(PostActions),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for TestActions {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&TestActions::DoSomething,) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "DoSomething");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&TestActions::Post(ref __self_0),) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Post");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    impl Action for TestActions {
        fn name(&self) -> ActionName {
            match self {
                Self::DoSomething => {
                    ActionName(<[_]>::into_vec(box [::std::borrow::Cow::Borrowed(
                        "DoSomething",
                    )]))
                }
                Self::Post(subaction) => {
                    let mut name = Action::name(subaction);
                    name.0.insert(0, ::std::borrow::Cow::Borrowed("Post"));
                    name
                }
            }
        }
    }
    enum PostActions {
        Read,
        Update,
        Delete,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for PostActions {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&PostActions::Read,) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Read");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&PostActions::Update,) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Update");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&PostActions::Delete,) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Delete");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    impl Action for PostActions {
        fn name(&self) -> ActionName {
            match self {
                Self::Read => {
                    ActionName(<[_]>::into_vec(box [::std::borrow::Cow::Borrowed("Read")]))
                }
                Self::Update => ActionName(<[_]>::into_vec(box [::std::borrow::Cow::Borrowed(
                    "Update",
                )])),
                Self::Delete => ActionName(<[_]>::into_vec(box [::std::borrow::Cow::Borrowed(
                    "Delete",
                )])),
            }
        }
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker]
    pub const basics: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::basics"),
            ignore: false,
            allow_fail: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(|| test::assert_test_result(basics())),
    };
    fn basics() {
        let statements = <[_]>::into_vec(box [
            Statement {
                resources: <[_]>::into_vec(box [ResourceName::any()]),
                actions: ActionNameList::from(TestActions::Post(PostActions::Read)),
                allowed: true,
            },
            Statement {
                resources: <[_]>::into_vec(box [ResourceName::named("all-actions-allowed")]),
                actions: ActionNameList::All,
                allowed: true,
            },
            Statement {
                resources: <[_]>::into_vec(box [ResourceName::named("only-post-actions-allowed")]),
                actions: ActionNameList::from(ActionName(<[_]>::into_vec(box [Cow::Borrowed(
                    "Post",
                )]))),
                allowed: true,
            },
        ]);
        let permissions = Permissions::from(statements);
        if !permissions.allowed_to(
            &ResourceName::named("someresource"),
            &TestActions::Post(PostActions::Read),
        ) {
            :: core :: panicking :: panic ("assertion failed: permissions.allowed_to(&ResourceName::named(\\\"someresource\\\"),\\n                       &TestActions::Post(PostActions::Read))")
        };
        if !permissions.allowed_to(
            &ResourceName::named("all-actions-allowed"),
            &TestActions::Post(PostActions::Update),
        ) {
            :: core :: panicking :: panic ("assertion failed: permissions.allowed_to(&ResourceName::named(\\\"all-actions-allowed\\\"),\\n                       &TestActions::Post(PostActions::Update))")
        };
        if !permissions.allowed_to(
            &ResourceName::named("all-actions-allowed"),
            &TestActions::DoSomething,
        ) {
            :: core :: panicking :: panic ("assertion failed: permissions.allowed_to(&ResourceName::named(\\\"all-actions-allowed\\\"),\\n                       &TestActions::DoSomething)")
        };
        if !permissions.allowed_to(
            &ResourceName::named("only-post-actions-allowed"),
            &TestActions::Post(PostActions::Delete),
        ) {
            :: core :: panicking :: panic ("assertion failed: permissions.allowed_to(&ResourceName::named(\\\"only-post-actions-allowed\\\"),\\n                       &TestActions::Post(PostActions::Delete))")
        };
        if !!permissions.allowed_to(
            &ResourceName::named("someresource"),
            &TestActions::Post(PostActions::Update),
        ) {
            :: core :: panicking :: panic ("assertion failed: !permissions.allowed_to(&ResourceName::named(\\\"someresource\\\"),\\n                        &TestActions::Post(PostActions::Update))")
        };
        if !!permissions.allowed_to(
            &ResourceName::named("someresource"),
            &TestActions::Post(PostActions::Delete),
        ) {
            :: core :: panicking :: panic ("assertion failed: !permissions.allowed_to(&ResourceName::named(\\\"someresource\\\"),\\n                        &TestActions::Post(PostActions::Delete))")
        };
        if !!permissions.allowed_to(
            &ResourceName::named("only-post-actions-allowed"),
            &TestActions::DoSomething,
        ) {
            :: core :: panicking :: panic ("assertion failed: !permissions.allowed_to(&ResourceName::named(\\\"only-post-actions-allowed\\\"),\\n                        &TestActions::DoSomething)")
        };
    }
    use crate as actionable;
    enum Request {
        UnprotectedEnumParameter(u64),
        UnprotectedNoParameters,
        #[actionable(protection = "simple")]
        SimplyPotectedEnumParameter(u64),
        #[actionable(protection = "simple")]
        SimplyProtectedNoParameters,
        #[actionable(protection = "custom")]
        CustomProtectedEnumParameter(u64),
        #[actionable(protection = "custom")]
        CustomProtectedNoParameters,
    }
    trait RequestDispatcher: Send + Sync {
        type Output: Send + Sync;
        type Error: From<actionable::PermissionDenied> + Send + Sync;
        type UnprotectedEnumParameterHandler: UnprotectedEnumParameterHandler<Dispatcher = Self>;
        type UnprotectedNoParametersHandler: UnprotectedNoParametersHandler<Dispatcher = Self>;
        type SimplyPotectedEnumParameterHandler: SimplyPotectedEnumParameterHandler<
            Dispatcher = Self,
        >;
        type SimplyProtectedNoParametersHandler: SimplyProtectedNoParametersHandler<
            Dispatcher = Self,
        >;
        type CustomProtectedEnumParameterHandler: CustomProtectedEnumParameterHandler<
            Dispatcher = Self,
        >;
        type CustomProtectedNoParametersHandler: CustomProtectedNoParametersHandler<
            Dispatcher = Self,
        >;
        #[must_use]
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn act<'life0, 'life1, 'async_trait>(
            &'life0 self,
            request: Request,
            permissions: &'life1 actionable::Permissions,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<Output = Result<Self::Output, Self::Error>>
                    + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            'life1: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) =
                    ::core::option::Option::None::<Result<Self::Output, Self::Error>>
                {
                    return __ret;
                }
                let __self = self;
                let request = request;
                let permissions = permissions;
                let __ret: Result<Self::Output, Self::Error> = {
                    match request {
                        Request::UnprotectedEnumParameter(arg0) => {
                            Self::UnprotectedEnumParameterHandler::handle(arg0).await
                        }
                        Request::UnprotectedNoParameters => {
                            Self::UnprotectedNoParametersHandler::handle().await
                        }
                        Request::SimplyPotectedEnumParameter(arg0) => {
                            Self::SimplyPotectedEnumParameterHandler::handle(permissions, arg0)
                                .await
                        }
                        Request::SimplyProtectedNoParameters => {
                            Self::SimplyProtectedNoParametersHandler::handle(permissions).await
                        }
                        Request::CustomProtectedEnumParameter(arg0) => {
                            Self::CustomProtectedEnumParameterHandler::handle(permissions, arg0)
                                .await
                        }
                        Request::CustomProtectedNoParameters => {
                            Self::CustomProtectedNoParametersHandler::handle(permissions).await
                        }
                    }
                };
                #[allow(unreachable_code)]
                __ret
            })
        }
    }
    trait UnprotectedEnumParameterHandler: Send + Sync {
        type Dispatcher: RequestDispatcher;
        #[must_use]
        #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
        fn handle<'async_trait>(
            arg0: u64,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                        Output = Result<
                            <Self::Dispatcher as RequestDispatcher>::Output,
                            <Self::Dispatcher as RequestDispatcher>::Error,
                        >,
                    > + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            Self: 'async_trait;
    }
    trait UnprotectedNoParametersHandler: Send + Sync {
        type Dispatcher: RequestDispatcher;
        #[must_use]
        #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
        fn handle<'async_trait>() -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                        Output = Result<
                            <Self::Dispatcher as RequestDispatcher>::Output,
                            <Self::Dispatcher as RequestDispatcher>::Error,
                        >,
                    > + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            Self: 'async_trait;
    }
    trait SimplyPotectedEnumParameterHandler: Send + Sync {
        type Dispatcher: RequestDispatcher;
        fn resource_name(arg0: &u64) -> actionable::ResourceName;
        type Action: actionable::Action;
        fn action() -> Self::Action;
        #[must_use]
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn handle<'life0, 'async_trait>(
            permissions: &'life0 actionable::Permissions,
            arg0: u64,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                        Output = Result<
                            <Self::Dispatcher as RequestDispatcher>::Output,
                            <Self::Dispatcher as RequestDispatcher>::Error,
                        >,
                    > + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<
                        <Self::Dispatcher as RequestDispatcher>::Output,
                        <Self::Dispatcher as RequestDispatcher>::Error,
                    >,
                > {
                    return __ret;
                }
                let permissions = permissions;
                let arg0 = arg0;
                let __ret: Result<
                    <Self::Dispatcher as RequestDispatcher>::Output,
                    <Self::Dispatcher as RequestDispatcher>::Error,
                > = {
                    if permissions.allowed_to(&Self::resource_name(&arg0), &Self::action()) {
                        Self::handle_protected(arg0).await
                    } else {
                        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                            &["not yet implemented: "],
                            &match (&::core::fmt::Arguments::new_v1(
                                &["Err(Self::Error::from(PermissionDenied))"],
                                &match () {
                                    () => [],
                                },
                            ),)
                            {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ))
                    }
                };
                #[allow(unreachable_code)]
                __ret
            })
        }
        #[must_use]
        #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
        fn handle_protected<'async_trait>(
            arg0: u64,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                        Output = Result<
                            <Self::Dispatcher as RequestDispatcher>::Output,
                            <Self::Dispatcher as RequestDispatcher>::Error,
                        >,
                    > + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            Self: 'async_trait;
    }
    trait SimplyProtectedNoParametersHandler: Send + Sync {
        type Dispatcher: RequestDispatcher;
        fn resource_name() -> actionable::ResourceName;
        type Action: actionable::Action;
        fn action() -> Self::Action;
        #[must_use]
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn handle<'life0, 'async_trait>(
            permissions: &'life0 actionable::Permissions,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                        Output = Result<
                            <Self::Dispatcher as RequestDispatcher>::Output,
                            <Self::Dispatcher as RequestDispatcher>::Error,
                        >,
                    > + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<
                        <Self::Dispatcher as RequestDispatcher>::Output,
                        <Self::Dispatcher as RequestDispatcher>::Error,
                    >,
                > {
                    return __ret;
                }
                let permissions = permissions;
                let __ret: Result<
                    <Self::Dispatcher as RequestDispatcher>::Output,
                    <Self::Dispatcher as RequestDispatcher>::Error,
                > = {
                    if permissions.allowed_to(&Self::resource_name(), &Self::action()) {
                        Self::handle_protected().await
                    } else {
                        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                            &["not yet implemented: "],
                            &match (&::core::fmt::Arguments::new_v1(
                                &["Err(Self::Error::from(PermissionDenied))"],
                                &match () {
                                    () => [],
                                },
                            ),)
                            {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ))
                    }
                };
                #[allow(unreachable_code)]
                __ret
            })
        }
        #[must_use]
        #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
        fn handle_protected<'async_trait>() -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                        Output = Result<
                            <Self::Dispatcher as RequestDispatcher>::Output,
                            <Self::Dispatcher as RequestDispatcher>::Error,
                        >,
                    > + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            Self: 'async_trait;
    }
    trait CustomProtectedEnumParameterHandler: Send + Sync {
        type Dispatcher: RequestDispatcher;
        fn is_allowed(permissions: &actionable::Permissions, arg0: &u64) -> bool;
        #[must_use]
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn handle<'life0, 'async_trait>(
            permissions: &'life0 actionable::Permissions,
            arg0: u64,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                        Output = Result<
                            <Self::Dispatcher as RequestDispatcher>::Output,
                            <Self::Dispatcher as RequestDispatcher>::Error,
                        >,
                    > + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<
                        <Self::Dispatcher as RequestDispatcher>::Output,
                        <Self::Dispatcher as RequestDispatcher>::Error,
                    >,
                > {
                    return __ret;
                }
                let permissions = permissions;
                let arg0 = arg0;
                let __ret: Result<
                    <Self::Dispatcher as RequestDispatcher>::Output,
                    <Self::Dispatcher as RequestDispatcher>::Error,
                > = {
                    if Self::is_allowed(permissions, &arg0) {
                        Self::handle_protected(arg0).await
                    } else {
                        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                            &["not yet implemented: "],
                            &match (&::core::fmt::Arguments::new_v1(
                                &["Err(Self::Error::from(PermissionDenied))"],
                                &match () {
                                    () => [],
                                },
                            ),)
                            {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ))
                    }
                };
                #[allow(unreachable_code)]
                __ret
            })
        }
        #[must_use]
        #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
        fn handle_protected<'async_trait>(
            arg0: u64,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                        Output = Result<
                            <Self::Dispatcher as RequestDispatcher>::Output,
                            <Self::Dispatcher as RequestDispatcher>::Error,
                        >,
                    > + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            Self: 'async_trait;
    }
    trait CustomProtectedNoParametersHandler: Send + Sync {
        type Dispatcher: RequestDispatcher;
        fn is_allowed(permissions: &actionable::Permissions) -> bool;
        #[must_use]
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn handle<'life0, 'async_trait>(
            permissions: &'life0 actionable::Permissions,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                        Output = Result<
                            <Self::Dispatcher as RequestDispatcher>::Output,
                            <Self::Dispatcher as RequestDispatcher>::Error,
                        >,
                    > + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<
                        <Self::Dispatcher as RequestDispatcher>::Output,
                        <Self::Dispatcher as RequestDispatcher>::Error,
                    >,
                > {
                    return __ret;
                }
                let permissions = permissions;
                let __ret: Result<
                    <Self::Dispatcher as RequestDispatcher>::Output,
                    <Self::Dispatcher as RequestDispatcher>::Error,
                > = {
                    if Self::is_allowed(permissions) {
                        Self::handle_protected().await
                    } else {
                        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                            &["not yet implemented: "],
                            &match (&::core::fmt::Arguments::new_v1(
                                &["Err(Self::Error::from(PermissionDenied))"],
                                &match () {
                                    () => [],
                                },
                            ),)
                            {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ))
                    }
                };
                #[allow(unreachable_code)]
                __ret
            })
        }
        #[must_use]
        #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
        fn handle_protected<'async_trait>() -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                        Output = Result<
                            <Self::Dispatcher as RequestDispatcher>::Output,
                            <Self::Dispatcher as RequestDispatcher>::Error,
                        >,
                    > + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            Self: 'async_trait;
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Request {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&Request::UnprotectedEnumParameter(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "UnprotectedEnumParameter");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Request::UnprotectedNoParameters,) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "UnprotectedNoParameters");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Request::SimplyPotectedEnumParameter(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "SimplyPotectedEnumParameter");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Request::SimplyProtectedNoParameters,) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "SimplyProtectedNoParameters");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Request::CustomProtectedEnumParameter(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "CustomProtectedEnumParameter");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Request::CustomProtectedNoParameters,) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "CustomProtectedNoParameters");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    struct Dispatcher;
    impl RequestDispatcher for Dispatcher {
        type Output = ();
        type Error = anyhow::Error;
        type UnprotectedEnumParameterHandler = Self;
        type UnprotectedNoParametersHandler = Self;
        type SimplyPotectedEnumParameterHandler = Self;
        type SimplyProtectedNoParametersHandler = Self;
        type CustomProtectedNoParametersHandler = Self;
        type CustomProtectedEnumParameterHandler = Self;
    }
    impl UnprotectedEnumParameterHandler for Dispatcher {
        type Dispatcher = Self;
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn handle<'async_trait>(
            arg1: u64,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<Output = Result<(), anyhow::Error>>
                    + ::core::marker::Send
                    + 'async_trait,
            >,
        > {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) =
                    ::core::option::Option::None::<Result<(), anyhow::Error>>
                {
                    return __ret;
                }
                let arg1 = arg1;
                let __ret: Result<(), anyhow::Error> =
                    { ::core::panicking::panic("not yet implemented") };
                #[allow(unreachable_code)]
                __ret
            })
        }
    }
    impl UnprotectedNoParametersHandler for Dispatcher {
        type Dispatcher = Self;
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn handle<'async_trait>() -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<Output = Result<(), anyhow::Error>>
                    + ::core::marker::Send
                    + 'async_trait,
            >,
        > {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) =
                    ::core::option::Option::None::<Result<(), anyhow::Error>>
                {
                    return __ret;
                }
                let __ret: Result<(), anyhow::Error> =
                    { ::core::panicking::panic("not yet implemented") };
                #[allow(unreachable_code)]
                __ret
            })
        }
    }
    impl SimplyPotectedEnumParameterHandler for Dispatcher {
        type Dispatcher = Self;
        type Action = TestActions;
        fn resource_name(arg1: &u64) -> ResourceName {
            ::core::panicking::panic("not yet implemented")
        }
        fn action() -> Self::Action {
            TestActions::DoSomething
        }
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn handle_protected<'async_trait>(
            arg1: u64,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<Output = Result<(), anyhow::Error>>
                    + ::core::marker::Send
                    + 'async_trait,
            >,
        > {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) =
                    ::core::option::Option::None::<Result<(), anyhow::Error>>
                {
                    return __ret;
                }
                let arg1 = arg1;
                let __ret: Result<(), anyhow::Error> =
                    { ::core::panicking::panic("not yet implemented") };
                #[allow(unreachable_code)]
                __ret
            })
        }
    }
    impl SimplyProtectedNoParametersHandler for Dispatcher {
        type Dispatcher = Self;
        type Action = TestActions;
        fn resource_name() -> ResourceName {
            ::core::panicking::panic("not yet implemented")
        }
        fn action() -> Self::Action {
            TestActions::DoSomething
        }
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn handle_protected<'async_trait>() -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<Output = Result<(), anyhow::Error>>
                    + ::core::marker::Send
                    + 'async_trait,
            >,
        > {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) =
                    ::core::option::Option::None::<Result<(), anyhow::Error>>
                {
                    return __ret;
                }
                let __ret: Result<(), anyhow::Error> =
                    { ::core::panicking::panic("not yet implemented") };
                #[allow(unreachable_code)]
                __ret
            })
        }
    }
    impl CustomProtectedNoParametersHandler for Dispatcher {
        type Dispatcher = Self;
        fn is_allowed(permissions: &Permissions) -> bool {
            ::core::panicking::panic("not yet implemented")
        }
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn handle_protected<'async_trait>() -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<Output = Result<(), anyhow::Error>>
                    + ::core::marker::Send
                    + 'async_trait,
            >,
        > {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) =
                    ::core::option::Option::None::<Result<(), anyhow::Error>>
                {
                    return __ret;
                }
                let __ret: Result<(), anyhow::Error> =
                    { ::core::panicking::panic("not yet implemented") };
                #[allow(unreachable_code)]
                __ret
            })
        }
    }
    impl CustomProtectedEnumParameterHandler for Dispatcher {
        type Dispatcher = Self;
        fn is_allowed(permissions: &Permissions, arg1: &u64) -> bool {
            ::core::panicking::panic("not yet implemented")
        }
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn handle_protected<'async_trait>(
            arg1: u64,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<Output = Result<(), anyhow::Error>>
                    + ::core::marker::Send
                    + 'async_trait,
            >,
        > {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) =
                    ::core::option::Option::None::<Result<(), anyhow::Error>>
                {
                    return __ret;
                }
                let arg1 = arg1;
                let __ret: Result<(), anyhow::Error> =
                    { ::core::panicking::panic("not yet implemented") };
                #[allow(unreachable_code)]
                __ret
            })
        }
    }
    async fn example() {
        let dispatcher = Dispatcher;
        dispatcher
            .act(
                Request::SimplyPotectedEnumParameter(1),
                &Permissions::default(),
            )
            .await
            .unwrap()
    }
}
/// An `action` was denied.
#[error("Action '{action}' was denied on resource'{resource}'")]
pub struct PermissionDenied {
    /// The resource that `action` was attempted upon.
    pub resource: ResourceName,
    /// The `action` attempted upon `resource`.
    pub action: ActionName,
}
#[allow(unused_qualifications)]
impl std::error::Error for PermissionDenied {}
#[allow(unused_qualifications)]
impl std::fmt::Display for PermissionDenied {
    #[allow(clippy::used_underscore_binding)]
    fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        #[allow(unused_imports)]
        use thiserror::private::{DisplayAsDisplay, PathAsDisplay};
        #[allow(unused_variables, deprecated)]
        let Self { resource, action } = self;
        __formatter.write_fmt(::core::fmt::Arguments::new_v1(
            &["Action \'", "\' was denied on resource\'", "\'"],
            &match (&action.as_display(), &resource.as_display()) {
                (arg0, arg1) => [
                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                ],
            },
        ))
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for PermissionDenied {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            PermissionDenied {
                resource: ref __self_0_0,
                action: ref __self_0_1,
            } => {
                let debug_trait_builder =
                    &mut ::core::fmt::Formatter::debug_struct(f, "PermissionDenied");
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "resource",
                    &&(*__self_0_0),
                );
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "action", &&(*__self_0_1));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
#[rustc_main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&basics])
}
