use smallvec::SmallVec;
use std::fmt::Debug;
use web_sys::{console, Node};

use crate::{
    attribute::IntoAttribue,
    element::{ElementBuilderExt, IntoElement, ManyElements},
    event::IntoEvent,
    set_node,
    value::IntoValue,
    work,
};

pub trait VecLike {
    type Item;

    #[inline]
    fn add_element(&mut self, element: Self::Item);

    #[inline]
    fn extend_owned_slice<const N: usize>(&mut self, slice: [Self::Item; N]) {
        self.extend_slice(&slice)
    }

    #[inline]
    fn extend_slice(&mut self, slice: &[Self::Item]);

    #[inline]
    fn len(&self) -> usize;
}

impl<Item: Copy> VecLike for Vec<Item> {
    type Item = Item;

    fn add_element(&mut self, element: Self::Item) {
        self.push(element);
    }

    fn extend_slice(&mut self, slice: &[Self::Item]) {
        self.extend(slice.into_iter().copied());
    }

    fn len(&self) -> usize {
        self.len()
    }
}

impl<const N: usize, Item: Copy> VecLike for SmallVec<[Item; N]> {
    type Item = Item;

    fn add_element(&mut self, element: Self::Item) {
        self.push(element);
    }

    fn extend_slice(&mut self, slice: &[Self::Item]) {
        self.extend_from_slice(slice);
    }

    fn len(&self) -> usize {
        self.len()
    }
}

#[derive(Default)]
pub struct MsgBuilder<V: VecLike<Item = u8> + AsRef<[u8]>> {
    pub(crate) buf: V,
    // the number of bytes an id takes
    pub(crate) id_size: u8,
}

impl<V: VecLike<Item = u8> + AsRef<[u8]>> MsgBuilder<V> {
    pub const fn with(v: V) -> Self {
        Self { buf: v, id_size: 1 }
    }
}

impl MsgBuilder<Vec<u8>> {
    pub const fn new() -> Self {
        Self {
            buf: Vec::new(),
            id_size: 1,
        }
    }
}

enum Op {
    PushRoot = 0,
    PopRoot = 1,
    AppendChildren = 2,
    ReplaceWith = 3,
    InsertBefore = 4,
    InsertAfter = 5,
    Remove = 6,
    CreateTextNode = 7,
    CreateElement = 8,
    CreateElementNs = 9,
    CreatePlaceholder = 10,
    SetEventListener = 11,
    RemoveEventListener = 12,
    SetText = 13,
    SetAttribute = 14,
    RemoveAttribute = 15,
    RemoveAttributeNs = 16,
    SetIdSize = 17,
    CreateFullElement = 18,
    CreateTemplate = 19,
    CreateTemplateRef = 20,
}

impl<V: VecLike<Item = u8> + AsRef<[u8]> + Debug> MsgBuilder<V> {
    pub fn create_full_element(&mut self, builder: impl ElementBuilderExt) {
        self.buf.add_element(Op::CreateFullElement as u8);
        builder.encode(&mut self.buf, self.id_size);
    }

    pub fn check_id(&mut self, id: [u8; 8]) {
        let first_contentful_byte = id.iter().rev().position(|&b| b != 0).unwrap_or(id.len());
        let contentful_size = id.len() - first_contentful_byte;
        self.check_id_size(contentful_size as u8);
    }

    pub fn check_id_size(&mut self, size: u8) {
        if size > self.id_size {
            self.set_id_size(size);
        }
    }

    pub fn create_element(&mut self, element: impl IntoElement, id: Option<u64>) {
        let id = id.map(|id| id.to_le_bytes());
        if let Some(id) = id {
            self.check_id(id);
        }
        self.buf.add_element(Op::CreateElement as u8);
        if let Some(id) = id {
            let contentful_id = &id[..self.id_size as usize];
            self.buf.extend_slice(contentful_id);
        } else {
            self.buf.add_element(0);
        }
        element.encode(&mut self.buf);
    }

    pub fn create_element_ns(&mut self, element: impl IntoElement, ns: &str, id: Option<u64>) {
        let id = id.map(|id| id.to_le_bytes());
        if let Some(id) = id {
            self.check_id(id);
        }
        self.buf.add_element(Op::CreateElementNs as u8);
        if let Some(id) = id {
            let contentful_id = &id[..self.id_size as usize];
            self.buf.extend_slice(contentful_id);
        } else {
            self.buf.add_element(0);
        }
        element.encode(&mut self.buf);
        encode_str(&mut self.buf, ns);
    }

    pub fn create_placeholder(&mut self, id: u64) {
        let id = id.to_le_bytes();
        self.check_id(id);
        self.buf.add_element(Op::CreatePlaceholder as u8);
        let contentful_id = &id[..self.id_size as usize];
        self.buf.extend_slice(contentful_id);
    }

    pub fn create_text_node(&mut self, text: &str, id: Option<u64>) {
        let id = id.map(|id| id.to_le_bytes());
        if let Some(id) = id {
            self.check_id(id);
        }
        self.buf.add_element(Op::CreateTextNode as u8);
        if let Some(id) = id {
            let contentful_id = &id[..self.id_size as usize];
            self.buf.extend_slice(contentful_id);
        } else {
            self.buf.add_element(0);
        }
        encode_str(&mut self.buf, text);
    }

    pub fn set_attribute(
        &mut self,
        attribute: impl IntoAttribue,
        value: impl IntoValue,
        id: Option<u64>,
    ) {
        let id = id.map(|id| id.to_le_bytes());
        if let Some(id) = id {
            self.check_id(id);
        }
        self.buf.add_element(Op::SetAttribute as u8);
        if let Some(id) = id {
            let contentful_id = &id[..self.id_size as usize];
            self.buf.extend_slice(contentful_id);
        } else {
            self.buf.add_element(0);
        }
        attribute.encode(&mut self.buf);
        value.encode(&mut self.buf);
    }

    pub fn remove_attribute(&mut self, attribute: impl IntoAttribue, id: u64) {
        let id = id.to_le_bytes();
        self.check_id(id);
        self.buf.add_element(Op::RemoveAttribute as u8);
        let contentful_id = &id[..self.id_size as usize];
        self.buf.extend_slice(contentful_id);
        attribute.encode(&mut self.buf);
    }

    pub fn remove_attribute_ns(&mut self, attribute: impl IntoAttribue, ns: &str, id: u64) {
        let id = id.to_le_bytes();
        self.check_id(id);
        self.buf.add_element(Op::RemoveAttributeNs as u8);
        let contentful_id = &id[..self.id_size as usize];
        self.buf.extend_slice(contentful_id);
        attribute.encode(&mut self.buf);
        encode_str(&mut self.buf, ns);
    }

    pub fn append_children(&mut self, children: u8) {
        self.buf.add_element(Op::AppendChildren as u8);
        self.buf.add_element(children);
    }

    pub fn push_root(&mut self, id: u64) {
        let id = id.to_le_bytes();
        self.check_id(id);
        self.buf.add_element(Op::PushRoot as u8);
        let contentful_id = &id[..self.id_size as usize];
        self.buf.extend_slice(contentful_id);
    }

    pub fn pop_root(&mut self) {
        self.buf.add_element(Op::PopRoot as u8);
    }

    pub fn insert_after(&mut self, id: u64, num: u32) {
        let id = id.to_le_bytes();
        self.check_id(id);
        self.buf.add_element(Op::InsertAfter as u8);
        let contentful_id = &id[..self.id_size as usize];
        self.buf.extend_slice(contentful_id);
        self.buf.extend_slice(&num.to_le_bytes());
    }

    pub fn insert_before(&mut self, id: u64, num: u32) {
        let id = id.to_le_bytes();
        self.check_id(id);
        self.buf.add_element(Op::InsertBefore as u8);
        let contentful_id = &id[..self.id_size as usize];
        self.buf.extend_slice(contentful_id);
        self.buf.extend_slice(&num.to_le_bytes());
    }

    pub fn remove(&mut self, id: u64) {
        let id = id.to_le_bytes();
        self.check_id(id);
        self.buf.add_element(Op::Remove as u8);
        let contentful_id = &id[..self.id_size as usize];
        self.buf.extend_slice(contentful_id);
    }

    pub fn set_event_listener(&mut self, event: impl IntoEvent, id: u64) {
        let id = id.to_le_bytes();
        self.check_id(id);
        self.buf.add_element(Op::SetEventListener as u8);
        let contentful_id = &id[..self.id_size as usize];
        self.buf.extend_slice(contentful_id);
        event.encode(&mut self.buf);
    }

    pub fn remove_event_listener(&mut self, event: impl IntoEvent, id: u64) {
        let id = id.to_le_bytes();
        self.check_id(id);
        self.buf.add_element(Op::RemoveEventListener as u8);
        let contentful_id = &id[..self.id_size as usize];
        self.buf.extend_slice(contentful_id);
        event.encode(&mut self.buf);
    }

    pub fn set_id_size(&mut self, id_size: u8) {
        self.id_size = id_size;
        self.buf.add_element(Op::SetIdSize as u8);
        self.buf.add_element(id_size);
    }

    pub fn set_node(&mut self, id: u64, node: Node) {
        set_node(id, node);
    }

    pub fn replace_with(&mut self, id: u64, num: u32) {
        let id = id.to_le_bytes();
        self.check_id(id);
        self.buf.add_element(Op::ReplaceWith as u8);
        let contentful_id = &id[..self.id_size as usize];
        self.buf.extend_slice(contentful_id);
        self.buf.extend_slice(&num.to_le_bytes());
    }

    pub fn set_text(&mut self, id: u64, text: &str) {
        let id = id.to_le_bytes();
        self.check_id(id);
        self.buf.add_element(Op::SetText as u8);
        let contentful_id = &id[..self.id_size as usize];
        self.buf.extend_slice(contentful_id);
        encode_str(&mut self.buf, text);
    }

    pub fn create_template(&mut self, builder: impl ManyElements, id: u64) {
        let id = id.to_le_bytes();
        self.check_id(id);
        self.check_id_size(builder.max_id_size());
        self.buf.add_element(Op::CreateTemplate as u8);
        let contentful_id = &id[..self.id_size as usize];
        self.buf.extend_slice(contentful_id);
        builder.encode(&mut self.buf, self.id_size);
    }

    pub fn create_template_ref(&mut self, template_id: u64, node_id: Option<u64>) {
        let template_id = template_id.to_le_bytes();
        self.check_id(template_id);
        let node_id = node_id.map(|id| id.to_le_bytes());
        if let Some(id) = node_id {
            self.check_id(id);
        }
        self.buf.add_element(Op::CreateTemplateRef as u8);
        let contentful_id = &template_id[..self.id_size as usize];
        self.buf.extend_slice(contentful_id);
        if let Some(id) = node_id {
            let contentful_id = &id[..self.id_size as usize];
            self.buf.extend_slice(contentful_id);
        } else {
            self.buf.add_element(0);
        }
    }

    pub fn build(&self) {
        work(self.buf.as_ref())
    }
}

pub fn encode_str<V: VecLike<Item = u8>>(buf: &mut V, s: &str) {
    let b = s.as_bytes();
    let len = b.len();
    buf.add_element(len as u8);
    buf.extend_slice(b);
}
