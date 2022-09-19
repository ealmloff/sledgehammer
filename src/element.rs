#![allow(non_camel_case_types)]

use std::ops::RangeInclusive;

use crate::{
    attribute::ManyAttrs,
    builder::{encode_str, VecLike},
};

pub trait IntoElement {
    const LEN: RangeInclusive<Option<usize>>;

    fn size(&self) -> usize;
    fn encode<V: VecLike<Item = u8>>(self, v: &mut V);
}

impl IntoElement for Element {
    const LEN: RangeInclusive<Option<usize>> = Some(1)..=Some(1);

    fn size(&self) -> usize {
        1
    }

    fn encode<V: VecLike<Item = u8>>(self, v: &mut V) {
        v.add_element(self as u8)
    }
}

impl<S: AsRef<str>> IntoElement for S {
    const LEN: RangeInclusive<Option<usize>> = Some(2)..=None;

    fn size(&self) -> usize {
        self.as_ref().len() + 2
    }

    fn encode<V: VecLike<Item = u8>>(self, v: &mut V) {
        v.add_element(255);
        encode_str(v, self.as_ref());
    }
}

pub trait ManyElements {
    fn size(&self) -> usize;
    fn len(&self) -> usize;
    fn encode<V: VecLike<Item = u8>>(self, v: &mut V);
}

impl ManyElements for () {
    fn len(&self) -> usize {
        0
    }

    fn size(&self) -> usize {
        0
    }

    fn encode<V: VecLike<Item = u8>>(self, _: &mut V) {}
}

macro_rules! impl_many_elements {
    (( $( ($t:ident, $i:ident) ),+ )) => {
        impl< $($t),+ > ManyElements for ($($t,)+)
            where $($t: ElementBuilderExt),+ {
            fn size(&self) -> usize {
                let ($($i,)+) = self;
                0 $(+ $i.size())*
            }

            fn len(&self) -> usize {
                let ($($i,)+) = self;
                0 $(+ 1+ 0*$i.size())*
            }

            fn encode<V: VecLike<Item = u8>>(self, v: &mut V) {
                let ($($i,)+) = self;
                $($i.encode(v);)+
            }
        }
    };
}

impl_many_elements!(((T1, t1)));
impl_many_elements!(((T1, t1), (T2, t2)));
impl_many_elements!(((T1, t1), (T2, t2), (T3, t3)));
impl_many_elements!(((T1, t1), (T2, t2), (T3, t3), (T4, t4)));
impl_many_elements!(((T1, t1), (T2, t2), (T3, t3), (T4, t4), (T5, t5)));
impl_many_elements!(((T1, t1), (T2, t2), (T3, t3), (T4, t4), (T5, t5), (T6, t6)));
impl_many_elements!((
    (T1, t1),
    (T2, t2),
    (T3, t3),
    (T4, t4),
    (T5, t5),
    (T6, t6),
    (T7, t7)
));
impl_many_elements!((
    (T1, t1),
    (T2, t2),
    (T3, t3),
    (T4, t4),
    (T5, t5),
    (T6, t6),
    (T7, t7),
    (T8, t8)
));
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
));
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
));
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
));

pub struct ElementBuilder<K: IntoElement, A: ManyAttrs, E: ManyElements> {
    kind: K,
    attrs: A,
    children: E,
}

impl<K: IntoElement, A: ManyAttrs, E: ManyElements> ElementBuilder<K, A, E> {
    pub const fn new(kind: K, attrs: A, children: E) -> Self {
        Self {
            kind,
            attrs,
            children,
        }
    }
}

pub trait ElementBuilderExt {
    fn size(&self) -> usize;
    fn encode<V: VecLike<Item = u8>>(self, v: &mut V);
}

impl<K: IntoElement, A: ManyAttrs, E: ManyElements> ElementBuilderExt for ElementBuilder<K, A, E> {
    fn size(&self) -> usize {
        2 + self.kind.size() + self.attrs.size() + self.children.size()
    }

    fn encode<V: VecLike<Item = u8>>(self, v: &mut V) {
        self.kind.encode(v);
        v.add_element(self.attrs.len() as u8);
        self.attrs.encode(v);
        v.add_element(self.children.len() as u8);
        self.children.encode(v);
    }
}

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
    head,
    header,
    h1,
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
