use std::{convert::Infallible, fmt::Arguments, io::Write};

use ufmt::{uWrite, uwrite};
use web_sys::Node;

use crate::{
    batch::{Batch, Op},
    last_needs_memory, update_last_memory, work_last_created, ElementBuilder, IntoAttribue,
    IntoElement, JsInterpreter, MSG_METADATA_PTR, MSG_PTR_PTR, STR_LEN_PTR, STR_PTR_PTR,
};

static mut INTERPRETER_EXISTS: bool = false;

/// An id that may be either the last node or a node with an assigned id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MaybeId {
    /// The last node that was created or navigated to.
    LastNode,
    /// A node that was created and stored with an id
    Node(NodeId),
}

/// A node that was created and stored with an id
/// It is recommended to create and store ids with a slab allocator with an exposed slab index for example the excellent [slab](https://docs.rs/slab) crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(pub u32);

/// The [`MsgChannel`] handles communication with the dom. It allows you to send batched operations to the dom.
/// All of the functions that are not marked otherwise are qued and not exicuted imidately. When you want to exicute the que you have to call [`MsgChannel::flush`].
/// There should only be one msg channel per program.
pub struct MsgChannel {
    pub(crate) js_interpreter: JsInterpreter,
    batch: Batch,
}

impl Default for MsgChannel {
    fn default() -> Self {
        unsafe {
            assert!(
                !INTERPRETER_EXISTS,
                "Found another MsgChannel. Only one MsgChannel can be created"
            );
            INTERPRETER_EXISTS = true;
        }
        assert!(0x1F > Op::CloneNodeChildren as u8);
        format!(
            "init: {:?}, {:?}, {:?}",
            unsafe { MSG_PTR_PTR as usize },
            unsafe { STR_PTR_PTR as usize },
            unsafe { STR_LEN_PTR as usize }
        );
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
            batch: Batch::default(),
        }
    }
}

impl MsgChannel {
    /// IMPORTANT: Unlike other methods this method is exicuted immediatly and does not wait for the next flush
    pub fn set_node(&mut self, id: NodeId, node: Node) {
        self.js_interpreter.SetNode(id.0, node);
    }

    /// IMPORTANT: Unlike other methods this method is exicuted immediatly and does not wait for the next flush
    pub fn get_node(&mut self, id: NodeId) -> Node {
        self.js_interpreter.GetNode(id.0)
    }

    /// Exicutes any queued operations in the order they were added
    #[inline]
    pub fn flush(&mut self) {
        debug_assert_eq!(0usize.to_le_bytes().len(), 32 / 8);
        self.batch.encode_op(Op::Stop);
        let msg_ptr = self.batch.msg.as_ptr() as usize;
        // the pointer will only be updated when the message vec is resized, so we have a flag to check if the pointer has changed to avoid unnecessary decoding
        if unsafe { *MSG_PTR_PTR } != msg_ptr || unsafe { *MSG_METADATA_PTR } == 255 {
            unsafe {
                let mut_ptr_ptr: *mut usize = std::mem::transmute(MSG_PTR_PTR);
                *mut_ptr_ptr = msg_ptr;
                let mut_ptr_ptr: *mut usize = std::mem::transmute(MSG_METADATA_PTR);
                *mut_ptr_ptr = 1;
            }
        } else {
            unsafe {
                let mut_ptr_ptr: *mut usize = std::mem::transmute(MSG_METADATA_PTR);
                *mut_ptr_ptr = 0;
            }
        }
        unsafe {
            let mut_str_ptr_ptr: *mut usize = std::mem::transmute(STR_PTR_PTR);
            *mut_str_ptr_ptr = self.batch.str_buf.as_ptr() as usize;
            let mut_str_len_ptr: *mut usize = std::mem::transmute(STR_LEN_PTR);
            *mut_str_len_ptr = self.batch.str_buf.len() as usize;
            let mut_ptr_ptr: *mut usize = std::mem::transmute(MSG_METADATA_PTR);
            *mut_ptr_ptr |= (!self.batch.str_buf.is_empty() as usize) << 1;
            if *mut_str_len_ptr < 100 {
                *mut_ptr_ptr |= (self.batch.str_buf.is_ascii() as usize) << 2;
            }
        }
        if last_needs_memory() {
            update_last_memory(wasm_bindgen::memory());
        }
        work_last_created();
        self.batch.msg.clear();
        self.batch.current_op_batch_idx = 0;
        self.batch.current_op_byte_idx = 3;
        self.batch.str_buf.clear();
    }

    /// Appends a number of nodes as children of the given node.
    pub fn append_child(&mut self, root: MaybeId, child: NodeId) {
        self.batch.append_child(root, child)
    }

    /// Appends a number of nodes as children of the given node.
    pub fn append_children(&mut self, root: MaybeId, children: Vec<NodeId>) {
        self.batch.append_children(root, children)
    }

    /// Replace a node with another node
    pub fn replace_with(&mut self, root: MaybeId, node: NodeId) {
        self.batch.replace_with(root, node)
    }

    /// Replace a node with a number of nodes
    pub fn replace_with_nodes(&mut self, root: MaybeId, nodes: Vec<NodeId>) {
        self.batch.replace_with_nodes(root, nodes)
    }

    /// Insert a single node after a given node.
    pub fn insert_after(&mut self, root: MaybeId, node: NodeId) {
        self.batch.insert_after(root, node)
    }

    /// Insert a number of nodes after a given node.
    pub fn insert_nodes_after(&mut self, root: MaybeId, nodes: &[NodeId]) {
        self.batch.insert_nodes_after(root, nodes)
    }

    /// Insert a single node before a given node.
    pub fn insert_before(&mut self, root: MaybeId, node: NodeId) {
        self.batch.insert_before(root, node)
    }

    /// Insert a number of nodes before a given node.
    pub fn insert_nodes_before(&mut self, root: MaybeId, nodes: &[NodeId]) {
        self.batch.insert_nodes_before(root, nodes)
    }

    /// Remove a node from the DOM.
    pub fn remove(&mut self, id: MaybeId) {
        self.batch.remove(id)
    }

    /// Create a new text node
    pub fn create_text_node(&mut self, text: impl WritableText, id: MaybeId) {
        self.batch.create_text_node(text, id)
    }

    /// Create a new element node
    pub fn create_element<'a, 'b>(&mut self, tag: impl IntoElement<'a, 'b>, id: Option<NodeId>) {
        self.batch.create_element(tag, id)
    }

    /// Set the textcontent of a node.
    pub fn set_text(&mut self, text: impl WritableText, root: MaybeId) {
        self.batch.set_text(text, root)
    }

    /// Set the value of a node's attribute.
    pub fn set_attribute<'a, 'b>(
        &mut self,
        attr: impl IntoAttribue<'a, 'b>,
        value: impl WritableText,
        root: MaybeId,
    ) {
        self.batch.set_attribute(attr, value, root)
    }

    /// Remove an attribute from a node.
    pub fn remove_attribute<'a, 'b>(&mut self, attr: impl IntoAttribue<'a, 'b>, root: MaybeId) {
        self.batch.remove_attribute(attr, root)
    }

    /// Clone a node and store it with a new id.
    pub fn clone_node(&mut self, id: MaybeId, new_id: MaybeId) {
        self.batch.clone_node(id, new_id)
    }

    /// Clone the children of a given node and store them with new ids.
    pub fn clone_node_children(&mut self, id: MaybeId, new_ids: Vec<NodeId>) {
        self.batch.clone_node_children(id, new_ids)
    }

    /// Move the last node to the first child
    pub fn first_child(&mut self) {
        self.batch.first_child()
    }

    /// Move the last node to the next sibling
    pub fn next_sibling(&mut self) {
        self.batch.next_sibling()
    }

    /// Move the last node to the parent node
    pub fn parent_node(&mut self) {
        self.batch.parent_node()
    }

    /// Store the last node with the given id. This is useful when traversing the document tree.
    pub fn store_with_id(&mut self, id: NodeId) {
        self.batch.store_with_id(id)
    }

    /// Set the last node to the given id. The last node can be used to traverse the document tree without passing objects between wasm and js every time.
    pub fn set_last_node(&mut self, id: NodeId) {
        self.batch.set_last_node(id)
    }

    /// Build a full element, slightly more efficent than creating the element creating the element with `create_element` and then setting the attributes.
    pub fn build_full_element(&mut self, el: ElementBuilder) {
        self.batch.build_full_element(el)
    }

    /// Set a style property on a node.
    pub fn set_style(&mut self, style: &str, value: &str, id: MaybeId) {
        self.batch.set_style(style, value, id)
    }

    /// Remove a style property from a node.
    pub fn remove_style(&mut self, style: &str, id: MaybeId) {
        self.batch.remove_style(style, id)
    }
}

/// Something that can be written as a utf-8 string to a buffer
pub trait WritableText {
    fn write_as_text(self, to: &mut Vec<u8>);
}

impl<'a> WritableText for &'a str {
    fn write_as_text(self, to: &mut Vec<u8>) {
        to.extend_from_slice(self.as_bytes());
    }
}

impl WritableText for Arguments<'_> {
    fn write_as_text(self, to: &mut Vec<u8>) {
        let _ = to.write_fmt(self);
    }
}

/// A wrapper around a `Vec<u8>` that can implement `Writable`
pub struct WritableVecWrapper<'a>(&'a mut Vec<u8>);

impl uWrite for WritableVecWrapper<'_> {
    type Error = Infallible;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        self.0.extend_from_slice(s.as_bytes());
        Ok(())
    }
}

impl WritableText for u8 {
    fn write_as_text(self, to: &mut Vec<u8>) {
        let mut v = WritableVecWrapper(to);
        let _ = uwrite!(v, "{}", self);
    }
}

impl WritableText for u16 {
    fn write_as_text(self, to: &mut Vec<u8>) {
        let mut v = WritableVecWrapper(to);
        let _ = uwrite!(v, "{}", self);
    }
}

impl WritableText for u32 {
    fn write_as_text(self, to: &mut Vec<u8>) {
        let mut v = WritableVecWrapper(to);
        let _ = uwrite!(v, "{}", self);
    }
}

impl WritableText for u64 {
    fn write_as_text(self, to: &mut Vec<u8>) {
        let mut v = WritableVecWrapper(to);
        let _ = uwrite!(v, "{}", self);
    }
}

impl WritableText for usize {
    fn write_as_text(self, to: &mut Vec<u8>) {
        let mut v = WritableVecWrapper(to);
        let _ = uwrite!(v, "{}", self);
    }
}

impl WritableText for i8 {
    fn write_as_text(self, to: &mut Vec<u8>) {
        let mut v = WritableVecWrapper(to);
        let _ = uwrite!(v, "{}", self);
    }
}

impl WritableText for i16 {
    fn write_as_text(self, to: &mut Vec<u8>) {
        let mut v = WritableVecWrapper(to);
        let _ = uwrite!(v, "{}", self);
    }
}

impl WritableText for i32 {
    fn write_as_text(self, to: &mut Vec<u8>) {
        let mut v = WritableVecWrapper(to);
        let _ = uwrite!(v, "{}", self);
    }
}

impl WritableText for i64 {
    fn write_as_text(self, to: &mut Vec<u8>) {
        let mut v = WritableVecWrapper(to);
        let _ = uwrite!(v, "{}", self);
    }
}

impl WritableText for isize {
    fn write_as_text(self, to: &mut Vec<u8>) {
        let mut v = WritableVecWrapper(to);
        let _ = uwrite!(v, "{}", self);
    }
}

impl<F> WritableText for F
where
    F: FnOnce(WritableVecWrapper),
{
    fn write_as_text(self, to: &mut Vec<u8>) {
        self(WritableVecWrapper(to));
    }
}
