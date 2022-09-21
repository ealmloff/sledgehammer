use smallvec::SmallVec;
use std::fmt::Debug;
use web_sys::Node;

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

    pub fn check_id(&mut self, id: &impl IntoId) {
        self.check_id_size(id.max_el_size());
    }

    pub fn check_id_size(&mut self, size: u8) {
        if size > self.id_size {
            self.set_id_size(size);
        }
    }

    pub fn create_element(&mut self, element: impl IntoElement, id: impl IntoId) {
        self.check_id(&id);
        self.buf.add_element(Op::CreateElement as u8);
        id.encode(&mut self.buf, self.id_size);
        element.encode(&mut self.buf);
    }

    pub fn create_element_ns(&mut self, element: impl IntoElement, ns: &str, id: impl IntoId) {
        self.check_id(&id);
        self.buf.add_element(Op::CreateElementNs as u8);
        id.encode(&mut self.buf, self.id_size);
        element.encode(&mut self.buf);
        encode_str(&mut self.buf, ns);
    }

    pub fn create_placeholder(&mut self, id: impl IntoId) {
        self.check_id(&id);
        self.buf.add_element(Op::CreatePlaceholder as u8);
        id.encode(&mut self.buf, self.id_size);
    }

    pub fn create_text_node(&mut self, text: &str, id: impl IntoId) {
        self.check_id(&id);
        self.buf.add_element(Op::CreateTextNode as u8);
        id.encode(&mut self.buf, self.id_size);
        encode_str(&mut self.buf, text);
    }

    pub fn set_attribute(
        &mut self,
        attribute: impl IntoAttribue,
        value: impl IntoValue,
        id: impl IntoId,
    ) {
        self.check_id(&id);
        self.buf.add_element(Op::SetAttribute as u8);
        id.encode(&mut self.buf, self.id_size);
        attribute.encode(&mut self.buf);
        value.encode(&mut self.buf);
    }

    pub fn remove_attribute(&mut self, attribute: impl IntoAttribue, id: impl IntoId) {
        self.check_id(&id);
        self.buf.add_element(Op::RemoveAttribute as u8);
        id.encode(&mut self.buf, self.id_size);
        attribute.encode(&mut self.buf);
    }

    pub fn remove_attribute_ns(&mut self, attribute: impl IntoAttribue, ns: &str, id: impl IntoId) {
        self.check_id(&id);
        self.buf.add_element(Op::RemoveAttributeNs as u8);
        id.encode(&mut self.buf, self.id_size);
        encode_str(&mut self.buf, ns);
    }

    pub fn append_children(&mut self, children: u8) {
        self.buf.add_element(Op::AppendChildren as u8);
        self.buf.add_element(children);
    }

    pub fn push_root(&mut self, id: impl IntoId) {
        self.check_id(&id);
        self.buf.add_element(Op::PushRoot as u8);
        id.encode(&mut self.buf, self.id_size);
    }

    pub fn pop_root(&mut self) {
        self.buf.add_element(Op::PopRoot as u8);
    }

    pub fn insert_after(&mut self, id: impl IntoId, num: u32) {
        self.check_id(&id);
        self.buf.add_element(Op::InsertAfter as u8);
        id.encode(&mut self.buf, self.id_size);
        self.buf.extend_slice(&num.to_le_bytes());
    }

    pub fn insert_before(&mut self, id: impl IntoId, num: u32) {
        self.check_id(&id);
        self.buf.add_element(Op::InsertBefore as u8);
        id.encode(&mut self.buf, self.id_size);
        self.buf.extend_slice(&num.to_le_bytes());
    }

    pub fn remove(&mut self, id: impl IntoId) {
        self.check_id(&id);
        self.buf.add_element(Op::Remove as u8);
        id.encode(&mut self.buf, self.id_size);
    }

    pub fn set_event_listener(&mut self, event: impl IntoEvent, id: impl IntoId) {
        self.check_id(&id);
        self.buf.add_element(Op::SetEventListener as u8);
        id.encode(&mut self.buf, self.id_size);
        event.encode(&mut self.buf);
    }

    pub fn remove_event_listener(&mut self, event: impl IntoEvent, id: impl IntoId) {
        self.check_id(&id);
        self.buf.add_element(Op::RemoveEventListener as u8);
        id.encode(&mut self.buf, self.id_size);
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

    pub fn replace_with(&mut self, id: impl IntoId, num: u32) {
        self.check_id(&id);
        self.buf.add_element(Op::ReplaceWith as u8);
        id.encode(&mut self.buf, self.id_size);
        self.buf.extend_slice(&num.to_le_bytes());
    }

    pub fn set_text(&mut self, id: impl IntoId, text: &str) {
        self.check_id(&id);
        self.buf.add_element(Op::SetText as u8);
        id.encode(&mut self.buf, self.id_size);
        encode_str(&mut self.buf, text);
    }

    pub fn create_template(&mut self, builder: impl ManyElements, id: impl IntoId) {
        self.check_id(&id);
        self.check_id_size(builder.max_id_size());
        self.buf.add_element(Op::CreateTemplate as u8);
        id.encode(&mut self.buf, self.id_size);
        builder.encode(&mut self.buf, self.id_size);
    }

    pub fn create_template_ref(&mut self, template_id: impl IntoId, node_id: impl IntoId) {
        self.check_id(&template_id);
        self.check_id(&node_id);
        self.buf.add_element(Op::CreateTemplateRef as u8);
        template_id.encode(&mut self.buf, self.id_size);
        node_id.encode(&mut self.buf, self.id_size);
    }

    pub fn build(&self) {
        work(self.buf.as_ref())
    }
}

pub trait IntoId {
    fn size(&self, id_size: u8) -> u8;
    fn max_el_size(&self) -> u8;
    fn encode<V: VecLike<Item = u8>>(self, v: &mut V, id_size: u8);
}

impl IntoId for u64 {
    fn size(&self, id_size: u8) -> u8 {
        id_size
    }

    fn max_el_size(&self) -> u8 {
        self.to_le_bytes().max_el_size()
    }

    fn encode<V: VecLike<Item = u8>>(self, v: &mut V, id_size: u8) {
        self.to_le_bytes().encode(v, id_size)
    }
}

impl IntoId for [u8; 8] {
    fn size(&self, id_size: u8) -> u8 {
        id_size
    }

    fn max_el_size(&self) -> u8 {
        let first_contentful_byte = self.iter().rev().position(|&b| b != 0).unwrap_or(8);
        (8 - first_contentful_byte) as u8
    }

    fn encode<V: VecLike<Item = u8>>(self, v: &mut V, id_size: u8) {
        v.add_element(1);
        v.extend_slice(&self[..id_size as usize]);
    }
}

impl IntoId for Option<u64> {
    fn size(&self, id_size: u8) -> u8 {
        self.map_or(1, |id| id.size(id_size))
    }

    fn max_el_size(&self) -> u8 {
        self.map(|id| id.max_el_size()).unwrap_or(0)
    }

    fn encode<V: VecLike<Item = u8>>(self, v: &mut V, id_size: u8) {
        if let Some(id) = self {
            id.encode(v, id_size);
        } else {
            v.add_element(0);
        }
    }
}

impl IntoId for Option<[u8; 8]> {
    fn size(&self, id_size: u8) -> u8 {
        self.map_or(1, |id| id.size(id_size))
    }

    fn max_el_size(&self) -> u8 {
        self.map(|id| id.max_el_size()).unwrap_or(0)
    }

    fn encode<V: VecLike<Item = u8>>(self, v: &mut V, id_size: u8) {
        if let Some(id) = self {
            id.encode(v, id_size);
        } else {
            v.add_element(0);
        }
    }
}

impl IntoId for (u64, u64) {
    fn size(&self, id_size: u8) -> u8 {
        self.0.size(id_size) + self.1.size(id_size)
    }

    fn max_el_size(&self) -> u8 {
        self.0.max_el_size().max(self.1.max_el_size())
    }

    fn encode<V: VecLike<Item = u8>>(self, v: &mut V, id_size: u8) {
        v.add_element(2);
        let (id1, id2) = (self.0.to_le_bytes(), self.1.to_le_bytes());
        v.extend_slice(&id1[..id_size as usize]);
        v.extend_slice(&id2[..id_size as usize]);
    }
}

pub fn encode_str<V: VecLike<Item = u8>>(buf: &mut V, s: &str) {
    let b = s.as_bytes();
    let len = b.len();
    buf.add_element(len as u8);
    buf.extend_slice(b);
}
