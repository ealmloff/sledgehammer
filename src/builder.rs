use std::{fmt::Arguments, io::Write};

use web_sys::{Element, Node};

use crate::{
    get_id_size, set_id_size, work_last_created, ElementBuilderExt, IntoAttribue, JsInterpreter,
    MSG_PTR_PTR, STR_LEN_PTR, STR_PTR_PTR,
};

pub(crate) fn id_size(bytes: [u8; 8]) -> u8 {
    let first_contentful_byte = bytes.iter().rev().position(|&b| b != 0).unwrap_or(8);
    (8 - first_contentful_byte) as u8
}

#[allow(clippy::len_without_is_empty)]
pub trait VecLike: AsRef<[u8]> + Write {
    fn add_element(&mut self, element: u8);

    #[inline]
    fn extend_owned_slice<const N: usize>(&mut self, slice: [u8; N]) {
        self.extend_slice(&slice)
    }

    fn extend_slice(&mut self, slice: &[u8]);

    fn len(&self) -> usize;

    fn clear(&mut self);

    fn set(&mut self, index: usize, value: u8);
}

impl VecLike for Vec<u8> {
    fn add_element(&mut self, element: u8) {
        self.push(element);
    }

    fn extend_slice(&mut self, slice: &[u8]) {
        self.extend(slice.iter().copied());
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn set(&mut self, index: usize, value: u8) {
        self[index] = value;
    }
}

// impl<const N: usize> VecLike for SmallVec<[u8; N]> {
//     fn add_element(&mut self, element: u8) {
//         self.push(element);
//     }

//     fn extend_slice(&mut self, slice: &[u8]) {
//         self.extend_from_slice(slice);
//     }

//     fn len(&self) -> usize {
//         self.len()
//     }

//     fn clear(&mut self) {
//         self.clear();
//     }
// }

pub struct MsgBuilder<V: VecLike + AsRef<[u8]> = Vec<u8>> {
    pub(crate) msg: V,
    pub(crate) str_buf: V,
    pub(crate) js_interpreter: JsInterpreter,
}

impl<V: VecLike + AsRef<[u8]>> MsgBuilder<V> {
    pub fn with(v: V, v2: V, el: Element) -> Self {
        format!(
            "init: {:?}, {:?}, {:?}",
            unsafe { MSG_PTR_PTR as usize },
            unsafe { STR_PTR_PTR as usize },
            unsafe { STR_LEN_PTR as usize }
        );
        let js_interpreter = unsafe {
            JsInterpreter::new(
                el,
                wasm_bindgen::memory(),
                MSG_PTR_PTR as usize,
                STR_PTR_PTR as usize,
                STR_LEN_PTR as usize,
            )
        };

        Self {
            msg: v,
            str_buf: v2,
            js_interpreter,
        }
    }
}

impl MsgBuilder<Vec<u8>> {
    pub fn new(el: Element) -> Self {
        Self::with(Vec::new(), Vec::new(), el)
    }
}

enum Op {
    /// Pop the topmost node from our stack and append them to the node
    /// at the top of the stack.
    // /// The parent to append nodes to.
    // root: Option<u64>,

    // /// The ids of the children to append.
    // children: Vec<u64>,
    AppendChildren = 0,

    /// Replace a given (single) node with a handful of nodes currently on the stack.
    // /// The ID of the node to be replaced.
    // root: Option<u64>,

    // /// The ids of the nodes to replace the root with.
    // nodes: Vec<u64>,
    ReplaceWith = 1,

    /// Insert a number of nodes after a given node.
    // /// The ID of the node to insert after.
    // root: Option<u64>,

    // /// The ids of the nodes to insert after the target node.
    // nodes: Vec<u64>,
    InsertAfter = 2,

    /// Insert a number of nodes before a given node.
    // /// The ID of the node to insert before.
    // root: Option<u64>,

    // /// The ids of the nodes to insert before the target node.
    // nodes: Vec<u64>,
    InsertBefore = 3,

    /// Remove a particular node from the DOM
    // /// The ID of the node to remove.
    // root: Option<u64>,
    Remove = 4,

    /// Create a new purely-text node
    // /// The ID the new node should have.
    // root: Option<u64>,

    // /// The textcontent of the node
    // text: &'bump str,
    CreateTextNode = 5,

    /// Create a new purely-element node
    // /// The ID the new node should have.
    // root: Option<u64>,

    // /// The tagname of the node
    // tag: &'bump str,

    // /// The number of children nodes that will follow this message.
    // children: u32,
    /// Create a new purely-comment node with a given namespace
    // /// The ID the new node should have.
    // root: Option<u64>,

    // /// The namespace of the node
    // tag: &'bump str,

    // /// The namespace of the node (like `SVG`)
    // ns: &'static str,

    // /// The number of children nodes that will follow this message.
    // children: u32,
    CreateElement = 6,

    /// Create a new placeholder node.
    /// In most implementations, this will either be a hidden div or a comment node.
    // /// The ID the new node should have.
    // root: Option<u64>,
    CreatePlaceholder = 7,

    /// Set the textcontent of a node.
    // /// The ID of the node to set the textcontent of.
    // root: Option<u64>,

    // /// The textcontent of the node
    // text: &'bump str,
    SetText = 10,

    /// Set the value of a node's attribute.
    // /// The ID of the node to set the attribute of.
    // root: Option<u64>,

    // /// The name of the attribute to set.
    // field: &'static str,

    // /// The value of the attribute.
    // value: AttributeValue<'bump>,

    // // value: &'bump str,
    // /// The (optional) namespace of the attribute.
    // /// For instance, "style" is in the "style" namespace.
    // ns: Option<&'bump str>,
    SetAttribute = 11,

    /// Remove an attribute from a node.
    // /// The ID of the node to remove.
    // root: Option<u64>,

    // /// The name of the attribute to remove.
    // name: &'static str,

    // /// The namespace of the attribute.
    // ns: Option<&'bump str>,
    RemoveAttribute = 12,

    /// Clones a node.
    // /// The ID of the node to clone.
    // id: Option<u64>,

    // /// The ID of the new node.
    // new_id: u64,
    CloneNode = 13,

    /// Clones the children of a node. (allows cloning fragments)
    // /// The ID of the node to clone.
    // id: Option<u64>,

    // /// The ID of the new node.
    // new_ids: Vec<u64>,
    CloneNodeChildren = 14,

    /// Navigates to the last node to the first child of the current node.
    FirstChild = 15,

    /// Navigates to the last node to the last child of the current node.
    NextSibling = 16,

    /// Navigates to the last node to the parent of the current node.
    ParentNode = 17,

    /// Stores the last node with a new id.
    // /// The ID of the node to store.
    // id: u64,
    StoreWithId = 18,

    /// Manually set the last node.
    // /// The ID to set the last node to.
    // id: u64,
    SetLastNode = 19,

    /// Set id size
    SetIdSize = 20,

    /// Stop
    Stop = 21,

    /// Build Full Element
    BuildFullElement = 22,
}

impl<V: VecLike> MsgBuilder<V> {
    pub fn append_children(&mut self, root: Option<u64>, children: Vec<u64>) {
        let root = root.map(|id| self.check_id(id));
        for child in &children {
            self.check_id(*child);
        }
        self.msg.add_element(Op::AppendChildren as u8);
        self.encode_maybe_id(root);
        self.msg
            .extend_slice(&(children.len() as u32).to_le_bytes());
        for child in children {
            self.encode_id(child.to_le_bytes());
        }
    }

    pub fn replace_with(&mut self, root: Option<u64>, nodes: Vec<u64>) {
        let root = root.map(|id| self.check_id(id));
        for child in &nodes {
            self.check_id(*child);
        }
        self.msg.add_element(Op::ReplaceWith as u8);
        self.encode_maybe_id(root);
        self.msg.extend_slice(&(nodes.len() as u32).to_le_bytes());
        for node in nodes {
            self.encode_id(node.to_le_bytes());
        }
    }

    pub fn insert_after(&mut self, root: Option<u64>, nodes: Vec<u64>) {
        let root = root.map(|id| self.check_id(id));
        for child in &nodes {
            self.check_id(*child);
        }
        self.msg.add_element(Op::InsertAfter as u8);
        self.encode_maybe_id(root);
        self.msg.extend_slice(&(nodes.len() as u32).to_le_bytes());
        for node in nodes {
            self.encode_id(node.to_le_bytes());
        }
    }

    pub fn insert_before(&mut self, root: Option<u64>, nodes: Vec<u64>) {
        let root = root.map(|id| self.check_id(id));
        for child in &nodes {
            self.check_id(*child);
        }
        self.msg.add_element(Op::InsertBefore as u8);
        self.encode_maybe_id(root);
        self.msg.extend_slice(&(nodes.len() as u32).to_le_bytes());
        for node in nodes {
            self.encode_id(node.to_le_bytes());
        }
    }

    pub fn remove(&mut self, id: Option<u64>) {
        let root = id.map(|id| self.check_id(id));
        self.msg.add_element(Op::Remove as u8);
        self.encode_maybe_id(root);
    }

    pub fn create_text_node(&mut self, text: Arguments, id: Option<u64>) {
        let root = id.map(|id| self.check_id(id));
        self.msg.add_element(Op::CreateTextNode as u8);
        self.encode_str(text);
        self.encode_maybe_id(root);
    }

    pub fn create_element(
        &mut self,
        tag: Arguments,
        ns: Option<Arguments>,
        id: Option<u64>,
        children: u32,
    ) {
        let root = id.map(|id| self.check_id(id));
        self.msg.add_element(Op::CreateElement as u8);
        self.encode_cachable_str(tag);
        if let Some(ns) = ns {
            self.msg.add_element(1);
            self.encode_cachable_str(ns);
        } else {
            self.msg.add_element(0);
        }
        self.encode_maybe_id(root);
        self.msg.extend_slice(&children.to_le_bytes());
    }

    pub fn create_placeholder(&mut self, id: Option<u64>) {
        let root = id.map(|id| self.check_id(id));
        self.msg.add_element(Op::CreatePlaceholder as u8);
        self.encode_maybe_id(root);
    }

    pub fn set_text(&mut self, text: Arguments, root: Option<u64>) {
        let root = root.map(|id| self.check_id(id));
        self.msg.add_element(Op::SetText as u8);
        self.encode_maybe_id(root);
        self.encode_str(text);
    }

    pub fn set_attribute(&mut self, attr: impl IntoAttribue, value: Arguments, root: Option<u64>) {
        let root = root.map(|id| self.check_id(id));
        self.msg.add_element(Op::SetAttribute as u8);
        self.encode_maybe_id(root);
        attr.encode(self);
        self.encode_str(value);
    }

    pub fn remove_attribute(&mut self, attr: impl IntoAttribue, root: Option<u64>) {
        let root = root.map(|id| self.check_id(id));
        self.msg.add_element(Op::RemoveAttribute as u8);
        self.encode_maybe_id(root);
        attr.encode(self);
    }

    pub fn clone_node(&mut self, id: Option<u64>, new_id: Option<u64>) {
        let root = id.map(|id| self.check_id(id));
        let new_id = new_id.map(|id| self.check_id(id));
        self.msg.add_element(Op::CloneNode as u8);
        self.encode_maybe_id(root);
        self.encode_maybe_id(new_id);
    }

    pub fn clone_node_children(&mut self, id: Option<u64>, new_ids: Vec<u64>) {
        let root = id.map(|id| self.check_id(id));
        for id in &new_ids {
            self.check_id(*id);
        }
        self.msg.add_element(Op::CloneNodeChildren as u8);
        self.encode_maybe_id(root);
        for id in new_ids {
            self.encode_maybe_id(Some(id.to_le_bytes()));
        }
    }

    pub fn first_child(&mut self) {
        self.msg.add_element(Op::FirstChild as u8);
    }

    pub fn next_sibling(&mut self) {
        self.msg.add_element(Op::NextSibling as u8);
    }

    pub fn parent_node(&mut self) {
        self.msg.add_element(Op::ParentNode as u8);
    }

    pub fn store_with_id(&mut self, id: u64) {
        let id = self.check_id(id);
        self.msg.add_element(Op::StoreWithId as u8);
        self.encode_id(id);
    }

    pub fn set_last_node(&mut self, id: u64) {
        let id = self.check_id(id);
        self.msg.add_element(Op::SetLastNode as u8);
        self.encode_id(id);
    }

    pub fn build_full_element(&mut self, el: impl ElementBuilderExt) {
        self.msg.add_element(Op::BuildFullElement as u8);
        el.encode(self, get_id_size());
    }

    #[inline]
    pub(crate) fn encode_maybe_id(&mut self, id: Option<[u8; 8]>) {
        match id {
            Some(id) => {
                self.msg.add_element(1);
                self.encode_id(id);
            }
            None => {
                self.msg.add_element(0);
            }
        }
    }

    #[inline]
    pub(crate) fn encode_id(&mut self, bytes: [u8; 8]) {
        self.msg.extend_slice(&bytes[..(get_id_size() as usize)]);
    }

    #[inline]
    fn check_id(&mut self, id: u64) -> [u8; 8] {
        let bytes = id.to_le_bytes();
        let byte_size = id_size(bytes);
        if byte_size > get_id_size() {
            self.set_byte_size(byte_size);
        }
        bytes
    }

    #[inline]
    fn set_byte_size(&mut self, byte_size: u8) {
        set_id_size(byte_size);
        self.msg.add_element(Op::SetIdSize as u8);
        self.msg.add_element(byte_size);
    }

    pub(crate) fn encode_str(&mut self, string: Arguments) {
        let prev_len = self.str_buf.len();
        self.str_buf.write_fmt(string).unwrap();
        let len = self.str_buf.len() - prev_len;
        self.msg.extend_slice(&(len as u16).to_le_bytes());
    }

    pub(crate) fn encode_cachable_str(&mut self, string: Arguments) {
        let prev_len = self.str_buf.len();
        self.str_buf.write_fmt(string).unwrap();
        let len = self.str_buf.len() - prev_len;
        self.msg.extend_slice(&(len as u16).to_le_bytes());
    }

    #[inline]
    pub fn flush(&mut self) {
        assert_eq!(0usize.to_le_bytes().len(), 32 / 8);
        self.msg.add_element(Op::Stop as u8);
        unsafe {
            let mut_ptr_ptr: *mut usize = std::mem::transmute(MSG_PTR_PTR);
            *mut_ptr_ptr = self.msg.as_ref().as_ptr() as usize;
            let mut_str_ptr_ptr: *mut usize = std::mem::transmute(STR_PTR_PTR);
            *mut_str_ptr_ptr = self.str_buf.as_ref().as_ptr() as usize;
            let mut_str_len_ptr: *mut usize = std::mem::transmute(STR_LEN_PTR);
            *mut_str_len_ptr = self.str_buf.len() as usize;
        }
        work_last_created(wasm_bindgen::memory());
        self.msg.clear();
        self.str_buf.clear();
    }

    pub fn set_node(&mut self, id: u64, node: Node) {
        self.js_interpreter.SetNode(id, node);
    }
}
