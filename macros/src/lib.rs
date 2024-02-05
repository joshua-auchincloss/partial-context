use partial_context::context_impl;
use proc_macro::TokenStream;
mod partial_context;

#[proc_macro_derive(PartialContext, attributes(context, context_needs, partial))]
pub fn with_context(args: TokenStream) -> TokenStream {
    context_impl(args)
}
