use proc_macro::TokenStream;
use quote::quote;
use syn::fold::Fold;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, parse_quote, ItemFn, Pat};
struct Determinate;

impl Parse for Determinate {
    fn parse(_input: ParseStream) -> Result<Self> {
        Ok(Determinate {})
    }
}

impl Fold for Determinate {
    fn fold_item_fn(&mut self, input: ItemFn) -> ItemFn {
        let ItemFn {
            attrs,
            vis,
            sig,
            block,
        } = input;
        let inputs = &sig.inputs;
        let output = &sig.output;
        let stmts = block.stmts.to_owned();
        let output_type = match output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_, output_type) => Some(output_type),
        };

        let future_type = match output_type {
            Some(output_type) => quote! {dyn Future<Output = #output_type>},
            None => quote! { dyn Future<Output = ()> },
        };

        let mut input_idents = Punctuated::<Box<Pat>, Comma>::new();
        for input in inputs.iter() {
            match input {
                syn::FnArg::Typed(pat_type) => input_idents.push(pat_type.pat.clone()),
                syn::FnArg::Receiver(_) => continue,
            }
        }

        let mut block = block;
        block.stmts = parse_quote! {
                use std::pin::Pin;
                use futures::{executor::block_on, future::join_all, Future};
                let f = |#inputs| #output { #(#stmts)* };
                let wrapper = |f: fn(#inputs) #output| -> Pin<Box<#future_type>> {
            Box::pin(async move { f(#input_idents) })
        };
                let mut futures = vec![];
                for _ in 0..num_cpus::get() {
                    futures.push(wrapper(f));
                }
                let future = async { join_all(futures).await };
                let values = block_on(future);
                assert!(values.iter().all(|&item| item == values[0]));
                values[0]
            };

        ItemFn {
            attrs,
            vis,
            sig,
            block,
        }
    }
}

#[proc_macro_attribute]
pub fn determinate(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as ItemFn);
    let mut determinate = parse_macro_input!(attr as Determinate);
    let output = determinate.fold_item_fn(input);
    TokenStream::from(quote!(#output))
}

#[proc_macro_attribute]
pub fn indeterminate(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as ItemFn);

    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = input;
    let stmts = &block.stmts;

    let output = &sig.output;
    let output_type = match output {
        syn::ReturnType::Default => None,
        syn::ReturnType::Type(_, output_type) => Some(output_type),
    };

    TokenStream::from(quote! {
        #(#attrs)* #vis #sig {
            use once_cell::sync::Lazy;
            static RESULT: Lazy<#output_type> = Lazy::new(|| { #(#stmts)* });
            *RESULT
        }
    })
}
