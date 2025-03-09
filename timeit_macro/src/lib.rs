use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn timeit(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let vis = &input.vis; // copy the visibilty from the original function
    let fn_name = &input.sig.ident;
    let fn_body = &input.block;
    let fn_sig = &input.sig;

    let output = quote! {
        #vis #fn_sig {
            #[cfg(debug_assertions)] // Only active in debug mode
            {
                use std::time::Instant;
                let start = Instant::now();
                let result = (|| #fn_body)();
                let duration = start.elapsed();
                println!("Execution time of {}: {:?}", stringify!(#fn_name), duration);
                return result;
            }

            #[cfg(not(debug_assertions))] // In release mode without the timeing
            #fn_body
        }
    };

    output.into()
}
