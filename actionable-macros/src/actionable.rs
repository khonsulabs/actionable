#![allow(clippy::default_trait_access)]

use darling::{ast, FromDeriveInput, FromField, FromMeta, FromVariant, ToTokens};
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::{quote, quote_spanned};

use crate::{actionable, ActionableArgs};

#[derive(Debug, FromDeriveInput)]
#[darling(supports(enum_any))]
struct Actionable {
    ident: syn::Ident,
    vis: syn::Visibility,
    data: ast::Data<Variant, ()>,

    /// Overrides the crate name for `actionable` references.
    #[darling(skip)]
    actionable: Option<ActionableArgs>,
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
    #[darling(default)]
    subaction: bool,
}

struct VariantResult {
    handler: VariantHandler,
    match_case: TokenStream,
}

enum VariantHandler {
    Subaction,
    Handler {
        handler: TokenStream,
        name: syn::Ident,
    },
}

#[derive(Default)]
struct Handler {
    tokens: TokenStream,
    parameters: Vec<syn::Ident>,
}

struct Context<'a> {
    enum_name: &'a syn::Ident,
    generated_dispatcher_name: &'a syn::Ident,
    pub_tokens: &'a TokenStream,
    actionable: &'a syn::Path,
    async_keyword: &'a TokenStream,
    await_suffix: &'a TokenStream,
    async_trait_attribute: &'a TokenStream,
}

impl Variant {
    pub fn generate_code(&self, context: &Context<'_>) -> VariantResult {
        let variant_name = &self.ident;
        let handler_name =
            syn::Ident::new(&format!("{}Handler", variant_name), variant_name.span());

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
            let arg_name = field.ident.as_ref().map_or_else(
                || syn::Ident::new(&format!("arg{}", index), variant_name.span()),
                |ident| {
                    is_struct_style = true;
                    ident.clone()
                },
            );
            let arg_type = &field.ty;
            method_parameters.push(quote!(#arg_name: #arg_type));
            byref_method_parameters.push(quote!(#arg_name: &#byref_lifetime #arg_type));
            enum_parameters.push(arg_name);
        }

        let handler = if self.subaction {
            if self.fields.len() != 1 {
                abort!(self.ident, "subactions should only have one field")
            }

            Handler::default()
        } else {
            self.generate_handler(
                &handler_name,
                &enum_parameters,
                &method_parameters,
                &byref_method_parameters,
                context,
            )
        };

        let match_case = self.generate_match_case(
            is_struct_style,
            &handler_name,
            &enum_parameters,
            &handler.parameters,
            context,
        );

        let handler = if self.subaction {
            VariantHandler::Subaction
        } else {
            VariantHandler::Handler {
                handler: handler.tokens,
                name: handler_name,
            }
        };

        VariantResult {
            handler,
            match_case,
        }
    }

    #[allow(clippy::cognitive_complexity)]
    // The complexity is because of the multiple big quote! macros, but I don't see a way to make
    // this much easier to read
    #[allow(clippy::too_many_arguments)] // TODO maybe refactor?
    fn generate_handler(
        &self,
        handler_name: &syn::Ident,
        enum_parameters: &[syn::Ident],
        method_parameters: &[TokenStream],
        byref_method_parameters: &[TokenStream],
        context: &Context<'_>,
    ) -> Handler {
        let variant_name = &self.ident;

        let mut handle_parameters = enum_parameters.to_vec();
        handle_parameters.insert(0, syn::Ident::new("self", variant_name.span()));
        handle_parameters.insert(1, syn::Ident::new("permissions", variant_name.span()));

        let generated_dispatcher_name = context.generated_dispatcher_name;
        let self_as_dispatcher = quote! {<Self as #generated_dispatcher_name>};
        let result_type = quote!(Result<
            #self_as_dispatcher::Output,
            #self_as_dispatcher::Error
        >);
        let async_keyword = context.async_keyword;
        let actionable = context.actionable;
        let await_suffix = context.await_suffix;
        let implementation = match self.protection {
            Protection::None => quote! {
                #[allow(clippy::too_many_arguments)]
                #async_keyword fn handle(
                    &self,
                    permissions: &#actionable::Permissions,
                    #(#method_parameters),*
                ) -> #result_type;
            },
            Protection::Simple => {
                quote! {
                    #[allow(clippy::ptr_arg, clippy::too_many_arguments)]
                    #async_keyword fn resource_name<'a>(&'a self,#(#byref_method_parameters),*) -> Result<#actionable::ResourceName<'a>, #self_as_dispatcher::Error>;
                    type Action: #actionable::Action;
                    fn action() -> Self::Action;

                    #[allow(clippy::too_many_arguments)]
                    #async_keyword fn handle(
                        &self,
                        permissions: &#actionable::Permissions,
                        #(#method_parameters),*
                    ) -> #result_type {
                        let resource = self.resource_name(#(&#enum_parameters),*)#await_suffix?;
                        let action = Self::action();
                        permissions.check(&resource, &action)?;
                        self.handle_protected(permissions, #(#enum_parameters),*)#await_suffix
                    }

                    #[allow(clippy::too_many_arguments)]
                    #async_keyword fn handle_protected(
                        &self,
                        permissions: &#actionable::Permissions,
                        #(#method_parameters),*
                    ) -> #result_type;
                }
            }
            Protection::Custom => {
                quote! {
                    #[allow(clippy::ptr_arg, clippy::too_many_arguments)]
                    #async_keyword fn verify_permissions(&self, permissions: &#actionable::Permissions, #(#byref_method_parameters),*) -> Result<(), #self_as_dispatcher::Error>;

                    #[allow(clippy::too_many_arguments)]
                    #async_keyword fn handle(
                        &self,
                        permissions: &#actionable::Permissions,
                        #(#method_parameters),*
                    ) -> #result_type {
                        self.verify_permissions(permissions, #(&#enum_parameters),*)#await_suffix?;
                        self.handle_protected(permissions, #(#enum_parameters),*)#await_suffix
                    }

                    #[allow(clippy::too_many_arguments)]
                    #async_keyword fn handle_protected(
                        &self,
                        permissions: &#actionable::Permissions,
                        #(#method_parameters),*
                    ) -> #result_type;
                }
            }
        };

        let async_trait_attribute = context.async_trait_attribute;
        let pub_tokens = context.pub_tokens;
        Handler {
            parameters: handle_parameters,
            tokens: quote_spanned! {
                variant_name.span() =>
                    #async_trait_attribute
                    #[doc(hidden)]
                    #pub_tokens trait #handler_name: #generated_dispatcher_name  {
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
        context: &Context<'_>,
    ) -> TokenStream {
        let variant_name = &self.ident;
        let enum_name = context.enum_name;
        let await_suffix = context.await_suffix;
        if self.subaction {
            quote_spanned! {
                variant_name.span() => #enum_name::#variant_name(arg0) => {
                    self.handle_subaction(permissions, #(#enum_parameters),*)#await_suffix
                },
            }
        } else if is_struct_style {
            quote_spanned! {
                variant_name.span() => #enum_name::#variant_name{#(#enum_parameters),*} => {
                    <Self as #handler_name>::handle(#(#handle_parameters),*)#await_suffix
                },
            }
        } else if self.fields.is_empty() {
            quote_spanned! {
                variant_name.span() => #enum_name::#variant_name => {
                    <Self as #handler_name>::handle(#(#handle_parameters),*)#await_suffix
                }
            }
        } else {
            quote_spanned! {
                variant_name.span() => #enum_name::#variant_name(#(#enum_parameters),*) => {
                    <Self as #handler_name>::handle(#(#handle_parameters),*)#await_suffix
                },
            }
        }
    }
}

#[derive(Debug, FromField)]
struct Field {
    ident: Option<syn::Ident>,
    ty: syn::Type,
}

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

        let args = self.actionable.clone().unwrap_or_default();
        let actionable = actionable(args.actionable, enum_name.span());

        let mut handlers = Vec::new();
        let mut handler_names = Vec::new();
        let mut match_cases = Vec::new();

        let generated_dispatcher_name =
            syn::Ident::new(&format!("{}Dispatcher", enum_name), enum_name.span());

        let mut subaction = false;

        let (async_keyword, await_suffix, async_trait_attribute) = if args.asynchronous {
            (
                quote!(async),
                quote!(.await),
                quote!(#[#actionable::async_trait]),
            )
        } else {
            (quote!(), quote!(), quote!())
        };

        let context = Context {
            enum_name,
            generated_dispatcher_name: &generated_dispatcher_name,
            pub_tokens: &pub_tokens,
            actionable: &actionable,
            async_keyword: &async_keyword,
            await_suffix: &await_suffix,
            async_trait_attribute: &async_trait_attribute,
        };

        for variant in enum_data {
            let result = variant.generate_code(&context);
            match result.handler {
                VariantHandler::Subaction => {
                    if subaction {
                        abort!(self.ident, "only one subaction is allowed")
                    }
                    subaction = true;
                }
                VariantHandler::Handler { handler, name } => {
                    handlers.push(handler);
                    handler_names.push(name);
                }
            }
            match_cases.push(result.match_case);
        }

        let (subaction_type, subaction_handler) = if subaction {
            (quote!(<Self::Subaction>), quote! {
                type Subaction: Send;
                #async_keyword fn handle_subaction(&self, permissions: &#actionable::Permissions, subaction: Self::Subaction) -> Result<Self::Output, Self::Error>;
            })
        } else {
            (TokenStream::default(), TokenStream::default())
        };

        tokens.extend(quote! {
            #async_trait_attribute
            #[doc(hidden)]
            #pub_tokens trait #generated_dispatcher_name: Send + Sync {
                type Output: Send + Sync;
                type Error: From<#actionable::PermissionDenied> + Send + Sync;

                #async_keyword fn dispatch_to_handlers(&self, permissions: &#actionable::Permissions, request: #enum_name#subaction_type) -> Result<Self::Output, Self::Error>
                where Self: #(#handler_names)+* {
                    match request {
                        #(#match_cases)*
                    }
                }

                #subaction_handler
            }

            #(#handlers)*
        });
    }
}

pub fn derive(input: &syn::DeriveInput) -> Result<TokenStream, darling::Error> {
    let mut actionable = Actionable::from_derive_input(input)?;

    if let Some(attr) = input
        .attrs
        .iter()
        .find(|attr| attr.path.segments.first().unwrap().ident == "actionable")
    {
        let args: ActionableArgs = syn::parse2(attr.tokens.clone())?;
        actionable.actionable = Some(args);
    }

    Ok(actionable.into_token_stream())
}
