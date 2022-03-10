#![allow(clippy::default_trait_access)]

use darling::{FromDeriveInput, ToTokens};
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::quote;
use syn::{parse::Parse, punctuated::Punctuated};

use crate::{actionable, Error};

#[derive(Debug, FromDeriveInput)]
#[darling(supports(any))]
struct Dispatcher {
    ident: syn::Ident,
    generics: syn::Generics,
    #[darling(skip)]
    args: Option<Args>,
}

#[derive(Debug, Default)]
struct Args {
    inputs: Vec<syn::Path>,
    actionable: Option<syn::Path>,
    asynchronous: bool,
}

impl Parse for Args {
    fn parse(input: &'_ syn::parse::ParseBuffer<'_>) -> syn::Result<Self> {
        let content;
        let _ = syn::parenthesized!(content in input);
        let content: Punctuated<Arg, syn::Token![,]> = content.parse_terminated(Arg::parse)?;
        let mut args = Self::default();
        for arg in content {
            match arg {
                Arg::Actionable(actionable) => {
                    args.actionable = Some(actionable);
                }
                Arg::Input(path) => {
                    args.inputs.push(path);
                }
            }
        }

        Ok(args)
    }
}

impl ToTokens for Dispatcher {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let type_name = &self.ident;

        let args = self.args.as_ref().unwrap();

        let actionable = actionable(args.actionable.clone(), type_name.span());

        let (dispatcher_trait, async_keyword, await_suffix, async_trait_attribute) =
            if args.asynchronous {
                (
                    quote!(AsyncDispatcher),
                    quote!(async),
                    quote!(.await),
                    quote!(#[#actionable::async_trait]),
                )
            } else {
                (quote!(Dispatcher), quote!(), quote!(), quote!())
            };

        let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();

        for enum_type in &args.inputs {
            let generated_dispatcher_name = syn::Ident::new(
                &format!("{}Dispatcher", enum_type.segments.last().unwrap().ident),
                type_name.span(),
            );

            tokens.extend(quote! {
                #async_trait_attribute
                impl#impl_generics #actionable::#dispatcher_trait<#enum_type> for #type_name#type_generics #where_clause {
                    type Result = Result<<Self as #generated_dispatcher_name>::Output,<Self as #generated_dispatcher_name>::Error>;

                    #async_keyword fn dispatch(&self, permissions: &#actionable::Permissions, request: #enum_type) -> Self::Result {
                        #generated_dispatcher_name::dispatch_to_handlers(self, permissions, request)#await_suffix
                    }
                }
            });
        }
    }
}

#[allow(clippy::redundant_pub_crate)] // Error is a private type
pub(crate) fn derive(input: &syn::DeriveInput, asynchronous: bool) -> Result<TokenStream, Error> {
    let mut dispatcher = Dispatcher::from_derive_input(input)?;
    let attr = match input
        .attrs
        .iter()
        .find(|attr| attr.path.segments.first().unwrap().ident == "dispatcher")
    {
        Some(attr) => attr,
        None => abort!(input.ident, "missing `dispatcher` attribute"),
    };
    let mut args: Args = syn::parse2(attr.tokens.clone())?;
    args.asynchronous = asynchronous;
    dispatcher.args = Some(args);
    Ok(dispatcher.into_token_stream())
}

enum Arg {
    Actionable(syn::Path),
    Input(syn::Path),
}

impl Parse for Arg {
    fn parse(input: &'_ syn::parse::ParseBuffer<'_>) -> syn::Result<Self> {
        let ident: syn::Ident = input.parse()?;
        match ident.to_string().as_str() {
            "actionable" => {
                let _: syn::Token![=] = input.parse()?;
                Ok(Self::Actionable(input.parse()?))
            }
            "input" => {
                let _: syn::Token![=] = input.parse()?;
                Ok(Self::Input(input.parse()?))
            }
            _ => abort!(ident, "unknown parameter"),
        }
    }
}
