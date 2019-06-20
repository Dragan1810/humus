#![recursion_limit = "128"]

use proc_macro_hack::proc_macro_hack;


extern crate proc_macro;

use proc_macro2::{Literal, TokenStream, TokenTree};
use quote::quote;
use snax::{SnaxAttribute, SnaxItem, SnaxTag};

#[proc_macro_hack]
pub fn html(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);

    let parsed_content = snax::parse(input).expect("Could not even");
    let output = create_node(&parsed_content);
    // eprint!("{:#?}", parsed_content);
    proc_macro::TokenStream::from(output)
}

fn create_node(item: &SnaxItem) -> TokenStream {
    match item {
        SnaxItem::Tag(tag) => create_tag(tag),
        SnaxItem::SelfClosingTag(_tag) => TokenStream::new(),
        SnaxItem::Content(tt) => create_content(tt),
        SnaxItem::Fragment(_fragment) => TokenStream::new(),
    }
}

fn create_tag(tag: &SnaxTag) -> TokenStream {
    let attribute_insertions = emit_attributes(&tag.attributes);
    let child_insertions = emit_children(&tag.children);

    let tag_name_literal = Literal::string(&tag.name.to_string());

    quote!({
        ::humus::render::h(#tag_name_literal, #child_insertions, #attribute_insertions)
    })
}


fn emit_attributes(attributes: &[SnaxAttribute]) -> TokenStream {
    attributes
        .iter()
        .map(|attribute| match attribute {
            SnaxAttribute::Simple { name, value } => quote!(
                ::humus::render::attr(#name, #value)
            ),
        })
        .collect()
}


fn emit_children(children: &[SnaxItem]) -> TokenStream {
    children
        .iter()
        .map(|child| {
            let emitted = create_node(child);

            quote!(
                #emitted
            )
        })
        .collect()
}


fn create_content(tt: &TokenTree) -> TokenStream {
    quote!(
        ::humus::render::t(#tt);
    )
}