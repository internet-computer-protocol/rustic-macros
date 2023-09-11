#![doc = include_str!("../README.md")]

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
//use syn::punctuated::Punctuated;
use syn::{parse_macro_input, AttributeArgs, ItemFn, NestedMeta};

/// Macro used to add guards to functions. If the guard function returns `Ok` then execution would continue.
/// If the guard function returns `Err` the execution would stop and state changes would be reverted by invoking `ic_cdk::api::call::reject`.
/// 
/// If multiple guard functions exist they're executed in the order declared.
/// 
/// Usually the `modifiers` macro should be the last macro declared for a function, below everything else such as `update` and `query`.
/// Those are the only macros tested to work together with this `modifiers` macro. 
/// If you would like to use it with other macros or use a different order, remember Rust macro ordering rules apply, and always check the expanded result. 
/// 
/// The signatures of guard functions must be of
/// ```
/// fn func_name(params...) -> Result<(), String>
/// ```
/// Then the guard functions can be added as modifiers using the follow syntax:
/// ```
/// #[modifiers("func_name@params", ...)]
/// ```
/// The `params` is a commma-separated list(without spaces in between) of the arguments.
/// # Examples
/// ```
/// #[modifiers("guard_func")]
/// #[modifiers("guard_func@42,SomeEnum::A")]
/// #[modifiers("guard_func0", "guard_func1")]
/// ```
#[proc_macro_attribute]
pub fn modifiers(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let func = parse_macro_input!(input as ItemFn);

    // Capture asyncness and visibility
    let asyncness = &func.sig.asyncness;
    let vis = &func.vis;
    let ret_type = &func.sig.output;
    let generics = &func.sig.generics;
    let where_clause = &func.sig.generics.where_clause;
    let attrs = &func.attrs;
    let _unsafety = &func.sig.unsafety;
    let _abi = &func.sig.abi;

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
            if modi == "non_reentrant" {
                quote! {
                    let __guard = rustic::reentrancy_guard::ReentrancyGuard::new();
                }
            } else {
                quote! {
                    let r: Result<(), String> = #modi_ident(#(#params_tokens),*);
                    if let Err(e) = r {
                        ic_cdk::api::call::reject(&e);
                        panic!("{} failed: {}", stringify!(#modi_ident), e);
                    }
                }
            }
        })
        .collect();

    let expanded = quote! {
        #(#attrs)*
        #vis #asyncness fn #fn_name #generics (#fn_params) #ret_type #where_clause {
            #(#modifier_checks)*
            #(#fn_stmts)*
        }
    };

    TokenStream::from(expanded)
}
