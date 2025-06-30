use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
    Data, DeriveInput, GenericParam, Generics, parse_macro_input, parse_quote, spanned::Spanned,
    token::Token,
};

#[proc_macro_derive(FromBytecode)]
pub fn derive_from_bytecode(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let struct_impl = from_reader(&input.data);
    let expanded = quote! {
        impl #impl_generics FromBytecode for #name #ty_generics #where_clause {
            fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
                #struct_impl
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}

fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(FromBytecode));
        }
    }
    generics
}

fn from_reader(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match &data.fields {
            syn::Fields::Named(fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote! {#name: FromBytecode::from_reader(reader)?}
                });
                quote! {
                    Ok(Self {#(#recurse,)*})
                }
            }
            syn::Fields::Unnamed(_) => todo!(),
            syn::Fields::Unit => todo!(),
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}
