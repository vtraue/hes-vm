use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, Lit, LitStr, Pat, Token, Variant, parse::Parse, parse_macro_input};

// test_valid_wat {valid_param_count, "source"}

struct ValidSourceInfo {
    fn_name: Ident,
    source: LitStr,
}
impl Parse for ValidSourceInfo {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let fn_name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let source: LitStr = input.parse()?;
        Ok(Self { fn_name, source })
    }
}
#[proc_macro]
pub fn test_valid_wast(input: TokenStream) -> TokenStream {
    let ValidSourceInfo { fn_name, source } = parse_macro_input!(input as ValidSourceInfo);

    let expanded = quote! {
        #[test]
        fn #fn_name() -> Result<(), ReadAndValidateError> {
            let src = #source;
            Ok(_ = read_and_validate_wat(src)?)
        }
    };
    TokenStream::from(expanded)
}

struct InvalidSourceInfo {
    fn_name: Ident,
    source: LitStr,
    err: syn::Pat,
}
impl Parse for InvalidSourceInfo {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let fn_name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let source: LitStr = input.parse()?;
        input.parse::<Token![,]>()?;
        let err = syn::Pat::parse_single(input)?;
        Ok(Self {
            fn_name,
            source,
            err,
        })
    }
}

#[proc_macro]
pub fn test_invalid_wast(input: TokenStream) -> TokenStream {
    let InvalidSourceInfo {
        fn_name,
        source,
        err,
    } = parse_macro_input!(input as InvalidSourceInfo);

    let expanded = quote! {
        #[test]
        fn #fn_name() -> Result<(), ReadAndValidateError> {
            let src = #source;
            let err = read_and_validate_wat(src);
            assert!(matches!(err.unwrap_err(), ReadAndValidateError::ValidationError(#err)));
            Ok(())
        }
    };
    TokenStream::from(expanded)
}
