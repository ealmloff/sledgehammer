use std::{any::Any, convert::TryFrom, str::FromStr, thread::Builder};

use bumpalo::Bump;
use proc_macro::TokenStream;
use quote::quote;
use sledgehammer_encoder::{
    attribute::AnyAttribute,
    batch::{Batch, FinalizedBatch},
    element::AnyElement,
    Attribute, Element, ElementBuilder, NodeBuilder, TextBuilder,
};
use syn::{Expr, Lit};
use syn_rsx::{parse, Node, NodeType};

enum NodeInProgress {
    Element(ElementInProgress),
    Text(String),
}

struct ElementInProgress {
    kind: String,
    attributes: Vec<(String, String)>,
    children: Vec<NodeInProgress>,
}

fn walk_nodes<'a>(nodes: &'a Vec<Node>, inside: &mut Option<ElementInProgress>) {
    for node in nodes {
        match node {
            Node::Doctype(doctype) => {}
            Node::Element(element) => {
                let name = element.name.to_string();

                let mut builder = Some(ElementInProgress {
                    kind: name,
                    attributes: Vec::new(),
                    children: Vec::new(),
                });

                // attributes
                walk_nodes(&element.attributes, &mut builder);

                // children
                walk_nodes(&element.children, &mut builder);

                match inside {
                    Some(el) => el.children.push(NodeInProgress::Element(builder.unwrap())),
                    None => *inside = builder,
                }
            }
            Node::Attribute(attribute) => {
                let key_str = attribute.key.to_string();
                if let Some(el) = inside {
                    if let Some(val) = &attribute.value {
                        el.attributes.push((key_str, as_str_lit(&val)))
                    }
                }
            }
            Node::Text(txt) => {
                if let Some(el) = inside {
                    el.children
                        .push(NodeInProgress::Text(as_str_lit(&txt.value)))
                }
            }
            Node::Fragment(_) => {
                panic!("fragments are not supported")
            }
            Node::Comment(_) => {}
            Node::Block(_) => {
                panic!("blocks are not supported")
            }
        }
    }
}

fn as_str_lit(expr: &Expr) -> String {
    if let Expr::Lit(u) = expr {
        if let Lit::Str(s) = &u.lit {
            return s.value();
        }
    }
    panic!("expected string")
}

/// Converts HTML to `String`.
///
/// Values returned from braced blocks `{}` are expected to return something
/// that implements `Display`.
///
/// See [syn-rsx docs](https://docs.rs/syn-rsx/) for supported tags and syntax.
///
/// # Example
///
/// ```
/// use html_to_string_macro::html;
///
/// let world = "planet";
/// assert_eq!(html!(<div>"hello "{world}</div>), "<div>hello planet</div>");
/// ```
#[proc_macro]
pub fn html(tokens: TokenStream) -> TokenStream {
    match parse(tokens) {
        Ok(nodes) => {
            let mut builder = None;
            walk_nodes(&nodes, &mut builder);
            match builder {
                Some(builder) => {
                    let bump = Bump::new();
                    let builder = NodeInProgress::Element(builder);
                    let builder = build_in_progress(&bump, &builder);
                    let mut batch = Batch::default();
                    match builder {
                        NodeBuilder::Text(txt) => batch.build_text_node(txt),
                        NodeBuilder::Element(el) => {
                            batch.build_full_element(el);
                        }
                    }
                    let finalized = batch.finalize();
                    let msg = &finalized.msg;
                    let str = &finalized.str;
                    quote! {
                        StaticBatch{
                            msg: &[$($msg),*],
                            str: &[$(str),*]
                        }
                    }
                }
                None => {
                    panic!("empty html call");
                }
            }
        }
        Err(error) => error.to_compile_error(),
    }
    .into()
}

fn build_in_progress<'a>(allocator: &'a Bump, node: &'a NodeInProgress) -> NodeBuilder<'a> {
    match node {
        NodeInProgress::Element(el) => {
            let mut builder = ElementBuilder::new(match Element::from_str(&el.kind) {
                Ok(el) => AnyElement::Element(el),
                Err(_) => AnyElement::Str(&el.kind),
            });
            let children: Vec<_> = el
                .children
                .iter()
                .map(|node| build_in_progress(allocator, node))
                .collect();
            builder = builder.children(allocator.alloc(children));
            let attributes: Vec<(AnyAttribute<'_, '_>, &str)> = el
                .attributes
                .iter()
                .map(|(attr, value)| {
                    (
                        match Attribute::from_str(attr) {
                            Ok(a) => AnyAttribute::Attribute(a),
                            Err(_) => AnyAttribute::Str(attr),
                        },
                        &*allocator.alloc_str(&value),
                    )
                })
                .collect();
            builder = builder.attrs(allocator.alloc(attributes));
            NodeBuilder::Element(builder)
        }
        NodeInProgress::Text(txt) => NodeBuilder::Text(TextBuilder::new(txt)),
    }
}
