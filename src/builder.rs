use std::{convert::Infallible, fmt::Arguments, io::Write};

use ufmt::{uWrite, uwrite};
use web_sys::Node;

use crate::{
    element::ElementBuilderExt, last_needs_memory, update_last_memory, work_last_created,
    IntoAttribue, IntoElement, JsInterpreter, MSG_METADATA_PTR, MSG_PTR_PTR, STR_LEN_PTR,
    STR_PTR_PTR,
};

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
pub struct MsgChannel {
    pub(crate) msg: Vec<u8>,
    pub(crate) str_buf: Vec<u8>,
    pub(crate) js_interpreter: JsInterpreter,
    current_op_batch_idx: usize,
    current_op_byte_idx: usize,
    current_op_bit_pack_index: u8,
}

impl MsgChannel {
    /// Create a MsgChannel with the specified Vecs and root element
    fn with(v: Vec<u8>, v2: Vec<u8>) -> Self {
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
            msg: v,
            str_buf: v2,
            js_interpreter,
            current_op_byte_idx: 3,
            current_op_bit_pack_index: 0,
            current_op_batch_idx: 0,
        }
    }
}

impl Default for MsgChannel {
    fn default() -> Self {
        Self::with(Vec::new(), Vec::new())
    }
}

// operations that have no booleans can be encoded as a half byte, these are placed first
enum Op {
    /// Navigates to the last node to the first child of the current node.
    FirstChild = 0,

    /// Navigates to the last node to the last child of the current node.
    NextSibling = 1,

    /// Navigates to the last node to the parent of the current node.
    ParentNode = 2,

    /// Stores the last node with a new id.
    StoreWithId = 3,

    /// Manually set the last node.
    SetLastNode = 4,

    /// Stop
    Stop = 5,

    /// Build Full Element
    BuildFullElement = 6,

    /// Pop the topmost node from our stack and append them to the node
    AppendChildren = 7,

    /// Replace a given (single) node with a handful of nodes currently on the stack.
    ReplaceWith = 8,

    /// Insert a number of nodes after a given node.
    InsertAfter = 9,

    /// Insert a number of nodes before a given node.
    InsertBefore = 10,

    /// Remove a particular node from the DOM
    Remove = 11,

    /// Create a new text node
    CreateTextNode = 12,

    /// Create a new element node
    CreateElement = 13,

    /// Set the textcontent of a node.
    SetText = 14,

    /// Set the value of a node's attribute.
    SetAttribute = 15,

    /// Remove an attribute from a node.
    RemoveAttribute = 16,

    SetStyle = 17,

    RemoveStyle = 18,

    /// Clones a node.
    CloneNode = 19,

    /// Clones the children of a node. (allows cloning fragments)
    CloneNodeChildren = 20,
}

impl MsgChannel {
    /// Appends a number of nodes as children of the given node.
    pub fn append_child(&mut self, root: MaybeId, child: NodeId) {
        self.encode_op(Op::AppendChildren);
        self.encode_maybe_id(root);
        self.encode_bool(false);
        self.encode_id(child);
    }

    /// Appends a number of nodes as children of the given node.
    pub fn append_children(&mut self, root: MaybeId, children: Vec<NodeId>) {
        self.encode_op(Op::AppendChildren);
        self.encode_maybe_id(root);
        self.encode_bool(true);
        self.encode_u32(children.len() as u32);
        for child in children {
            self.encode_id(child);
        }
    }

    /// Replace a node with another node
    pub fn replace_with(&mut self, root: MaybeId, node: NodeId) {
        self.encode_op(Op::ReplaceWith);
        self.encode_maybe_id(root);
        self.encode_bool(false);
        self.encode_id(node);
    }

    /// Replace a node with a number of nodes
    pub fn replace_with_nodes(&mut self, root: MaybeId, nodes: Vec<NodeId>) {
        self.encode_op(Op::ReplaceWith);
        self.encode_maybe_id(root);
        self.encode_bool(true);
        self.encode_u32(nodes.len() as u32);
        for node in nodes {
            self.encode_id(node);
        }
    }

    /// Insert a single node after a given node.
    pub fn insert_after(&mut self, root: MaybeId, node: NodeId) {
        self.encode_op(Op::InsertAfter);
        self.encode_maybe_id(root);
        self.encode_bool(false);
        self.encode_id(node);
    }

    /// Insert a number of nodes after a given node.
    pub fn insert_nodes_after(&mut self, root: MaybeId, nodes: &[NodeId]) {
        self.encode_op(Op::InsertAfter);
        self.encode_maybe_id(root);
        self.encode_bool(true);
        self.encode_u32(nodes.len() as u32);
        for node in nodes {
            self.encode_id(*node);
        }
    }

    /// Insert a single node before a given node.
    pub fn insert_before(&mut self, root: MaybeId, node: NodeId) {
        self.encode_op(Op::InsertBefore);
        self.encode_maybe_id(root);
        self.encode_bool(false);
        self.encode_id(node);
    }

    /// Insert a number of nodes before a given node.
    pub fn insert_nodes_before(&mut self, root: MaybeId, nodes: &[NodeId]) {
        self.encode_op(Op::InsertBefore);
        self.encode_maybe_id(root);
        self.encode_bool(true);
        self.encode_u32(nodes.len() as u32);
        for node in nodes {
            self.encode_id(*node);
        }
    }

    /// Remove a node from the DOM.
    pub fn remove(&mut self, id: MaybeId) {
        self.encode_op(Op::Remove);
        self.encode_maybe_id(id);
    }

    /// Create a new text node
    pub fn create_text_node(&mut self, text: impl WritableText, id: MaybeId) {
        self.encode_op(Op::CreateTextNode);
        self.encode_str(text);
        self.encode_maybe_id(id);
    }

    /// Create a new element node
    pub fn create_element(&mut self, tag: impl IntoElement, id: Option<NodeId>) {
        self.encode_op(Op::CreateElement);
        tag.encode(self);
        self.encode_optional_id(id);
    }

    /// Set the textcontent of a node.
    pub fn set_text(&mut self, text: impl WritableText, root: MaybeId) {
        self.encode_op(Op::SetText);
        self.encode_maybe_id(root);
        self.encode_str(text);
    }

    /// Set the value of a node's attribute.
    pub fn set_attribute(
        &mut self,
        attr: impl IntoAttribue,
        value: impl WritableText,
        root: MaybeId,
    ) {
        self.encode_op(Op::SetAttribute);
        self.encode_maybe_id(root);
        attr.encode(self);
        self.encode_str(value);
    }

    /// Remove an attribute from a node.
    pub fn remove_attribute(&mut self, attr: impl IntoAttribue, root: MaybeId) {
        self.encode_op(Op::RemoveAttribute);
        self.encode_maybe_id(root);
        attr.encode(self);
    }

    /// Clone a node and store it with a new id.
    pub fn clone_node(&mut self, id: MaybeId, new_id: MaybeId) {
        self.encode_op(Op::CloneNode);
        self.encode_maybe_id(id);
        self.encode_maybe_id(new_id);
    }

    /// Clone the children of a given node and store them with new ids.
    pub fn clone_node_children(&mut self, id: MaybeId, new_ids: Vec<NodeId>) {
        self.encode_op(Op::CloneNodeChildren);
        self.encode_maybe_id(id);
        for id in new_ids {
            self.encode_optional_id_with_byte_bool(Some(id));
        }
    }

    /// Move the last node to the first child
    pub fn first_child(&mut self) {
        self.encode_op(Op::FirstChild);
    }

    /// Move the last node to the next sibling
    pub fn next_sibling(&mut self) {
        self.encode_op(Op::NextSibling);
    }

    /// Move the last node to the parent node
    pub fn parent_node(&mut self) {
        self.encode_op(Op::ParentNode);
    }

    /// Store the last node with the given id. This is useful when traversing the document tree.
    pub fn store_with_id(&mut self, id: NodeId) {
        self.encode_op(Op::StoreWithId);
        self.encode_id(id);
    }

    /// Set the last node to the given id. The last node can be used to traverse the document tree without passing objects between wasm and js every time.
    pub fn set_last_node(&mut self, id: NodeId) {
        self.encode_op(Op::SetLastNode);
        self.encode_id(id);
    }

    /// Build a full element, slightly more efficent than creating the element creating the element with `create_element` and then setting the attributes.
    pub fn build_full_element(&mut self, el: impl ElementBuilderExt) {
        self.encode_op(Op::BuildFullElement);
        el.encode(self);
    }

    /// Set a style property on a node.
    pub fn set_style(&mut self, style: &str, value: &str, id: MaybeId) {
        self.encode_op(Op::SetStyle);
        self.encode_maybe_id(id);
        self.encode_str(style);
        self.encode_str(value);
    }

    /// Remove a style property from a node.
    pub fn remove_style(&mut self, style: &str, id: MaybeId) {
        self.encode_op(Op::RemoveStyle);
        self.encode_maybe_id(id);
        self.encode_str(style);
    }

    #[inline]
    pub(crate) fn encode_optional_id(&mut self, id: Option<NodeId>) {
        match id {
            Some(id) => {
                self.encode_bool(true);
                self.encode_id(id);
            }
            None => {
                self.encode_bool(false);
            }
        }
    }

    #[inline]
    pub(crate) fn encode_maybe_id(&mut self, id: MaybeId) {
        match id {
            MaybeId::Node(id) => {
                self.encode_bool(true);
                self.encode_id(id);
            }
            MaybeId::LastNode => {
                self.encode_bool(false);
            }
        }
    }

    #[inline]
    pub(crate) fn encode_optional_id_with_byte_bool(&mut self, id: Option<NodeId>) {
        match id {
            Some(id) => {
                self.msg.push(1);
                self.encode_id(id);
            }
            None => {
                self.msg.push(0);
            }
        }
    }

    #[inline]
    pub(crate) fn encode_id(&mut self, id: NodeId) {
        self.encode_u32(id.0);
    }

    #[inline]
    pub(crate) fn encode_u32(&mut self, val: u32) {
        let le = val.to_le();
        #[allow(clippy::uninit_vec)]
        unsafe {
            let len = self.msg.len();
            self.msg.reserve(4);
            self.msg.set_len(len + 4);
            self.msg.as_mut_ptr().add(len).cast::<u32>().write(le);
        }
    }

    #[inline]
    pub(crate) fn encode_u16(&mut self, val: u16) {
        let le = val.to_le();
        #[allow(clippy::uninit_vec)]
        unsafe {
            let len = self.msg.len();
            self.msg.reserve(2);
            self.msg.set_len(len + 2);
            self.msg.as_mut_ptr().add(len).cast::<u16>().write(le);
        }
    }

    #[inline]
    pub(crate) fn encode_str(&mut self, string: impl WritableText) {
        let prev_len = self.str_buf.len();
        string.write_as_text(&mut self.str_buf);
        let len = self.str_buf.len() - prev_len;
        self.encode_u16(len as u16);
    }

    pub(crate) fn encode_cachable_str(&mut self, string: impl WritableText) {
        let prev_len = self.str_buf.len();
        string.write_as_text(&mut self.str_buf);
        let len = self.str_buf.len() - prev_len;
        self.encode_u16(len as u16);
    }

    #[inline]
    fn encode_op(&mut self, op: Op) {
        let u8_op = op as u8;

        self.current_op_byte_idx += 1;
        if self.current_op_byte_idx - self.current_op_batch_idx < 4 {
            self.msg[self.current_op_byte_idx] = u8_op;
        } else {
            self.current_op_batch_idx = self.msg.len();
            self.current_op_byte_idx = self.current_op_batch_idx;
            // reserve four bytes for the op batch
            #[allow(clippy::uninit_vec)]
            unsafe {
                let len = self.msg.len();
                self.msg.reserve(4);
                self.msg.set_len(len + 4);
            }
            self.msg[self.current_op_batch_idx] = u8_op;
        }
        self.current_op_bit_pack_index = 0;
    }

    #[inline]
    pub(crate) fn encode_bool(&mut self, value: bool) {
        if self.current_op_bit_pack_index < 3 {
            if value {
                self.msg[self.current_op_byte_idx] |= 1 << (self.current_op_bit_pack_index + 5);
            }
            self.current_op_bit_pack_index += 1;
        } else {
            todo!("handle more than 3 bools in a op");
        }
    }

    /// Exicutes any queued operations in the order they were added
    #[inline]
    pub fn flush(&mut self) {
        debug_assert_eq!(0usize.to_le_bytes().len(), 32 / 8);
        self.encode_op(Op::Stop);
        let msg_ptr = self.msg.as_ptr() as usize;
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
            *mut_str_ptr_ptr = self.str_buf.as_ptr() as usize;
            let mut_str_len_ptr: *mut usize = std::mem::transmute(STR_LEN_PTR);
            *mut_str_len_ptr = self.str_buf.len() as usize;
            let mut_ptr_ptr: *mut usize = std::mem::transmute(MSG_METADATA_PTR);
            *mut_ptr_ptr |= (!self.str_buf.is_empty() as usize) << 1;
            if *mut_str_len_ptr < 100 {
                *mut_ptr_ptr |= (self.str_buf.is_ascii() as usize) << 2;
            }
        }
        if last_needs_memory() {
            update_last_memory(wasm_bindgen::memory());
        }
        work_last_created();
        self.msg.clear();
        self.current_op_batch_idx = 0;
        self.current_op_byte_idx = 3;
        self.str_buf.clear();
    }

    /// IMPORTANT: Unlike other methods this method is exicuted immediatly and does not wait for the next flush
    pub fn set_node(&mut self, id: NodeId, node: Node) {
        self.js_interpreter.SetNode(id.0, node);
    }

    /// IMPORTANT: Unlike other methods this method is exicuted immediatly and does not wait for the next flush
    pub fn get_node(&mut self, id: NodeId) -> Node {
        self.js_interpreter.GetNode(id.0)
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
