#![doc = include_str!("../README.md")]
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

mod with_parent_span;

#[proc_macro_error]
#[proc_macro_attribute]
pub fn with_parent_span(args: TokenStream, input: TokenStream) -> TokenStream {
    with_parent_span::macros(args.into(), input.into()).into()
}
