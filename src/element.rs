#![allow(non_camel_case_types)]

use std::ops::RangeInclusive;

use crate::{attribute::ManyAttrs, builder::MsgChannel, MaybeId};

pub trait IntoElement {
    const LEN: RangeInclusive<Option<usize>>;

    fn size(&self) -> usize;
    fn encode(self, v: &mut MsgChannel);
}

impl IntoElement for Element {
    const LEN: RangeInclusive<Option<usize>> = Some(1)..=Some(1);

    fn size(&self) -> usize {
        1
    }

    fn encode(self, v: &mut MsgChannel) {
        v.msg.push(self as u8)
    }
}

impl<S: AsRef<str>> IntoElement for S {
    const LEN: RangeInclusive<Option<usize>> = Some(2)..=None;

    fn size(&self) -> usize {
        self.as_ref().len() + 2
    }

    fn encode(self, v: &mut MsgChannel) {
        v.msg.push(255);
        v.encode_str(self.as_ref());
    }
}

#[allow(clippy::len_without_is_empty)]
pub trait ManyElements {
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
    (( $( ($t:ident, $i:ident) ),+ )$l:literal) => {
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
    };
}

impl_many_elements!(((T1, t1))1);
impl_many_elements!(((T1, t1), (T2, t2))2);
impl_many_elements!(((T1, t1), (T2, t2), (T3, t3))3);
impl_many_elements!(((T1, t1), (T2, t2), (T3, t3), (T4, t4))4);
impl_many_elements!(((T1, t1), (T2, t2), (T3, t3), (T4, t4), (T5, t5))5);
impl_many_elements!(((T1, t1), (T2, t2), (T3, t3), (T4, t4), (T5, t5), (T6, t6))6);
impl_many_elements!((
    (T1, t1),
    (T2, t2),
    (T3, t3),
    (T4, t4),
    (T5, t5),
    (T6, t6),
    (T7, t7)
)7);
impl_many_elements!((
    (T1, t1),
    (T2, t2),
    (T3, t3),
    (T4, t4),
    (T5, t5),
    (T6, t6),
    (T7, t7),
    (T8, t8)
)8);
impl_many_elements!((
    (T1, t1),
    (T2, t2),
    (T3, t3),
    (T4, t4),
    (T5, t5),
    (T6, t6),
    (T7, t7),
    (T8, t8),
    (T9, t9)
)9);
impl_many_elements!((
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
)10);
impl_many_elements!((
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
)11);

pub struct ElementBuilder<K: IntoElement, A: ManyAttrs, E: ManyElements> {
    id: MaybeId,
    kind: K,
    attrs: A,
    children: E,
}

impl<K: IntoElement, A: ManyAttrs, E: ManyElements> ElementBuilder<K, A, E> {
    pub const fn new(id: MaybeId, kind: K, attrs: A, children: E) -> Self {
        Self {
            id,
            kind,
            attrs,
            children,
        }
    }
}

pub trait ElementBuilderExt {
    fn encode(self, v: &mut MsgChannel);
}

impl<K: IntoElement, A: ManyAttrs, E: ManyElements> ElementBuilderExt for ElementBuilder<K, A, E> {
    fn encode(self, v: &mut MsgChannel) {
        v.encode_maybe_id_with_byte_bool(self.id);
        self.kind.encode(v);
        self.attrs.encode(v);
        self.children.encode(v);
    }
}

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
