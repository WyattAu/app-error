use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Derive macro for `AppError` trait.
///
/// Generates `code()`, `recovery_class()`, `user_message()`, and `kind()` implementations
/// from enum variant attributes.
///
/// # Example
/// ```ignore
/// use error_classify::AppError;
///
/// #[derive(Debug, thiserror::Error, AppError)]
/// pub enum MyError {
///     #[error("io error: {0}")]
///     #[app_error(code = "Internal", recovery = "Retryable")]
///     Io(String),
///
///     #[error("not found: {0}")]
///     #[app_error(code = "NotFound", recovery = "Permanent")]
///     NotFound(String),
///
///     #[error("auth error: {0}")]
///     #[app_error(code = "Auth", recovery = "UserAction")]
///     Auth(String),
/// }
/// ```
#[proc_macro_derive(AppError, attributes(app_error))]
pub fn derive_app_error(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let variants = match &input.data {
        syn::Data::Enum(data) => &data.variants,
        _ => {
            return syn::Error::new_spanned(&input, "AppError can only be derived for enums")
                .to_compile_error()
                .into();
        }
    };

    let mut code_arms = Vec::new();
    let mut recovery_arms = Vec::new();
    let mut kind_arms = Vec::new();

    for variant in variants {
        let variant_name = &variant.ident;
        let variant_name_str = variant_name.to_string();

        let mut code = "Internal".to_string();
        let mut recovery = "Permanent".to_string();

        for attr in &variant.attrs {
            if !attr.path().is_ident("app_error") {
                continue;
            }
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("code") {
                    let value = meta.value()?;
                    let lit: syn::LitStr = value.parse()?;
                    code = lit.value();
                    Ok(())
                } else if meta.path.is_ident("recovery") {
                    let value = meta.value()?;
                    let lit: syn::LitStr = value.parse()?;
                    recovery = lit.value();
                    Ok(())
                } else {
                    Err(meta.error("expected `code` or `recovery`"))
                }
            });
        }

        let code_ident = syn::Ident::new(&code, variant_name.span());
        let recovery_ident = syn::Ident::new(&recovery, variant_name.span());

        code_arms.push(quote! {
            Self::#variant_name(..) => error_classify::ErrorCode::#code_ident,
        });

        recovery_arms.push(quote! {
            Self::#variant_name(..) => error_classify::RecoveryClass::#recovery_ident,
        });

        kind_arms.push(quote! {
            Self::#variant_name(..) => #variant_name_str,
        });
    }

    let expanded = quote! {
        impl error_classify::AppError for #name {
            #[cfg(feature = "errcode")]
            fn code(&self) -> error_classify::ErrorCode {
                match self {
                    #(#code_arms)*
                }
            }

            fn recovery_class(&self) -> error_classify::RecoveryClass {
                match self {
                    #(#recovery_arms)*
                }
            }

            fn user_message(&self) -> String {
                self.to_string()
            }

            fn kind(&self) -> &'static str {
                match self {
                    #(#kind_arms)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
