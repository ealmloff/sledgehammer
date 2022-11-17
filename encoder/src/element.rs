#![allow(non_camel_case_types)]

use crate::{attribute::AnyAttribute, batch::Batch, InNamespace, NodeId};

use self::sealed::Sealed;

mod sealed {
    use crate::{Element, InNamespace};

    pub trait Sealed {}

    impl Sealed for Element {}
    impl<'a> Sealed for &'a str {}
    impl<'a> Sealed for InNamespace<'a, Element> {}
    impl<'a, 'b> Sealed for InNamespace<'a, &'b str> {}
}

pub enum AnyElement<'a, 'b> {
    Element(Element),
    InNamespace(InNamespace<'a, Element>),
    Str(&'a str),
    InNamespaceStr(InNamespace<'a, &'b str>),
}

impl AnyElement<'_, '_> {
    pub fn encode(&self, v: &mut Batch) {
        match self {
            AnyElement::Element(a) => a.encode(v),
            AnyElement::InNamespace(a) => a.encode(v),
            AnyElement::Str(a) => a.encode(v),
            AnyElement::InNamespaceStr(a) => a.encode(v),
        }
    }

    pub(crate) unsafe fn encode_prealloc(&self, v: &mut Batch) {
        match self {
            AnyElement::Element(a) => a.encode_prealloc(v),
            AnyElement::InNamespace(a) => a.encode_prealloc(v),
            AnyElement::Str(a) => a.encode_prealloc(v),
            AnyElement::InNamespaceStr(a) => a.encode_prealloc(v),
        }
    }

    pub(crate) fn size(&self) -> usize {
        match self {
            AnyElement::Element(_) => 1,
            AnyElement::InNamespace(_) => 1 + 2,
            AnyElement::Str(_) => 2,
            AnyElement::InNamespaceStr(_) => 2 + 2,
        }
    }
}

/// Anything that can be turned into an element name
pub trait IntoElement<'a, 'b>: Sealed + Into<AnyElement<'a, 'b>> {
    /// If the element name can be encoded in a single byte
    const SINGLE_BYTE: bool = false;

    /// Encode the element into the message channel
    fn encode(&self, v: &mut Batch);

    /// Encode the element into the message channel with memory pre-allocated
    /// # Safety
    ///
    /// This is only safe if the batch is preallocated to the correct size
    unsafe fn encode_prealloc(&self, v: &mut Batch)
    where
        Self: Sized,
    {
        self.encode(v);
    }
}

impl<'a, 'b> Element {
    pub const fn any_element_const(self) -> AnyElement<'a, 'b> {
        AnyElement::Element(self)
    }
}

impl<'a, 'b> IntoElement<'a, 'b> for Element {
    const SINGLE_BYTE: bool = true;

    #[inline(always)]
    fn encode(&self, v: &mut Batch) {
        v.msg.push(*self as u8);
    }

    #[inline(always)]
    unsafe fn encode_prealloc(&self, v: &mut Batch)
    where
        Self: Sized,
    {
        unsafe {
            let ptr: *mut u8 = v.msg.as_mut_ptr();
            let old_len = v.msg.len();
            *ptr.add(old_len) = *self as u8;
            v.msg.set_len(old_len + 1);
        }
    }
}

impl<'a, 'b> From<Element> for AnyElement<'a, 'b> {
    fn from(e: Element) -> Self {
        AnyElement::Element(e)
    }
}

impl<'a, 'b> InNamespace<'a, Element> {
    /// Turn into an [`AnyElement`] in a const context
    pub const fn any_element_const(self) -> AnyElement<'a, 'b> {
        AnyElement::InNamespace(self)
    }
}

impl<'a, 'b> IntoElement<'a, 'b> for InNamespace<'a, Element> {
    fn encode(&self, v: &mut Batch) {
        v.msg.push(255);
        v.msg.push(self.0 as u8);
        v.encode_str(self.1);
    }
}

impl<'a, 'b> From<InNamespace<'a, Element>> for AnyElement<'a, 'b> {
    fn from(e: InNamespace<'a, Element>) -> Self {
        AnyElement::InNamespace(e)
    }
}

impl<'a, 'b> IntoElement<'a, 'b> for &'a str {
    fn encode(&self, v: &mut Batch) {
        v.msg.push(254);
        v.encode_str(*self);
    }
}

impl<'a, 'b> From<&'a str> for AnyElement<'a, 'b> {
    fn from(e: &'a str) -> Self {
        AnyElement::Str(e)
    }
}

impl<'a, 'b> IntoElement<'a, 'b> for InNamespace<'a, &'b str> {
    fn encode(&self, v: &mut Batch) {
        v.msg.push(253);
        v.encode_str(self.0);
        v.encode_str(self.1);
    }
}

impl<'a, 'b> From<InNamespace<'a, &'b str>> for AnyElement<'a, 'b> {
    fn from(e: InNamespace<'a, &'b str>) -> Self {
        AnyElement::InNamespaceStr(e)
    }
}

impl<'a, 'b> InNamespace<'a, &'b str> {
    pub const fn any_element_const(self) -> AnyElement<'a, 'b> {
        AnyElement::InNamespaceStr(self)
    }
}

/// A builder for any node
pub enum NodeBuilder<'a> {
    Text(TextBuilder<'a>),
    Element(ElementBuilder<'a>),
}

impl NodeBuilder<'_> {
    /// Encode the node into a batch
    pub(crate) fn encode(&self, v: &mut Batch) {
        match self {
            NodeBuilder::Text(t) => t.encode(v),
            NodeBuilder::Element(e) => e.encode(v),
        }
    }
}

impl<'a> From<TextBuilder<'a>> for NodeBuilder<'a> {
    fn from(t: TextBuilder<'a>) -> Self {
        NodeBuilder::Text(t)
    }
}

impl<'a> From<ElementBuilder<'a>> for NodeBuilder<'a> {
    fn from(e: ElementBuilder<'a>) -> Self {
        NodeBuilder::Element(e)
    }
}

/// A builder for an text node with a id, and text
pub struct TextBuilder<'a> {
    pub(crate) id: Option<NodeId>,
    pub(crate) text: &'a str,
}

impl<'a> TextBuilder<'a> {
    /// Create a new text builder
    pub const fn new(text: &'a str) -> Self {
        Self { id: None, text }
    }

    /// Set the id of the text node
    pub const fn id(mut self, id: NodeId) -> Self {
        self.id = Some(id);
        self
    }

    /// Encode the text node into a batch
    pub(crate) fn encode(&self, v: &mut Batch) {
        match self.id {
            Some(id) => {
                v.msg.push(3);
                v.encode_id(id);
            }
            None => {
                v.msg.push(2);
            }
        }
        v.encode_str(self.text);
    }
}

/// A builder for a element with an id, kind, attributes, and children
///
/// /// Example:
/// ```rust
/// let mut channel = MsgChannel::default();

/// // create an element using sledgehammer
/// channel.build_full_element(
///     ElementBuilder::new("div".into())
///         .id(NodeId(1))
///         .attrs(&[(Attribute::style.into(), "color: blue")])
///         .children(&[
///             ElementBuilder::new(Element::p.into())
///                 .into(),
///             TextBuilder::new("Hello from sledgehammer!").into(),
///         ]),
/// );
/// channel.flush();
/// ```
pub struct ElementBuilder<'a> {
    id: Option<NodeId>,
    kind: AnyElement<'a, 'a>,
    attrs: &'a [(AnyAttribute<'a, 'a>, &'a str)],
    children: &'a [NodeBuilder<'a>],
}

impl<'a> ElementBuilder<'a> {
    /// Create a new element builder
    pub const fn new(kind: AnyElement<'a, 'a>) -> Self {
        Self {
            id: None,
            kind,
            attrs: &[],
            children: &[],
        }
    }

    /// Set the id of the element
    pub const fn id(mut self, id: NodeId) -> Self {
        self.id = Some(id);
        self
    }

    /// Set the attributes of the element
    pub const fn attrs(mut self, attrs: &'a [(AnyAttribute<'a, 'a>, &'a str)]) -> Self {
        self.attrs = attrs;
        self
    }

    /// Set the children of the element
    pub const fn children(mut self, children: &'a [NodeBuilder<'a>]) -> Self {
        self.children = children;
        self
    }

    /// Encode the element into the a batch
    pub(crate) fn encode(&self, v: &mut Batch) {
        let size = 1
            + (self.id.is_some() as usize) * 4
            + self.kind.size()
            + 1
            + 1
            + self
                .attrs
                .iter()
                .map(|(k, _)| k.size_with_u8_discriminant() + 2)
                .sum::<usize>();
        v.msg.reserve(size);
        unsafe {
            match self.id {
                Some(id) => {
                    v.encode_u8_prealloc(1);
                    v.encode_id_prealloc(id);
                }
                None => {
                    v.encode_u8_prealloc(0);
                }
            }
            self.kind.encode_prealloc(v);
            // these are packed together so they can be read as a u16
            v.encode_u8_prealloc(self.attrs.len() as u8);
            v.encode_u8_prealloc(self.children.len() as u8);
            for (attr, value) in self.attrs {
                attr.encode_u8_discriminant_prealloc(v);
                v.encode_str_prealloc(*value);
            }
        }
        for child in self.children {
            child.encode(v);
        }
    }
}

macro_rules! elements {
    ($($i: ident),*) => {
        /// All built-in elements
        /// These are the element can be encoded with a single byte so they are more efficient (but less flexable) than a &str element
        #[allow(unused)]
        #[derive(Copy, Clone)]
        pub enum Element {
            $(
                $i
            ),*
        }

        pub struct NotElementError;

        impl std::str::FromStr for Element {
            type Err = NotElementError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(match s{
                    $(
                        stringify!($i) => Self::$i,
                    )*
                    _ => return Err(NotElementError)
                })
            }
        }
    };
}

elements! {
    a,
    abbr,
    acronym,
    address,
    applet,
    area,
    article,
    aside,
    audio,
    b,
    base,
    bdi,
    bdo,
    bgsound,
    big,
    blink,
    blockquote,
    body,
    br,
    button,
    canvas,
    caption,
    center,
    cite,
    code,
    col,
    colgroup,
    content,
    data,
    datalist,
    dd,
    del,
    details,
    dfn,
    dialog,
    dir,
    div,
    dl,
    dt,
    em,
    embed,
    fieldset,
    figcaption,
    figure,
    font,
    footer,
    form,
    frame,
    frameset,
    h1,
    head,
    header,
    hgroup,
    hr,
    html,
    i,
    iframe,
    image,
    img,
    input,
    ins,
    kbd,
    keygen,
    label,
    legend,
    li,
    link,
    main,
    map,
    mark,
    marquee,
    menu,
    menuitem,
    meta,
    meter,
    nav,
    nobr,
    noembed,
    noframes,
    noscript,
    object,
    ol,
    optgroup,
    option,
    output,
    p,
    param,
    picture,
    plaintext,
    portal,
    pre,
    progress,
    q,
    rb,
    rp,
    rt,
    rtc,
    ruby,
    s,
    samp,
    script,
    section,
    select,
    shadow,
    slot,
    small,
    source,
    spacer,
    span,
    strike,
    strong,
    style,
    sub,
    summary,
    sup,
    table,
    tbody,
    td,
    template,
    textarea,
    tfoot,
    th,
    thead,
    time,
    title,
    tr,
    track,
    tt,
    u,
    ul,
    var,
    video,
    wbr,
    xmp
}
