#![allow(non_camel_case_types)]

use crate::{attribute::ManyAttrs, builder::MsgChannel, InNamespace, NodeId};

use self::sealed::Sealed;

mod sealed {
    use crate::{Element, InNamespace};

    pub trait Sealed {}

    impl Sealed for Element {}
    impl<'a> Sealed for &'a str {}
    impl<'a> Sealed for InNamespace<'a, Element> {}
    impl<'a, 'b> Sealed for InNamespace<'a, &'b str> {}
}

/// Anything that can be turned into an element name
pub trait IntoElement: Sealed {
    fn encode(self, v: &mut MsgChannel);
}

impl IntoElement for Element {
    fn encode(self, v: &mut MsgChannel) {
        v.msg.push(self as u8);
    }
}

impl<'a> IntoElement for InNamespace<'a, Element> {
    fn encode(self, v: &mut MsgChannel) {
        v.msg.push(255);
        v.msg.push(self.0 as u8);
        v.encode_str(self.1);
    }
}

impl<'a> IntoElement for &'a str {
    fn encode(self, v: &mut MsgChannel) {
        v.msg.push(254);
        v.encode_str(self);
    }
}

impl<'a, 'b> IntoElement for InNamespace<'a, &'b str> {
    fn encode(self, v: &mut MsgChannel) {
        v.msg.push(253);
        v.encode_str(self.0);
        v.encode_str(self.1);
    }
}

/// Something that can be turned into a list of elements
#[allow(clippy::len_without_is_empty)]
pub trait ManyElements: sealed_many_elements::Sealed {
    fn len(&self) -> usize;
    fn encode(self, v: &mut MsgChannel);
}

impl ManyElements for () {
    fn len(&self) -> usize {
        0
    }

    fn encode(self, v: &mut MsgChannel) {
        v.msg.push(<Self as ManyElements>::len(&self) as u8);
    }
}

macro_rules! impl_many_elements {
    (
        $(
            ( $( ($t:ident, $i:ident) ),+ )$l:literal
        )+
    ) => {
        mod sealed_many_elements {
            use super::*;

            pub trait Sealed {}

            impl Sealed for () {}
            $(
                impl< $($t),+ > Sealed for ($($t,)+)
                    where $($t: ElementBuilderExt),+ {

                }
            )+
        }
        $(
            impl< $($t),+ > ManyElements for ($($t,)+)
                where $($t: ElementBuilderExt),+ {
                fn len(&self) -> usize {
                    $l
                }

                fn encode(self, v: &mut MsgChannel) {
                    v.msg.push(self.len() as u8);
                    let ($($i,)+) = self;
                    $($i.encode(v);)+
                }
            }
        )+
    };
}

impl_many_elements!(((T1, t1))1
    ((T1, t1), (T2, t2))2
    ((T1, t1), (T2, t2), (T3, t3))3
    ((T1, t1), (T2, t2), (T3, t3), (T4, t4))4
    ((T1, t1), (T2, t2), (T3, t3), (T4, t4), (T5, t5))5
    ((T1, t1), (T2, t2), (T3, t3), (T4, t4), (T5, t5), (T6, t6))6
    (
        (T1, t1),
        (T2, t2),
        (T3, t3),
        (T4, t4),
        (T5, t5),
        (T6, t6),
        (T7, t7)
    )7
    (
        (T1, t1),
        (T2, t2),
        (T3, t3),
        (T4, t4),
        (T5, t5),
        (T6, t6),
        (T7, t7),
        (T8, t8)
    )8
    (
        (T1, t1),
        (T2, t2),
        (T3, t3),
        (T4, t4),
        (T5, t5),
        (T6, t6),
        (T7, t7),
        (T8, t8),
        (T9, t9)
    )9
    (
        (T1, t1),
        (T2, t2),
        (T3, t3),
        (T4, t4),
        (T5, t5),
        (T6, t6),
        (T7, t7),
        (T8, t8),
        (T9, t9),
        (T10, t10)
    )10
    (
        (T1, t1),
        (T2, t2),
        (T3, t3),
        (T4, t4),
        (T5, t5),
        (T6, t6),
        (T7, t7),
        (T8, t8),
        (T9, t9),
        (T10, t10),
        (T11, t11)
    )11
);

/// A builder for a element with an id, kind, attributes, and children
pub struct ElementBuilder<K: IntoElement, A: ManyAttrs, E: ManyElements> {
    id: Option<NodeId>,
    kind: K,
    attrs: A,
    children: E,
}

impl<K: IntoElement, A: ManyAttrs, E: ManyElements> ElementBuilder<K, A, E> {
    pub const fn new(id: Option<NodeId>, kind: K, attrs: A, children: E) -> Self {
        Self {
            id,
            kind,
            attrs,
            children,
        }
    }
}

/// Extra functions for element builders
pub trait ElementBuilderExt: sealed_element_builder::Sealed {
    fn encode(self, v: &mut MsgChannel);
}

mod sealed_element_builder {
    use super::*;

    pub trait Sealed {}

    impl<K: IntoElement, A: ManyAttrs, E: ManyElements> Sealed for ElementBuilder<K, A, E> {}
}

impl<K: IntoElement, A: ManyAttrs, E: ManyElements> ElementBuilderExt for ElementBuilder<K, A, E> {
    fn encode(self, v: &mut MsgChannel) {
        v.encode_optional_id_with_byte_bool(self.id);
        self.kind.encode(v);
        self.attrs.encode(v);
        self.children.encode(v);
    }
}

/// All built-in elements
#[allow(unused)]
pub enum Element {
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
    xmp,
}
