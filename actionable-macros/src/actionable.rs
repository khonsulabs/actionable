#![allow(clippy::default_trait_access)]

use darling::{ast, FromDeriveInput, FromField, FromMeta, FromVariant, ToTokens};
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(actionable), supports(enum_any))]
struct Actionable {
    ident: syn::Ident,
    vis: syn::Visibility,
    data: ast::Data<Variant, ()>,
}

#[derive(Debug, FromMeta)]
enum Protection {
    None,
    Simple,
    Custom,
}

impl Default for Protection {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, FromVariant)]
#[darling(attributes(actionable))]
struct Variant {
    ident: syn::Ident,
    fields: ast::Fields<Field>,

    #[darling(default)]
    protection: Protection,
}
#[derive(Debug, FromField)]
#[darling(attributes(endpoint))]
struct Field {
    ident: Option<syn::Ident>,
    ty: syn::Type,
}

impl ToTokens for Actionable {
    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let enum_data = self
            .data
            .as_ref()
            .take_enum()
            .expect("Expected enum in data");

        let pub_tokens = match self.vis {
            syn::Visibility::Public(_) => quote! { pub },
            syn::Visibility::Crate(_) => quote! { pub(crate) },
            _ => TokenStream::default(),
        };

        let mut handlers = Vec::new();
        let mut associated_types = Vec::new();
        let mut match_cases = Vec::new();

        let enum_name = &self.ident;

        let generated_dispatcher_name =
            syn::Ident::new(&format!("{}Dispatcher", enum_name), enum_name.span());

        for variant in &enum_data {
            let variant_name = &variant.ident;
            let handler_name =
                syn::Ident::new(&format!("{}Handler", variant_name), variant_name.span());

            associated_types
                .push(quote_spanned! {variant_name.span() => type #handler_name: #handler_name<Dispatcher = Self>;});

            let mut method_parameters = Vec::new();
            let mut byref_method_parameters = Vec::new();
            let mut enum_parameters = Vec::new();
            let mut is_struct_style = false;

            for (index, field) in variant.fields.iter().enumerate() {
                if let Some(ident) = &field.ident {
                    is_struct_style = true;
                } else {
                    let arg_name = syn::Ident::new(&format!("arg{}", index), variant_name.span());
                    let arg_type = &field.ty;
                    method_parameters.push(quote!(#arg_name: #arg_type));
                    byref_method_parameters.push(quote!(#arg_name: &#arg_type));
                    enum_parameters.push(arg_name);
                }
            }

            let mut handle_parameters = enum_parameters.clone();
            handle_parameters.insert(0, syn::Ident::new("self", variant_name.span()));

            let implementation = match variant.protection {
                Protection::None => quote! {
                    async fn handle(
                        dispatcher: &Self::Dispatcher,
                        #(#method_parameters),*
                    ) -> Result<
                        <Self::Dispatcher as #generated_dispatcher_name>::Output,
                        <Self::Dispatcher as #generated_dispatcher_name>::Error
                    >;
                },
                Protection::Simple => {
                    handle_parameters
                        .insert(1, syn::Ident::new("permissions", variant_name.span()));

                    quote! {
                        fn resource_name(#(#byref_method_parameters),*) -> actionable::ResourceName;
                        type Action: actionable::Action;
                        fn action() -> Self::Action;

                        async fn handle(
                            dispatcher: &Self::Dispatcher,
                            permissions: &actionable::Permissions,
                            #(#method_parameters),*
                        ) -> Result<
                            <Self::Dispatcher as #generated_dispatcher_name>::Output,
                            <Self::Dispatcher as #generated_dispatcher_name>::Error
                        > {
                            if permissions.allowed_to(&Self::resource_name(#(&#enum_parameters),*), &Self::action()) {
                                Self::handle_protected(dispatcher, #(#enum_parameters),*).await
                            } else {
                                todo!("Err(Self::Error::from(PermissionDenied))")
                            }
                        }

                        async fn handle_protected(
                            dispatcher: &Self::Dispatcher,
                            #(#method_parameters),*
                        ) -> Result<
                            <Self::Dispatcher as #generated_dispatcher_name>::Output,
                            <Self::Dispatcher as #generated_dispatcher_name>::Error
                        >;
                    }
                }
                Protection::Custom => {
                    handle_parameters
                        .insert(1, syn::Ident::new("permissions", variant_name.span()));

                    quote! {
                        fn is_allowed(permissions: &actionable::Permissions, #(#byref_method_parameters),*) -> bool;

                        async fn handle(
                            dispatcher: &Self::Dispatcher,
                            permissions: &actionable::Permissions,
                            #(#method_parameters),*
                        ) -> Result<
                            <Self::Dispatcher as #generated_dispatcher_name>::Output,
                            <Self::Dispatcher as #generated_dispatcher_name>::Error
                        > {
                            if Self::is_allowed(permissions, #(&#enum_parameters),*) {
                                Self::handle_protected(dispatcher, #(#enum_parameters),*).await
                            } else {
                                todo!("Err(Self::Error::from(PermissionDenied))")
                            }
                        }

                        async fn handle_protected(
                            dispatcher: &Self::Dispatcher,
                            #(#method_parameters),*
                        ) -> Result<
                            <Self::Dispatcher as #generated_dispatcher_name>::Output,
                            <Self::Dispatcher as #generated_dispatcher_name>::Error
                        >;
                    }
                }
            };

            handlers.push(quote_spanned! {
                variant_name.span() =>
                    #[async_trait::async_trait]
                    #pub_tokens trait #handler_name: Send + Sync {
                        type Dispatcher: #generated_dispatcher_name;

                        #implementation
                    }
            });

            if is_struct_style {
                todo!()
            } else if variant.fields.is_empty() {
                match_cases.push(quote_spanned! {
                    variant_name.span() => #enum_name::#variant_name => {
                        Self::#handler_name::handle(#(#handle_parameters),*).await
                    }
                });
            } else {
                match_cases.push(quote_spanned! {
                    variant_name.span() => #enum_name::#variant_name(#(#enum_parameters),*) => {
                        Self::#handler_name::handle(#(#handle_parameters),*).await
                    },
                });
            }
        }

        tokens.extend(quote! {
            #[async_trait::async_trait]
            #pub_tokens trait #generated_dispatcher_name: Send + Sync {
                type Output: Send + Sync;
                type Error: From<actionable::PermissionDenied> + Send + Sync;

                #(#associated_types)*

                async fn act(&self, request: #enum_name, permissions: &actionable::Permissions) -> Result<Self::Output, Self::Error> {
                    match request {
                        #(#match_cases)*
                    }
                }
            }

            #(#handlers)*


            // impl actionable::Dispatcher for #self.ident {
            //     //     async fn act(input: Self) {
            //     //         match input {
            //     //             Self::<variant> => Foo::Handler.handle(Foo::Handler::new(), permissions)
            //     //         }
            //     //     }
            //     // }
            // }

            // #(#handlers)*
        })
    }
}

pub fn derive(input: &syn::DeriveInput) -> Result<TokenStream, darling::Error> {
    let actionable = Actionable::from_derive_input(input)?;
    Ok(actionable.into_token_stream())
}
