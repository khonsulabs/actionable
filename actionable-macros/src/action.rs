#![allow(clippy::default_trait_access)]

use darling::{ast, FromDeriveInput, FromField, FromVariant, ToTokens};
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::quote;

use crate::{Actionable, ActionableArgs};

#[derive(Debug, FromDeriveInput)]
#[darling(supports(enum_any))]
struct Action {
    ident: syn::Ident,
    generics: syn::Generics,
    data: ast::Data<Variant, ()>,

    /// Overrides the crate name for `actionable` references.
    #[darling(skip)]
    actionable: Option<Actionable>,
}

#[derive(Debug, FromVariant)]
struct Variant {
    ident: syn::Ident,
    fields: ast::Fields<Field>,
}

#[derive(Debug, FromField)]
struct Field {}

impl ToTokens for Action {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.ident;
        let enum_data = self
            .data
            .as_ref()
            .take_enum()
            .expect("Expected enum in data");

        let actionable = self.actionable.clone().map_or_else(
            || {
                let mut segments = syn::punctuated::Punctuated::new();
                segments.push_value(syn::PathSegment {
                    ident: syn::Ident::new("actionable", name.span()),
                    arguments: syn::PathArguments::None,
                });
                syn::Path {
                    leading_colon: None,
                    segments,
                }
            },
            |a| a.0,
        );
        let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();

        let variants = enum_data.into_iter().map(|variant| {
			let ident = variant.ident.clone();
			let ident_as_string = ident.to_string();
			match variant.fields.len() {
				0 => {
					quote! {
						Self::#ident => #actionable::ActionName(vec![::std::borrow::Cow::Borrowed(#ident_as_string)])
					}
				}
				1 => {
					quote! {
						Self::#ident(subaction) => {
							let mut name = Action::name(subaction);
							name.0.insert(0, ::std::borrow::Cow::Borrowed(#ident_as_string));
							name
						}
					}
				}
				_ => {
					abort!(
						variant.ident,
						"For derive(Action), all enum variants may have at most 1 field"
					)
				}
			}
		});

        tokens.extend(quote! {
            impl#impl_generics Action for #name#type_generics #where_clause {
                fn name(&self) -> #actionable::ActionName {
                    match self {
                        #(
                            #variants
                        ),*
                    }
                }
            }
        });
    }
}

pub fn derive(input: &syn::DeriveInput) -> Result<TokenStream, darling::Error> {
    let mut actionable = Action::from_derive_input(input)?;

    if let Some(attr) = input
        .attrs
        .iter()
        .find(|attr| attr.path.segments.first().unwrap().ident == "action")
    {
        let args: ActionableArgs = syn::parse2(attr.tokens.clone())?;
        actionable.actionable = args.0;
    }

    Ok(actionable.into_token_stream())
}
