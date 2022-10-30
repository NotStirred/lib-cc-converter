#![feature(proc_macro_quote)]
#![feature(extend_one)]

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{braced, parenthesized, parse_macro_input, ExprLit, Lit, Token};

#[derive(Debug)]
struct SchemaTree {
    pub nodes: Vec<SchemaNode>,
}

impl Parse for SchemaTree {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        braced!(content in input);

        let i: SchemaNode = content.parse()?;

        Ok(Self { nodes: vec![i] })
    }
}

#[derive(Debug)]
enum SchemaNode {
    Register(syn::Ident, Vec<SchemaNode>),
    Opt(String, Box<SchemaNode>),
    Req(String, Box<SchemaNode>),
    Reference(syn::Ident),
    List(syn::Ident),
}

impl Parse for SchemaNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: syn::Ident = input.parse()?;
        let node_ident = ident.to_string();
        let node_ident: &str = node_ident.as_str();
        match node_ident {
            "register" => {
                let registry: syn::Ident = input.parse()?;
                let content;
                braced!(content in input);

                let mut nodes = Vec::new();
                loop {
                    // parse a schema node
                    let node: SchemaNode = content.parse()?;
                    nodes.push(node);

                    // if there is a comma, parse another
                    let lookahead = content.lookahead1();
                    if !lookahead.peek(Token![,]) {
                        break;
                    }
                    content.parse::<Token![,]>()?; // consume the comma
                }

                Ok(SchemaNode::Register(registry, nodes))
            }
            "req" => {
                let content;
                parenthesized!(content in input);
                let val: ExprLit = content.parse()?;
                content.parse::<Token![,]>()?;
                if let Lit::Str(s) = val.lit {
                    Ok(SchemaNode::Req(s.value(), Box::new(content.parse()?)))
                } else {
                    Err(syn::Error::new(val.span(), "expected string literal"))
                }
            }
            "opt" => {
                let content;
                parenthesized!(content in input);
                let val: ExprLit = content.parse()?;
                content.parse::<Token![,]>()?;
                if let Lit::Str(s) = val.lit {
                    Ok(SchemaNode::Opt(s.value(), Box::new(content.parse()?)))
                } else {
                    Err(syn::Error::new(val.span(), "expected string literal"))
                }
            }
            "reference" => {
                let content;
                parenthesized!(content in input);
                let ident: syn::Ident = content.parse()?;
                Ok(SchemaNode::Reference(ident))
            }
            "list" => {
                let content;
                parenthesized!(content in input);
                let ident: syn::Ident = content.parse()?;
                Ok(SchemaNode::List(ident))
            }
            _ => Err(syn::Error::new(ident.span(), "unknown identifier")),
        }
    }
}

struct SchemaDefinition {
    info: syn::Ident,
    def: SchemaTree,
}

impl Parse for SchemaDefinition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let info: syn::Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let tree: SchemaTree = input.parse()?;

        Ok(SchemaDefinition { info, def: tree })
    }
}

#[proc_macro]
pub fn define_schema(input: TokenStream) -> TokenStream {
    let SchemaDefinition { info, def } = parse_macro_input!(input as SchemaDefinition);

    generate_output(info, &def.nodes[0]).into()
}

fn generate_output(info: syn::Ident, node: &SchemaNode) -> proc_macro2::TokenStream {
    match node {
        SchemaNode::Register(ident, nodes) => {
            let mut nodes_tokens = proc_macro2::TokenStream::new();
            for node in nodes {
                let inner = generate_output(info.clone(), node);
                nodes_tokens.extend(inner);
            }

            quote! {
                #info.references.#ident.insert(#info.from_version,
                    std::boxed::Box::leak(std::boxed::Box::new(|data: &mut quartz_nbt::Map<quartz_nbt::NbtTag>, from, to|{
                        #nodes_tokens
                        Ok(())
                    })));
            }
        }
        SchemaNode::Opt(val, node) => {
            let inner = generate_output(info, node);
            quote! {
                if let std::option::Option::Some(value) = data.get_mut(#val) {
                    #inner
                }
            }
        }
        SchemaNode::Req(val, node) => {
            let inner = generate_output(info, node);
            quote! {
                let value = data.get_mut(#val).unwrap();
                #inner
            }
        }
        SchemaNode::Reference(ident) => {
            let tokens = quote! {
                dfu_structures::types::convert(&crate::TYPES.#ident, value, from, to);
            };
            tokens
        }
        SchemaNode::List(ident) => {
            let tokens = quote! {
                let list: &mut quartz_nbt::NbtList = value.try_into()?;
                dfu_structures::types::convert_all(&crate::TYPES.#ident, list, from, to);
            };
            tokens
        }
    }
}
