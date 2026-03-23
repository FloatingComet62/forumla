use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(Params)]
pub fn derive_params(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(f) => &f.named,
            _ => panic!("#[derive(Params)] only supports named fields"),
        },
        _ => panic!("#[derive(Params)] only supports structs"),
    };

    let pair_exprs = fields.iter().map(|f| {
        let ident = f.ident.as_ref().unwrap();
        let key = ident.to_string();

        quote! {
            self.#ident.clone().map(|v| format!(
                "{}={}",
                urlencoding::encode(#key),
                urlencoding::encode(&v.to_string())
            ))
        }
    });

    let expanded = quote! {
        impl #name {
            pub fn search_params(&self) -> String {
                let parts: Vec<String> = vec![
                    #(#pair_exprs),*
                ].into_iter().flatten().collect();
                let output = parts.join("&");
                if output.len() > 0 {
                    return "?".to_string() + &output;
                }
                output
            }
        }
    };

    expanded.into()
}

#[proc_macro_attribute]
pub fn params(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let vis = &input.vis;

    let all_optional = attr.to_string().contains("all_optional");

    let fields = match &input.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(f) => &f.named,
            _ => panic!("#[params] only supports named fields"),
        },
        _ => panic!("#[params] only supports structs"),
    };

    let rewritten_fields = fields.iter().map(|f| {
        let ident = &f.ident;
        let fvis = &f.vis;
        let ty = &f.ty;
        if all_optional {
            quote! { #fvis #ident: Option<#ty> }
        } else {
            quote! { #fvis #ident: #ty }
        }
    });

    quote! {
        #vis struct #name {
            #(#rewritten_fields),*
        }
    }
    .into()
}
