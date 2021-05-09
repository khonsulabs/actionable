#![allow(clippy::default_trait_access)]

use darling::{FromDeriveInput, ToTokens};
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(dispatcher), supports(any))]
struct Dispatcher {
    ident: syn::Ident,
    vis: syn::Visibility,
    generics: syn::Generics,

    /// Overrides the crate name for `actionable` references.
    #[darling(default)]
    actionable: Option<String>,
    /// The enum that had `Actionable` derived on it.
    input: String,
}

impl ToTokens for Dispatcher {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let type_name = &self.ident;
        let generics = &self.generics;
        let actionable = self.actionable.as_deref().unwrap_or("actionable");
        let actionable = syn::Ident::new(actionable, type_name.span());

        for name in self.input.split(',').map(str::trim) {
            let enum_name = syn::Ident::new(name, type_name.span());
            let generated_dispatcher_name =
                syn::Ident::new(&format!("{}Dispatcher", enum_name), enum_name.span());

            tokens.extend(quote! {
                #[#actionable::async_trait]
                impl#generics #actionable::Dispatcher<#enum_name> for #type_name#generics {
                    type Result = Result<<Self as #generated_dispatcher_name>::Output,<Self as #generated_dispatcher_name>::Error>;

                    async fn dispatch(&self, permissions: &#actionable::Permissions, request: #enum_name) -> Self::Result {
                        #generated_dispatcher_name::dispatch_to_handlers(self, permissions, request).await
                    }
                }
            });
        }
    }
}

pub fn derive(input: &syn::DeriveInput) -> Result<TokenStream, darling::Error> {
    let dispatcher = Dispatcher::from_derive_input(input)?;
    Ok(dispatcher.into_token_stream())
}
