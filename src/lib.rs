use proc_macro::TokenStream;
use quote::quote;
use syn::fold::Fold;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, parse_quote, ItemFn, Pat};
struct Stack;

impl Parse for Stack {
    fn parse(_input: ParseStream) -> Result<Self> {
        Ok(Stack {})
    }
}

impl Fold for Stack {
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
                let futures = vec![wrapper(f), wrapper(f)];
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
    let mut stack = parse_macro_input!(attr as Stack);
    let output = stack.fold_item_fn(input);
    TokenStream::from(quote!(#output))
}
