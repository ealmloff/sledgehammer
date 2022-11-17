//! This module contains the [`MsgChannel`] type which is used to send batched dom manipulation operations.
//! The main way to add operations to a [`MsgChannel`] is by dirrectly calling the methods on it, and then calling [`MsgChannel::flush`] to send the operations to the browser.
//!
//!

use std::{fmt::Arguments, io::Write};

use web_sys::Node;

use crate::{
    batch::{Batch, Op, PreparedBatch},
    update_last_memory, work_last_created, ElementBuilder, IntoAttribue, IntoElement,
    JsInterpreter, MSG_METADATA_PTR, MSG_PTR_PTR, STR_LEN_PTR, STR_PTR_PTR,
};

/// Tracks if a interpreter has been created. Used to prevent multiple interpreters from being created.
static mut INTERPRETER_EXISTS: bool = false;

/// An id that may be either the last node or a node with an assigned id.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MaybeId {
    /// The last node that was created or navigated to.
    LastNode,
    /// A node that was created and stored with an id
    Node(NodeId),
}

impl MaybeId {
    #[inline(always)]
    pub(crate) const fn encoded_size(&self) -> u8 {
        match self {
            MaybeId::LastNode => 0,
            MaybeId::Node(_) => 4,
        }
    }
}

/// A node that was created and stored with an id
/// It is recommended to create and store ids with a slab allocator with an exposed slab index for example the excellent [slab](https://docs.rs/slab) crate.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(pub u32);

/// The [`MsgChannel`] handles communication with the dom. It allows you to send batched operations to the dom.
/// All of the functions that are not marked otherwise are qued and not exicuted imidately. When you want to exicute the que you have to call [`MsgChannel::flush`].
/// There should only be one [`MsgChannel`] per application.
pub struct MsgChannel {
    pub(crate) js_interpreter: JsInterpreter,
    last_mem_size: usize,
    batch: Batch,
}

impl Default for MsgChannel {
    fn default() -> Self {
        unsafe {
            debug_assert!(
                !INTERPRETER_EXISTS,
                "Found another MsgChannel. Only one MsgChannel can be created"
            );
            INTERPRETER_EXISTS = true;
        }
        debug_assert!(0x1F > Op::NoOp as u8);
        // format!(
        //     "init: {:?}, {:?}, {:?}",
        //     unsafe { MSG_PTR_PTR as usize },
        //     unsafe { STR_PTR_PTR as usize },
        //     unsafe { STR_LEN_PTR as usize }
        // );
        let js_interpreter = unsafe {
            JsInterpreter::new(
                wasm_bindgen::memory(),
                MSG_METADATA_PTR as usize,
                MSG_PTR_PTR as usize,
                STR_PTR_PTR as usize,
                STR_LEN_PTR as usize,
            )
        };

        Self {
            js_interpreter,
            last_mem_size: 0,
            batch: Batch::default(),
        }
    }
}

impl MsgChannel {
    /// IMPORTANT: This method is exicuted immediatly and does not wait for the next flush
    ///
    /// Example:
    /// ```no_run
    /// let window = web_sys::window().unwrap();
    /// let document = window.document().unwrap();
    /// let body = document.body().unwrap();
    /// let mut channel = MsgChannel::default();
    /// // assign the NodeId(0) to the body element from web-sys
    /// channel.set_node(NodeId(0), JsCast::dyn_into(body).unwrap());
    /// // no need to call flush here because set_node is exicuted immediatly
    /// ```
    pub fn set_node(&mut self, id: NodeId, node: Node) {
        self.js_interpreter.SetNode(id.0, node);
    }

    /// IMPORTANT: This method is exicuted immediatly and does not wait for the next flush
    ///
    /// Example:
    /// ```no_run
    /// let mut channel = MsgChannel::default();
    /// channel.create_element("div", Some(NodeId(0)));
    /// channel.flush();
    /// let element = channel.get_node(NodeId(0));
    /// let text = element.text_content().map(|t| t + " + web-sys");
    /// element.set_text_content(text.as_deref());
    /// // no need to call flush here because get_node is exicuted immediatly
    /// ```
    pub fn get_node(&mut self, id: NodeId) -> Node {
        self.js_interpreter.GetNode(id.0)
    }

    /// Exicutes any queued operations in the order they were added
    ///
    /// Example:
    ///
    /// ```no_run
    /// let mut channel = MsgChannel::default();
    /// // this does not immediatly create a <div> or <p>
    /// channel.create_element("div", None);
    /// channel.create_element("p", None);
    /// // this creates the <div> and <p> elements
    /// channel.flush();
    /// ```
    pub fn flush(&mut self) {
        self.batch.encode_op(Op::Stop);
        run_batch(
            &self.batch.msg,
            &self.batch.str_buf,
            &mut self.last_mem_size,
        );
        self.batch.msg.clear();
        self.batch.current_op_batch_idx = 0;
        self.batch.current_op_byte_idx = 3;
        self.batch.str_buf.clear();
    }

    /// Appends a number of nodes as children of the given node.
    ///
    /// Example:
    ///
    /// ```no_run
    /// let mut channel = MsgChannel::default();
    /// channel.create_element("div", Some(NodeId(0)));
    /// channel.create_element("p", None);
    /// // append the <p> element to the <div> element
    /// channel.append_child(MaybeId::Node(NodeId(0)), MaybeId::LastNode);
    /// channel.flush();
    /// ```
    pub fn append_child(&mut self, root: MaybeId, child: MaybeId) {
        self.batch.append_child(root, child)
    }

    /// Replace a node with another node
    ///
    /// Example:
    /// ```no_run
    /// let mut channel = MsgChannel::default();
    /// channel.create_element("div", Some(NodeId(0)));
    /// channel.create_element("p", None);
    /// // replace the <p> element with the <div> element
    /// channel.replace_with(MaybeId::Node(NodeId(0)), MaybeId::LastNode);
    /// channel.flush();
    /// ```
    pub fn replace_with(&mut self, root: MaybeId, node: MaybeId) {
        self.batch.replace_with(root, node)
    }

    /// Replace a node with many nodes
    ///
    /// Example:
    /// ```no_run
    /// let mut channel = MsgChannel::default();
    /// channel.create_element("div", Some(NodeId(0)));
    /// channel.create_element("p", None);
    /// // replace the <p> element with the <div> element
    /// channel.replace_with_nodes(MaybeId::Node(NodeId(0)), MaybeId::LastNode);
    /// channel.flush();
    /// ```
    pub fn replace_with_nodes(&mut self, root: MaybeId, nodes: &[MaybeId]) {
        self.batch.replace_with_nodes(root, nodes)
    }

    /// Insert a single node after a given node.
    ///
    /// Example:
    /// ```no_run
    /// let mut channel = MsgChannel::default();
    /// channel.create_element("div", Some(NodeId(0)));
    /// channel.create_element("p", None);
    /// // insert the <p> element after the <div> element
    /// channel.insert_after(MaybeId::Node(NodeId(0)), MaybeId::LastNode);
    /// channel.flush();
    /// ```
    pub fn insert_after(&mut self, root: MaybeId, node: MaybeId) {
        self.batch.insert_after(root, node)
    }

    /// Insert a many nodes after a given node.
    ///
    /// Example:
    /// ```no_run
    /// let mut channel = MsgChannel::default();
    /// channel.create_element("div", Some(NodeId(0)));
    /// channel.create_element("p", None);
    /// // insert the <p> element after the <div> element
    /// channel.insert_nodes_after(MaybeId::Node(NodeId(0)), &[MaybeId::LastNode]);
    /// channel.flush();
    /// ```
    pub fn insert_nodes_after(&mut self, root: MaybeId, nodes: &[MaybeId]) {
        self.batch.insert_nodes_after(root, nodes)
    }

    /// Insert a single node before a given node.
    ///
    /// Example:
    /// ```no_run
    /// let mut channel = MsgChannel::default();
    /// channel.create_element("div", Some(NodeId(0)));
    /// channel.create_element("p", None);
    /// // insert the <p> element before the <div> element
    /// channel.insert_before(MaybeId::Node(NodeId(0)), MaybeId::LastNode);
    /// channel.flush();
    /// ```
    pub fn insert_before(&mut self, root: MaybeId, node: MaybeId) {
        self.batch.insert_before(root, node)
    }

    /// Insert many nodes before a given node.
    ///
    /// Example:
    /// ```no_run
    /// let mut channel = MsgChannel::default();
    /// channel.create_element("div", Some(NodeId(0)));
    /// channel.create_element("p", None);
    /// // insert the <p> element before the <div> element
    /// channel.insert_nodes_before(MaybeId::Node(NodeId(0)), &[MaybeId::LastNode]);
    /// channel.flush();
    /// ```
    pub fn insert_nodes_before(&mut self, root: MaybeId, nodes: &[MaybeId]) {
        self.batch.insert_nodes_before(root, nodes)
    }

    /// Remove a node from the DOM.
    ///
    /// Example:
    /// ```no_run
    /// let mut channel = MsgChannel::default();
    /// channel.create_element("p", None);
    /// // remove the <p> element
    /// channel.remove(MaybeId::LastNode);
    /// channel.flush();
    /// ```
    pub fn remove(&mut self, id: MaybeId) {
        self.batch.remove(id)
    }

    /// Create a new text node
    ///
    /// Example:
    /// ```no_run
    /// let mut channel = MsgChannel::default();
    /// // create a text node with the text "Hello World"
    /// channel.create_text_node("Hello World", None);
    /// channel.flush();
    pub fn create_text_node(&mut self, text: impl WritableText, id: Option<NodeId>) {
        self.batch.create_text_node(text, id)
    }

    /// Create a new element node
    ///
    /// Example:
    /// ```no_run
    /// let mut channel = MsgChannel::default();
    /// // create a <div> element
    /// channel.create_element("div", None);
    /// channel.flush();
    /// ```
    pub fn create_element<'a, 'b>(&mut self, tag: impl IntoElement<'a, 'b>, id: Option<NodeId>) {
        self.batch.create_element(tag, id)
    }

    /// Set the textcontent of a node.
    ///
    /// Example:
    /// ```no_run
    /// let mut channel = MsgChannel::default();
    /// // create a text node with the text "Hello World"
    /// channel.create_text_node("Hello ", None);
    /// // set the text content of the text node to "Hello World!!!"
    /// channel.set_text_content("World!!!", MaybeId::LastNode);
    /// channel.flush();
    /// ```
    pub fn set_text(&mut self, text: impl WritableText, root: MaybeId) {
        self.batch.set_text(text, root)
    }

    /// Set the value of a node's attribute.
    ///
    /// Example:
    /// ```no_run
    /// let mut channel = MsgChannel::default();
    /// // create a <div> element
    /// channel.create_element("div", None);
    /// // set the attribute "id" to "my-div" on the <div> element
    /// channel.set_attribute(Attribute::id, "my-div", MaybeId::LastNode);
    /// channel.flush();
    /// ```
    pub fn set_attribute<'a, 'b>(
        &mut self,
        attr: impl IntoAttribue<'a, 'b>,
        value: impl WritableText,
        root: MaybeId,
    ) {
        self.batch.set_attribute(attr, value, root)
    }

    /// Remove an attribute from a node.
    ///
    /// Example:
    /// ```no_run
    /// let mut channel = MsgChannel::default();
    /// // create a <div> element
    /// channel.create_element("div", None);
    /// channel.set_attribute(Attribute::id, "my-div", MaybeId::LastNode);
    /// // remove the attribute "id" from the <div> element
    /// channel.remove_attribute(Attribute::id, MaybeId::LastNode);
    /// channel.flush();
    /// ```
    pub fn remove_attribute<'a, 'b>(&mut self, attr: impl IntoAttribue<'a, 'b>, root: MaybeId) {
        self.batch.remove_attribute(attr, root)
    }

    /// Clone a node and store it with a new id.
    ///
    /// Example:
    /// ```no_run
    /// let mut channel = MsgChannel::default();
    /// // create a <div> element
    /// channel.create_element("div", None);
    /// // clone the <div> element and store it with the id 1
    /// channel.clone_node(MaybeId::LastNode, Some(NodeId(1)));
    /// channel.flush();
    /// ```
    pub fn clone_node(&mut self, id: MaybeId, new_id: MaybeId) {
        self.batch.clone_node(id, new_id)
    }

    /// Move the last node to the first child
    ///
    /// Example:
    /// ```no_run
    /// let mut channel = MsgChannel::default();
    /// // create a element: <div><p></p></div>
    /// channel.build_full_element(
    ///     ElementBuilder::new("div".into())
    ///         .children(&[
    ///             ElementBuilder::new(Element::p.into())
    ///                 .into(),
    ///         ]),
    /// );
    /// // move from the <div> to the <p>
    /// channel.first_child();
    /// // operatons modifing the <p> element...
    /// channel.flush();
    /// ```
    pub fn first_child(&mut self) {
        self.batch.first_child()
    }

    /// Move the last node to the next sibling
    ///
    /// Example:
    /// ```no_run
    /// let mut channel = MsgChannel::default();
    /// // create a element: <div><h1></h1><p></p></div>
    /// channel.build_full_element(
    ///     ElementBuilder::new("div".into())
    ///         .children(&[
    ///             ElementBuilder::new(Element::h1.into())
    ///                 .into(),
    ///             ElementBuilder::new(Element::p.into())
    ///         ]),
    /// );
    /// // move from the <div> to the <h1>
    /// channel.first_child();
    /// // move from the <h1> to the <p>
    /// channel.next_sibling();
    /// // operatons modifing the <p> element...
    /// channel.flush();
    /// ```
    pub fn next_sibling(&mut self) {
        self.batch.next_sibling()
    }

    /// Move the last node to the parent node
    ///
    /// Example:
    /// ```no_run
    /// let mut channel = MsgChannel::default();
    /// // create a element: <div><p></p></div>
    /// channel.build_full_element(
    ///     ElementBuilder::new("div".into())
    ///         .children(&[
    ///             ElementBuilder::new(Element::p.into())
    ///                 .id(NodeId(0))
    ///                 .into(),
    ///         ]),
    /// );
    /// // move to the <p> element
    /// channel.set_last_node(NodeId(0));
    /// // move from the <p> to the <div>
    /// channel.parent_node();
    /// // operatons modifing the <p> element...
    /// channel.flush();
    /// ```
    pub fn parent_node(&mut self) {
        self.batch.parent_node()
    }

    /// Store the last node with the given id. This is useful when traversing the document tree.
    ///
    /// Example:
    /// ```no_run
    /// let mut channel = MsgChannel::default();
    /// // create a element without an id
    /// channel.create_element("div", None);
    /// // store the <div> element with the id 0
    /// channel.set_last_node(NodeId(0));
    /// channel.flush();
    /// ```
    pub fn store_with_id(&mut self, id: NodeId) {
        self.batch.store_with_id(id)
    }

    /// Set the last node to the given id. The last node can be used to traverse the document tree without passing objects between wasm and js every time.
    ///
    /// Example:
    /// ```no_run
    /// let mut channel = MsgChannel::default();
    /// // create a element: <div><h1><h2></h2></h1><p></p></div>
    /// channel.build_full_element(
    ///     ElementBuilder::new("div".into())
    ///         .children(&[
    ///             ElementBuilder::new(Element::h1.into())
    ///                 .children(&[
    ///                     ElementBuilder::new(Element::h2.into())
    ///                         .into(),
    ///                 ]).into(),
    ///             ElementBuilder::new(Element::p.into())
    ///                 .into(),
    ///         ]),
    /// );
    /// // move from the <div> to the <h1>
    /// channel.first_child();
    /// // store the <h1> element with the id 0
    /// channel.store_with_id(NodeId(0));
    /// // move from the <h1> to the <h2>
    /// channel.first_child();
    /// // update something in the <h2> element...
    /// // restore the <h1> element
    /// channel.set_last_node(NodeId(0));
    /// // move from the <h1> to the <p>
    /// channel.next_sibling();
    /// // operatons modifing the <p> element...
    /// channel.flush();
    /// ```
    pub fn set_last_node(&mut self, id: NodeId) {
        self.batch.set_last_node(id)
    }

    /// Build a full element, slightly more efficent than creating the element creating the element with `create_element` and then setting the attributes.
    ///
    /// Example:
    /// ```rust
    /// let mut channel = MsgChannel::default();
    /// // create an element using sledgehammer
    /// channel.build_full_element(
    ///     ElementBuilder::new("div".into())
    ///         .id(NodeId(0))
    ///         .attrs(&[(Attribute::style.into(), "color: blue")])
    ///         .children(&[
    ///             ElementBuilder::new(Element::p.into())
    ///                 .into(),
    ///             TextBuilder::new("Hello from sledgehammer!").into(),
    ///         ]),
    /// );
    /// channel.flush();
    /// ```
    pub fn build_full_element(&mut self, el: ElementBuilder) {
        self.batch.build_full_element(el)
    }

    /// Set a style property on a node.
    ///
    /// Example:
    /// ```rust
    /// let mut channel = MsgChannel::default();
    /// channel.create_element("div", None);
    /// // set the style property "color" to "blue"
    /// channel.set_style("color", "blue", MaybeId::LastNode);
    /// channel.flush();
    /// ```
    pub fn set_style(&mut self, style: &str, value: &str, id: MaybeId) {
        self.batch.set_style(style, value, id)
    }

    /// Remove a style property from a node.
    ///
    /// Example:
    /// ```rust
    /// let mut channel = MsgChannel::default();
    /// channel.create_element("div", None);
    /// channel.set_style("color", "blue", MaybeId::LastNode);
    /// // remove the color style
    /// channel.remove_style("color", MaybeId::LastNode);
    /// channel.flush();
    /// ```
    pub fn remove_style(&mut self, style: &str, id: MaybeId) {
        self.batch.remove_style(style, id)
    }

    /// Adds a batch of operations to the current batch.
    ///
    /// Example:
    /// ```rust
    /// let mut channel = MsgChannel::default();
    /// let mut batch = Batch::default();
    /// batch.create_element("div", None);
    /// // add the batch to the channel
    /// channel.append(batch);
    /// channel.flush();
    /// ```
    pub fn append(&mut self, batch: Batch) {
        self.batch.append(batch);
    }

    /// IMPORTANT: This method is exicuted immediatly and does not wait for the next flush
    ///
    /// Run a batch of operations on the DOM immediately. This only runs the operations that are in the batch, not the operations that are queued in the [`MsgChannel`].
    ///
    /// Example:
    /// ```rust
    /// let mut channel = MsgChannel::default();
    /// let mut batch = Batch::default();
    /// batch.create_element("div", None);
    /// // add the batch to the channel
    /// channel.run_batch(&batch.finalize());
    /// ```
    pub fn run_batch(&mut self, batch: impl PreparedBatch) {
        run_batch(batch.msg(), batch.str(), &mut self.last_mem_size);
    }
}

fn run_batch(msg: &[u8], str_buf: &[u8], last_mem_size: &mut usize) {
    debug_assert_eq!(0usize.to_le_bytes().len(), 32 / 8);
    let msg_ptr = msg.as_ptr() as usize;
    let str_ptr = str_buf.as_ptr() as usize;
    // the pointer will only be updated when the message vec is resized, so we have a flag to check if the pointer has changed to avoid unnecessary decoding
    if unsafe { *MSG_METADATA_PTR } == 255 {
        // this is the first message, so we need to encode all the metadata
        unsafe {
            let mut_ptr_ptr: *mut usize = std::mem::transmute(MSG_PTR_PTR);
            *mut_ptr_ptr = msg_ptr;
            let mut_metadata_ptr: *mut u8 = std::mem::transmute(MSG_METADATA_PTR);
            // the first bit encodes if the msg pointer has changed
            *mut_metadata_ptr = 1;
            let mut_str_ptr_ptr: *mut usize = std::mem::transmute(STR_PTR_PTR);
            *mut_str_ptr_ptr = str_ptr as usize;
            // the second bit encodes if the str pointer has changed
            *mut_metadata_ptr |= 2;
        }
    } else {
        if unsafe { *MSG_PTR_PTR } != msg_ptr {
            unsafe {
                let mut_ptr_ptr: *mut usize = std::mem::transmute(MSG_PTR_PTR);
                *mut_ptr_ptr = msg_ptr;
                let mut_ptr_ptr: *mut u8 = std::mem::transmute(MSG_METADATA_PTR);
                // the first bit encodes if the msg pointer has changed
                *mut_ptr_ptr = 1;
            }
        } else {
            unsafe {
                let mut_ptr_ptr: *mut u8 = std::mem::transmute(MSG_METADATA_PTR);
                // the first bit encodes if the msg pointer has changed
                *mut_ptr_ptr = 0;
            }
        }
        if unsafe { *STR_PTR_PTR } != str_ptr {
            unsafe {
                let mut_str_ptr_ptr: *mut usize = std::mem::transmute(STR_PTR_PTR);
                *mut_str_ptr_ptr = str_ptr as usize;
                let mut_metadata_ptr: *mut u8 = std::mem::transmute(MSG_METADATA_PTR);
                // the second bit encodes if the str pointer has changed
                *mut_metadata_ptr |= 1 << 1;
            }
        }
    }
    unsafe {
        let mut_metadata_ptr: *mut u8 = std::mem::transmute(MSG_METADATA_PTR);
        if !str_buf.is_empty() {
            // the third bit encodes if there is any strings
            *mut_metadata_ptr |= 1 << 2;
            let mut_str_len_ptr: *mut usize = std::mem::transmute(STR_LEN_PTR);
            *mut_str_len_ptr = str_buf.len() as usize;
            if *mut_str_len_ptr < 100 {
                // the fourth bit encodes if the strings are entirely ascii and small
                *mut_metadata_ptr |= (str_buf.is_ascii() as u8) << 3;
            }
        }
    }
    let new_mem_size = core::arch::wasm32::memory_size(0);
    // we need to update the memory if the memory has grown
    if new_mem_size != *last_mem_size {
        *last_mem_size = new_mem_size;
        update_last_memory(wasm_bindgen::memory());
    }

    // if last_needs_memory() {
    //     update_last_memory(wasm_bindgen::memory());
    // }
    work_last_created();
}

/// Something that can be written as a utf-8 string to a buffer
pub trait WritableText {
    fn write_as_text(self, to: &mut Vec<u8>);
}

impl WritableText for char {
    fn write_as_text(self, to: &mut Vec<u8>) {
        to.push(self as u8);
    }
}

impl<'a> WritableText for &'a str {
    #[inline(always)]
    fn write_as_text(self, to: &mut Vec<u8>) {
        let len = self.len();
        to.reserve(len);
        let old_len = to.len();
        #[allow(clippy::uninit_vec)]
        unsafe {
            let ptr = to.as_mut_ptr();
            let bytes = self.as_bytes();
            let str_ptr = bytes.as_ptr();
            for o in 0..len {
                *ptr.add(old_len + o) = *str_ptr.add(o);
            }
            to.set_len(old_len + len);
        }
        // let _ = to.write(self.as_bytes());
    }
}

impl WritableText for Arguments<'_> {
    fn write_as_text(self, to: &mut Vec<u8>) {
        let _ = to.write_fmt(self);
    }
}

impl<F> WritableText for F
where
    F: FnOnce(&mut Vec<u8>),
{
    fn write_as_text(self, to: &mut Vec<u8>) {
        self(to);
    }
}

macro_rules! write_unsized {
    ($t: ty) => {
        impl WritableText for $t {
            fn write_as_text(self, to: &mut Vec<u8>) {
                let mut n = self;
                let mut n2 = n;
                let mut num_digits = 0;
                while n2 > 0 {
                    n2 /= 10;
                    num_digits += 1;
                }
                let len = num_digits;
                to.reserve(len);
                let ptr = to.as_mut_ptr().cast::<u8>();
                let old_len = to.len();
                let mut i = len - 1;
                loop {
                    unsafe { ptr.add(old_len + i).write((n % 10) as u8 + b'0') }
                    n /= 10;

                    if n == 0 {
                        break;
                    } else {
                        i -= 1;
                    }
                }

                #[allow(clippy::uninit_vec)]
                unsafe {
                    to.set_len(old_len + (len - i));
                }
            }
        }
    };
}

macro_rules! write_sized {
    ($t: ty) => {
        impl WritableText for $t {
            fn write_as_text(self, to: &mut Vec<u8>) {
                let neg = self < 0;
                let mut n = if neg {
                    match self.checked_abs() {
                        Some(n) => n,
                        None => <$t>::MAX / 2 + 1,
                    }
                } else {
                    self
                };
                let mut n2 = n;
                let mut num_digits = 0;
                while n2 > 0 {
                    n2 /= 10;
                    num_digits += 1;
                }
                let len = if neg { num_digits + 1 } else { num_digits };
                to.reserve(len);
                let ptr = to.as_mut_ptr().cast::<u8>();
                let old_len = to.len();
                let mut i = len - 1;
                loop {
                    unsafe { ptr.add(old_len + i).write((n % 10) as u8 + b'0') }
                    n /= 10;

                    if n == 0 {
                        break;
                    } else {
                        i -= 1;
                    }
                }

                if neg {
                    i -= 1;
                    unsafe { ptr.add(old_len + i).write(b'-') }
                }

                #[allow(clippy::uninit_vec)]
                unsafe {
                    to.set_len(old_len + (len - i));
                }
            }
        }
    };
}

write_unsized!(u8);
write_unsized!(u16);
write_unsized!(u32);
write_unsized!(u64);
write_unsized!(u128);
write_unsized!(usize);

write_sized!(i8);
write_sized!(i16);
write_sized!(i32);
write_sized!(i64);
write_sized!(i128);
write_sized!(isize);
