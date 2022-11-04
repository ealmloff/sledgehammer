//!<div align="center">
//!  <!-- Crates version -->
//!  <a href="https://crates.io/crates/sledgehammer">
//!    <img src="https://img.shields.io/crates/v/sledgehammer.svg?style=flat-square"
//!    alt="Crates.io version" />
//!  </a>
//!  <!-- Downloads -->
//!  <a href="https://crates.io/crates/sledgehammer">
//!    <img src="https://img.shields.io/crates/d/sledgehammer.svg?style=flat-square"
//!      alt="Download" />
//!  </a>
//!  <!-- docs -->
//!  <a href="https://docs.rs/sledgehammer">
//!    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
//!      alt="docs.rs docs" />
//!  </a>
//!</div>
//!
//!**Breaking the WASM<->JS peformance boundry one brick at a time**
//!### Status: There are some holes in the wall.
//!
//!# What is Sledgehammer?
//!Sledgehammer provides faster rust bindings for dom manipuations by batching calls to js.
//!
//! # Getting started
//! - All operations go through a [`MsgChannel`] which handles the communication with js.
//!
//!# Benchmarks
//!
//!- run this benchmark in your browser: [dom operation time only (not paint time) js-framework-benchmark](https://demonthos.github.io/wasm_bindgen_sledgehammer/)
//!
//!This gives more consistant results than the official js-framework-benchmark because it excludes the variation in paint time. Because sledgehammer and wasm-bindgen implementations result in the same dom calls they should have the same paint time.
//!
//!- A few runs of [a fork of the js-framework-benchmark:](https://github.com/demonthos/js-framework-benchmark/tree/testing)
//!<div align="center">
//!  <img src="https://user-images.githubusercontent.com/66571940/199780394-a360581f-1496-4894-b7fe-3d5b5d627dbb.png" />
//!  <img src="https://user-images.githubusercontent.com/66571940/199780395-d7d00059-052e-40b7-9514-aba55800dc04.png" />
//!  <img src="https://user-images.githubusercontent.com/66571940/199780398-0060a62b-4d93-4a40-94a2-980835393aa2.png" />
//!</div>
//!
//!# How does this compare to wasm-bindgen/web-sys:
//!wasm-bindgen is a lot more general, and ergonomic to use than sledgehammer. It has bindings to a lot of apis that sledgehammer does not. For most users wasm-bindgen is a beter choice. Sledgehammer is specifically designed for web frameworks that want low level, fast access to the dom.
//!
//!# Why is it fast?
//!
//!## String decoding
//!
//!- Decoding strings are expensive to decode, but the cost doesn't change much with the size of the string. Wasm-bindgen calls TextDecoder.decode for every string. Sledehammer only calls TextEncoder.decode once per batch.
//!
//!- If the string is small it is faster to decode the string in javascript to avoid the constant overhead of TextDecoder.decode
//!
//!- See this benchmark: <https://jsbench.me/4vl97c05lb/5>
//!
//!## Single byte attributes and elements
//!
//!- In addition to making string decoding cheaper, sledehammer also uses less strings. All elements and attribute names are encoded as a single byte instead of a string and then turned back into a string in the javascipt intepreter.
//!
//!- To allow for custom elements and attributes, you can pass in a &str instead of a Attribute or Element enum.
//!
//!## Byte encoded operations
//!
//!- In sledehammer every operation is encoded as a sequence of bytes packed into an array. Every operation takes 1 byte plus whatever data is required for it.
//!
//!- Booleans are encoded as part of the operation byte to reduce the number of bytes read.
//!
//!- Each operation is encoded in a batch of four as a u32. Getting a number from an array buffer has a high constant cost, but getting a u32 instead of a u8 is not more expensive. Sledgehammer reads the u32 and then splits it into the 4 individual bytes.
//!
//!- See this benchmark: <https://jsbench.me/csl9lfauwi/2>
//!
//!## Minimize passing ids
//!
//!- A common set of operations for webframeworks to perform is traversing dom nodes after cloning them. Instead of assigning an id to every node, sledgehammer allows you to perform operations on the last node that was created or navigated to. This means traversing id takes only one byte per operation instead of 5.

#![allow(non_camel_case_types)]

// mod attrs;
pub mod attribute;
pub mod batch;
pub mod builder;
pub mod element;

pub use attribute::{Attribute, IntoAttribue};
pub use builder::{MsgChannel, NodeId, WritableText};
pub use element::{Element, ElementBuilder, IntoElement};

use wasm_bindgen::prelude::*;
use web_sys::Node;

#[used]
static mut MSG_PTR: usize = 0;
#[used]
static mut MSG_PTR_PTR: *const usize = unsafe { &MSG_PTR } as *const usize;
#[used]
static mut MSG_POS_UPDATED: u8 = 255;
#[used]
static mut MSG_METADATA_PTR: *const u8 = unsafe { &MSG_POS_UPDATED } as *const u8;
#[used]
static mut STR_PTR: usize = 0;
#[used]
static mut STR_PTR_PTR: *const usize = unsafe { &STR_PTR } as *const usize;
#[used]
static mut STR_LEN: usize = 0;
#[used]
static mut STR_LEN_PTR: *const usize = unsafe { &STR_LEN } as *const usize;

#[wasm_bindgen(module = "/interpreter_opt.js")]
// #[wasm_bindgen(module = "/interpreter.js")]
extern "C" {
    fn work_last_created();

    fn update_last_memory(mem: JsValue);

    fn last_needs_memory() -> bool;

    pub(crate) type JsInterpreter;

    #[wasm_bindgen(constructor)]
    pub(crate) fn new(
        mem: JsValue,
        msg_pos_updated_ptr: usize,
        msg_ptr: usize,
        str_ptr: usize,
        str_len_ptr: usize,
    ) -> JsInterpreter;

    #[wasm_bindgen(method)]
    pub(crate) fn UpdateMemory(this: &JsInterpreter, mem: JsValue);

    #[allow(unused)]
    #[wasm_bindgen(method)]
    pub(crate) fn NeedsMemory(this: &JsInterpreter) -> bool;

    #[wasm_bindgen(method)]
    pub(crate) fn SetNode(this: &JsInterpreter, id: u32, node: Node);

    #[allow(unused)]
    #[wasm_bindgen(method)]
    pub(crate) fn GetNode(this: &JsInterpreter, id: u32) -> Node;
}

pub struct InNamespace<'a, T>(pub T, pub &'a str);

pub trait WithNsExt {
    fn in_namespace(self, namespace: &str) -> InNamespace<Self>
    where
        Self: Sized,
    {
        InNamespace(self, namespace)
    }
}

impl WithNsExt for Element {}
impl WithNsExt for Attribute {}
impl<'a> WithNsExt for &'a str {}
