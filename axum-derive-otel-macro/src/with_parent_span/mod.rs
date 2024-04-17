use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, ItemFn};

#[cfg(test)]
mod tests;

mod with_parent_span;

pub fn macros(_args: TokenStream, input: TokenStream) -> TokenStream {
    let source_fn = match parse2::<ItemFn>(input) {
        Ok(syntax_tree) => syntax_tree,
        Err(error) => return error.to_compile_error(),
    };

    let new_item_fn = with_parent_span::transform(source_fn);
    quote!(#new_item_fn)
}
