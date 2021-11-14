use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

#[proc_macro_attribute]
pub fn determinate(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as ItemFn);
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = input;
    let inputs = &sig.inputs;
    let output = &sig.output;
    let stmts = &block.stmts;

    TokenStream::from(quote! {
        #(#attrs)* #vis #sig {
            use futures::{executor::block_on, future::join_all};
            let f = |#inputs| #output { #(#stmts)* };
            let futures = vec![wrapper(f), wrapper(f)];
            let future = async { join_all(futures).await };
            let values = block_on(future);
            assert!(values.iter().all(|&item| item == values[0]));
            values[0]
        }
    })
}
