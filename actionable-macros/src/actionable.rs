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

    /// Overrides the crate name for `actionable` references.
    #[darling(default)]
    actionable: Option<String>,
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

    protection: Protection,
}

struct VariantResult {
    handler: TokenStream,
    associated_type: TokenStream,
    match_case: TokenStream,
}

struct Handler {
    tokens: TokenStream,
    parameters: Vec<syn::Ident>,
}

impl Variant {
    pub fn generate_code(
        &self,
        enum_name: &syn::Ident,
        generated_dispatcher_name: &syn::Ident,
        pub_tokens: &TokenStream,
        actionable: &syn::Ident,
    ) -> VariantResult {
        let variant_name = &self.ident;
        let handler_name =
            syn::Ident::new(&format!("{}Handler", variant_name), variant_name.span());

        let associated_type = quote_spanned! {variant_name.span() => type #handler_name: #handler_name<Dispatcher = Self>;};

        let mut method_parameters = Vec::new();
        let mut byref_method_parameters = Vec::new();
        let mut enum_parameters = Vec::new();
        let mut is_struct_style = false;
        let byref_lifetime = if matches!(self.protection, Protection::Simple) {
            quote!('a)
        } else {
            TokenStream::default()
        };

        for (index, field) in self.fields.iter().enumerate() {
            let arg_name = if let Some(ident) = &field.ident {
                is_struct_style = true;
                ident.clone()
            } else {
                syn::Ident::new(&format!("arg{}", index), variant_name.span())
            };
            let arg_type = &field.ty;
            method_parameters.push(quote!(#arg_name: #arg_type));
            byref_method_parameters.push(quote!(#arg_name: &#byref_lifetime #arg_type));
            enum_parameters.push(arg_name);
        }

        let handler = self.generate_handler(
            &handler_name,
            generated_dispatcher_name,
            &enum_parameters,
            &method_parameters,
            &byref_method_parameters,
            pub_tokens,
            actionable,
        );

        let match_case = self.generate_match_case(
            is_struct_style,
            &handler_name,
            &enum_parameters,
            &handler.parameters,
            enum_name,
        );

        VariantResult {
            handler: handler.tokens,
            associated_type,
            match_case,
        }
    }

    #[allow(clippy::cognitive_complexity)]
    // The complexity is because of the multiple big quote! macros, but I don't see a way to make this much easier to read
    #[allow(clippy::too_many_arguments)] // TODO maybe refactor?
    fn generate_handler(
        &self,
        handler_name: &syn::Ident,
        generated_dispatcher_name: &syn::Ident,
        enum_parameters: &[syn::Ident],
        method_parameters: &[TokenStream],
        byref_method_parameters: &[TokenStream],
        pub_tokens: &TokenStream,
        actionable: &syn::Ident,
    ) -> Handler {
        let variant_name = &self.ident;

        let mut handle_parameters = enum_parameters.to_vec();
        handle_parameters.insert(0, syn::Ident::new("self", variant_name.span()));
        handle_parameters.insert(1, syn::Ident::new("permissions", variant_name.span()));

        let self_as_dispatcher = quote! {<Self::Dispatcher as #generated_dispatcher_name>};
        let result_type = quote!(Result<
            #self_as_dispatcher::Output,
            #self_as_dispatcher::Error
        >);

        let implementation = match self.protection {
            Protection::None => quote! {
                async fn handle(
                    dispatcher: &Self::Dispatcher,
                    permissions: &#actionable::Permissions,
                    #(#method_parameters),*
                ) -> #result_type;
            },
            Protection::Simple => {
                let permission_denied_error = quote!(Err(#self_as_dispatcher::Error::from(#actionable::PermissionDenied {
                    resource: resource.to_owned(),
                    action: #actionable::Action::name(&action),
                })));

                quote! {
                    #[allow(clippy::ptr_arg)]
                    fn resource_name<'a>(dispatcher: &'a Self::Dispatcher,#(#byref_method_parameters),*) -> #actionable::ResourceName<'a>;
                    type Action: #actionable::Action;
                    fn action() -> Self::Action;

                    async fn handle(
                        dispatcher: &Self::Dispatcher,
                        permissions: &#actionable::Permissions,
                        #(#method_parameters),*
                    ) -> #result_type {
                        let resource = Self::resource_name(dispatcher, #(&#enum_parameters),*);
                        let action = Self::action();
                        if permissions.allowed_to(&resource, &action) {
                            Self::handle_protected(dispatcher, permissions, #(#enum_parameters),*).await
                        } else {
                            #permission_denied_error
                        }
                    }

                    async fn handle_protected(
                        dispatcher: &Self::Dispatcher,
                        permissions: &#actionable::Permissions,
                        #(#method_parameters),*
                    ) -> #result_type;
                }
            }
            Protection::Custom => {
                quote! {
                    #[allow(clippy::ptr_arg)]
                    async fn verify_permissions(dispatcher: &Self::Dispatcher, permissions: &#actionable::Permissions, #(#byref_method_parameters),*) -> Result<(), #self_as_dispatcher::Error>;

                    async fn handle(
                        dispatcher: &Self::Dispatcher,
                        permissions: &#actionable::Permissions,
                        #(#method_parameters),*
                    ) -> #result_type {
                        Self::verify_permissions(dispatcher, permissions, #(&#enum_parameters),*).await?;
                        Self::handle_protected(dispatcher, permissions, #(#enum_parameters),*).await
                    }

                    async fn handle_protected(
                        dispatcher: &Self::Dispatcher,
                        permissions: &#actionable::Permissions,
                        #(#method_parameters),*
                    ) -> #result_type;
                }
            }
        };

        Handler {
            parameters: handle_parameters,
            tokens: quote_spanned! {
                variant_name.span() =>
                    #[#actionable::async_trait]
                    #[doc(hidden)]
                    #pub_tokens trait #handler_name: Send + Sync {
                        type Dispatcher: #generated_dispatcher_name;

                        #implementation
                    }
            },
        }
    }

    fn generate_match_case(
        &self,
        is_struct_style: bool,
        handler_name: &syn::Ident,
        enum_parameters: &[syn::Ident],
        handle_parameters: &[syn::Ident],
        enum_name: &syn::Ident,
    ) -> TokenStream {
        let variant_name = &self.ident;
        if is_struct_style {
            quote_spanned! {
                variant_name.span() => #enum_name::#variant_name{#(#enum_parameters),*} => {
                    Self::#handler_name::handle(#(#handle_parameters),*).await
                },
            }
        } else if self.fields.is_empty() {
            quote_spanned! {
                variant_name.span() => #enum_name::#variant_name => {
                    Self::#handler_name::handle(#(#handle_parameters),*).await
                }
            }
        } else {
            quote_spanned! {
                variant_name.span() => #enum_name::#variant_name(#(#enum_parameters),*) => {
                    Self::#handler_name::handle(#(#handle_parameters),*).await
                },
            }
        }
    }
}

#[derive(Debug, FromField)]
#[darling(attributes(endpoint))]
struct Field {
    ident: Option<syn::Ident>,
    ty: syn::Type,
}

impl Field {}

impl ToTokens for Actionable {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let enum_name = &self.ident;
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

        let actionable = self.actionable.as_deref().unwrap_or("actionable");
        let actionable = syn::Ident::new(actionable, enum_name.span());

        let mut handlers = Vec::new();
        let mut associated_types = Vec::new();
        let mut match_cases = Vec::new();

        let generated_dispatcher_name =
            syn::Ident::new(&format!("{}Dispatcher", enum_name), enum_name.span());

        for variant in &enum_data {
            let result = variant.generate_code(
                enum_name,
                &generated_dispatcher_name,
                &pub_tokens,
                &actionable,
            );
            handlers.push(result.handler);
            associated_types.push(result.associated_type);
            match_cases.push(result.match_case);
        }

        tokens.extend(quote! {
            #[#actionable::async_trait]
            #[doc(hidden)]
            #pub_tokens trait #generated_dispatcher_name: Send + Sync {
                type Output: Send + Sync;
                type Error: From<#actionable::PermissionDenied> + Send + Sync;

                #(#associated_types)*

                async fn dispatch_to_handlers(&self, permissions: &#actionable::Permissions, request: #enum_name) -> Result<Self::Output, Self::Error> {
                    match request {
                        #(#match_cases)*
                    }
                }
            }

            #(#handlers)*
        })
    }
}

pub fn derive(input: &syn::DeriveInput) -> Result<TokenStream, darling::Error> {
    let actionable = Actionable::from_derive_input(input)?;
    Ok(actionable.into_token_stream())
}
