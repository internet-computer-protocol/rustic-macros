extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
//use syn::punctuated::Punctuated;
use syn::{parse_macro_input, AttributeArgs, ItemFn, NestedMeta};

#[proc_macro_attribute]
pub fn modifiers(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let func = parse_macro_input!(input as ItemFn);

    // Capture asyncness and visibility
    let asyncness = &func.sig.asyncness;
    let vis = &func.vis;

    // Transform attribute arguments to strings
    let mut modifiers: Vec<(String, Vec<String>)> = Vec::new();
    for arg in args {
        if let NestedMeta::Lit(syn::Lit::Str(lit)) = arg {
            let val = lit.value();
            let parts: Vec<_> = val.split("@").collect();
            let func_name = parts[0].to_string();
            let params = if parts.len() > 1 {
                parts[1..]
                    .join("@")
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect()
            } else {
                Vec::new()
            };
            modifiers.push((func_name, params));
        }
    }

    // The function name and parameters
    let fn_name = &func.sig.ident;
    let fn_params = &func.sig.inputs;

    // Extract statements from the original block
    let fn_stmts = &func.block.stmts;

    // Generate each modifier check and function call
    let modifier_checks: Vec<proc_macro2::TokenStream> = modifiers
        .iter()
        .map(|(modi, params)| {
            let modi_ident: syn::Ident = syn::Ident::new(&modi, proc_macro2::Span::call_site());
            let params_tokens: Vec<proc_macro2::TokenStream> = params
                .iter()
                .map(|p| {
                    let param_token: proc_macro2::TokenStream = p.parse().unwrap();
                    param_token
                })
                .collect();
            quote! {
                let r: Result<(), String> = #modi_ident(#(#params_tokens),*);
                if let Err(e) = r {
                    ic_cdk::api::call::reject(&e);
                    return;
                }
            }
        })
        .collect();

    let expanded = quote! {
        #vis #asyncness fn #fn_name(#fn_params) {
            #(#modifier_checks)*
            #(#fn_stmts)*
        }
    };

    TokenStream::from(expanded)
}
