#![feature(proc_macro_quote)]
#![feature(extend_one)]

pub(crate) mod gen;
pub(crate) mod parse;

use gen::generate_output;
use parse::SchemaDefinition;
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
pub fn define_schema(input: TokenStream) -> TokenStream {
    let SchemaDefinition { info, root } = parse_macro_input!(input as SchemaDefinition);

    let mut nodes_tokens = proc_macro2::TokenStream::new();
    for node in &root.nodes {
        let inner = generate_output(&info, node);
        nodes_tokens.extend(inner);
    }

    nodes_tokens.into()
}
