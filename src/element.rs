#![allow(non_camel_case_types)]

use std::ops::RangeInclusive;

use crate::{
    attribute::ManyAttrs,
    builder::{MsgBuilder, VecLike},
    id_size,
};

pub trait IntoElement {
    const LEN: RangeInclusive<Option<usize>>;

    fn size(&self) -> usize;
    fn encode<V: VecLike<u8>>(self, v: &mut MsgBuilder<V>);
}

impl IntoElement for Element {
    const LEN: RangeInclusive<Option<usize>> = Some(1)..=Some(1);

    fn size(&self) -> usize {
        1
    }

    fn encode<V: VecLike<u8>>(self, v: &mut MsgBuilder<V>) {
        v.msg.add_element(self as u8)
    }
}

impl<S: AsRef<str>> IntoElement for S {
    const LEN: RangeInclusive<Option<usize>> = Some(2)..=None;

    fn size(&self) -> usize {
        self.as_ref().len() + 2
    }

    fn encode<V: VecLike<u8>>(self, v: &mut MsgBuilder<V>) {
        v.msg.add_element(255);
        v.encode_str(self.as_ref());
    }
}

#[allow(clippy::len_without_is_empty)]
pub trait ManyElements {
    fn size(&self, id_size: u8) -> usize;
    fn len(&self) -> usize;
    fn encode<V: VecLike<u8>>(self, v: &mut MsgBuilder<V>, id_size: u8);
    fn max_id_size(&self) -> u8;
}

impl ManyElements for () {
    fn len(&self) -> usize {
        0
    }

    fn size(&self, _: u8) -> usize {
        0
    }

    fn encode<V: VecLike<u8>>(self, v: &mut MsgBuilder<V>, _: u8) {
        v.msg.add_element(<Self as ManyElements>::len(&self) as u8);
    }

    fn max_id_size(&self) -> u8 {
        0
    }
}

macro_rules! impl_many_elements {
    (( $( ($t:ident, $i:ident) ),+ )) => {
        impl< $($t),+ > ManyElements for ($($t,)+)
            where $($t: ElementBuilderExt),+ {
            fn size(&self, id_size: u8) -> usize {
                let ($($i,)+) = self;
                0 $(+ $i.size(id_size))*
            }

            fn len(&self) -> usize {
                let ($($i,)+) = self;
                0 $(+ 1+ 0*$i.size(0))*
            }

            fn encode<V: VecLike<u8>>(self, v: &mut MsgBuilder<V>, id_size: u8) {
                v.msg.add_element(self.len() as u8);
                let ($($i,)+) = self;
                $($i.encode(v, id_size);)+
            }

            fn max_id_size(&self) -> u8 {
                let ($($i,)+) = self;
                [$($i.max_id_size(),)*].iter().max().copied().unwrap_or_default()
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
    id: Option<[u8; 8]>,
    kind: K,
    attrs: A,
    children: E,
}

impl<K: IntoElement, A: ManyAttrs, E: ManyElements> ElementBuilder<K, A, E> {
    pub const fn new(id: Option<u64>, kind: K, attrs: A, children: E) -> Self {
        Self {
            id: if let Some(id) = id {
                Some(id.to_le_bytes())
            } else {
                None
            },
            kind,
            attrs,
            children,
        }
    }
}

pub trait ElementBuilderExt {
    fn size(&self, id_size: u8) -> usize;
    fn encode<V: VecLike<u8>>(self, v: &mut MsgBuilder<V>, id_size: u8);
    fn max_id_size(&self) -> u8;
}

impl<K: IntoElement, A: ManyAttrs, E: ManyElements> ElementBuilderExt for ElementBuilder<K, A, E> {
    fn size(&self, id_size: u8) -> usize {
        2 + self.kind.size()
            + self.attrs.size()
            + self.children.size(id_size)
            + if self.id.is_some() { id_size } else { 1 } as usize
    }

    fn encode<V: VecLike<u8>>(self, v: &mut MsgBuilder<V>, id_size: u8) {
        v.encode_maybe_id(self.id);
        self.kind.encode(v);
        self.attrs.encode(v);
        self.children.encode(v, id_size);
    }

    fn max_id_size(&self) -> u8 {
        if let Some(id) = self.id {
            id_size(id)
        } else {
            1
        }
        .max(self.children.max_id_size())
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
