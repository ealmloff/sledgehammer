#![allow(non_camel_case_types)]

use crate::builder::{encode_str, VecLike};
use crate::value::IntoValue;

pub trait IntoAttribue {
    fn size(&self) -> usize;
    fn encode<V: VecLike<Item = u8>>(self, v: &mut V);
}

impl IntoAttribue for Attribute {
    fn size(&self) -> usize {
        1
    }

    fn encode<V: VecLike<Item = u8>>(self, v: &mut V) {
        v.add_element(self as u8)
    }
}

impl<S: AsRef<str>> IntoAttribue for S {
    fn size(&self) -> usize {
        2 + self.as_ref().len()
    }

    fn encode<V: VecLike<Item = u8>>(self, v: &mut V) {
        v.add_element(255);
        encode_str(v, self.as_ref());
    }
}

pub trait ManyAttrs {
    fn len(&self) -> usize;
    fn size(&self) -> usize;
    fn encode<V: VecLike<Item = u8>>(self, v: &mut V);
}

impl ManyAttrs for () {
    fn len(&self) -> usize {
        0
    }

    fn encode<V: VecLike<Item = u8>>(self, v: &mut V) {
        v.add_element(<Self as ManyAttrs>::len(&self) as u8);
    }

    #[inline]
    fn size(&self) -> usize {
        0
    }
}

macro_rules! impl_many_attrs {
    ( $( (($t:ident, $i:ident):($v:ident, $m:ident)) ,)+ ) => {
        impl< $($t, $v),+ > ManyAttrs for ($(($t, $v),)+)
        where $($t: IntoAttribue, $v: IntoValue),+ {
            #[inline]
            fn size(&self) -> usize {
                let ($(($i, $m),)+) = self;
                0 $(+ $i.size() + $m.size())*
            }

            fn len(&self) -> usize {
                let ($(($i, _),)+) = self;
                0 $(+ 1 + $i.size()*0)*
            }

            fn encode<V: VecLike<Item = u8>>(self, v: &mut V) {
                v.add_element(self.len() as u8);
                let ($(($i, $m),)+) = self;
                $($i.encode(v);$m.encode(v);)+
            }
        }
    };
}

impl_many_attrs!(((T1, t1): (A1, a1)),);
impl_many_attrs!(((T1, t1): (A1, a1)), ((T2, t2): (A2, a2)),);
impl_many_attrs!(
    ((T1, t1): (A1, a1)),
    ((T2, t2): (A2, a2)),
    ((T3, t3): (A3, a3)),
);
impl_many_attrs!(
    ((T1, t1): (A1, a1)),
    ((T2, t2): (A2, a2)),
    ((T3, t3): (A3, a3)),
    ((T4, t4): (A4, a4)),
);
impl_many_attrs!(
    ((T1, t1): (A1, a1)),
    ((T2, t2): (A2, a2)),
    ((T3, t3): (A3, a3)),
    ((T4, t4): (A4, a4)),
    ((T5, t5): (A5, a5)),
);
impl_many_attrs!(
    ((T1, t1): (A1, a1)),
    ((T2, t2): (A2, a2)),
    ((T3, t3): (A3, a3)),
    ((T4, t4): (A4, a4)),
    ((T5, t5): (A5, a5)),
    ((T6, t6): (A6, a6)),
);
impl_many_attrs!(
    ((T1, t1): (A1, a1)),
    ((T2, t2): (A2, a2)),
    ((T3, t3): (A3, a3)),
    ((T4, t4): (A4, a4)),
    ((T5, t5): (A5, a5)),
    ((T6, t6): (A6, a6)),
    ((T7, t7): (A7, a7)),
);
impl_many_attrs!(
    ((T1, t1): (A1, a1)),
    ((T2, t2): (A2, a2)),
    ((T3, t3): (A3, a3)),
    ((T4, t4): (A4, a4)),
    ((T5, t5): (A5, a5)),
    ((T6, t6): (A6, a6)),
    ((T7, t7): (A7, a7)),
    ((T8, t8): (A8, a8)),
);
impl_many_attrs!(
    ((T1, t1): (A1, a1)),
    ((T2, t2): (A2, a2)),
    ((T3, t3): (A3, a3)),
    ((T4, t4): (A4, a4)),
    ((T5, t5): (A5, a5)),
    ((T6, t6): (A6, a6)),
    ((T7, t7): (A7, a7)),
    ((T8, t8): (A8, a8)),
    ((T9, t9): (A9, a9)),
);
impl_many_attrs!(
    ((T1, t1): (A1, a1)),
    ((T2, t2): (A2, a2)),
    ((T3, t3): (A3, a3)),
    ((T4, t4): (A4, a4)),
    ((T5, t5): (A5, a5)),
    ((T6, t6): (A6, a6)),
    ((T7, t7): (A7, a7)),
    ((T8, t8): (A8, a8)),
    ((T9, t9): (A9, a9)),
    ((T10, t10): (A10, a10)),
);
impl_many_attrs!(
    ((T1, t1): (A1, a1)),
    ((T2, t2): (A2, a2)),
    ((T3, t3): (A3, a3)),
    ((T4, t4): (A4, a4)),
    ((T5, t5): (A5, a5)),
    ((T6, t6): (A6, a6)),
    ((T7, t7): (A7, a7)),
    ((T8, t8): (A8, a8)),
    ((T9, t9): (A9, a9)),
    ((T10, t10): (A10, a10)),
    ((T11, t11): (A11, a11)),
);

pub enum Attribute {
    accept_charset,
    accept,
    accesskey,
    action,
    align,
    allow,
    alt,
    aria_atomic,
    aria_busy,
    aria_controls,
    aria_current,
    aria_describedby,
    aria_description,
    aria_details,
    aria_disabled,
    aria_dropeffect,
    aria_errormessage,
    aria_flowto,
    aria_grabbed,
    aria_haspopup,
    aria_hidden,
    aria_invalid,
    aria_keyshortcuts,
    aria_label,
    aria_labelledby,
    aria_live,
    aria_owns,
    aria_relevant,
    aria_roledescription,
    r#async,
    autocapitalize,
    autocomplete,
    autofocus,
    autoplay,
    background,
    bgcolor,
    border,
    buffered,
    capture,
    challenge,
    charset,
    checked,
    cite,
    class,
    code,
    codebase,
    color,
    cols,
    colspan,
    content,
    contenteditable,
    contextmenu,
    controls,
    coords,
    crossorigin,
    csp,
    data,
    datetime,
    decoding,
    default,
    defer,
    dir,
    dirname,
    disabled,
    download,
    draggable,
    enctype,
    enterkeyhint,
    r#for,
    form,
    formaction,
    formenctype,
    formmethod,
    formnovalidate,
    formtarget,
    headers,
    height,
    hidden,
    high,
    href,
    hreflang,
    http_equiv,
    icon,
    id,
    importance,
    inputmode,
    integrity,
    intrinsicsize,
    ismap,
    itemprop,
    keytype,
    kind,
    label,
    lang,
    language,
    list,
    loading,
    r#loop,
    low,
    manifest,
    max,
    maxlength,
    media,
    method,
    min,
    minlength,
    multiple,
    muted,
    name,
    novalidate,
    open,
    optimum,
    pattern,
    ping,
    placeholder,
    poster,
    preload,
    radiogroup,
    readonly,
    referrerpolicy,
    rel,
    required,
    reversed,
    role,
    rows,
    rowspan,
    sandbox,
    scope,
    scoped,
    selected,
    shape,
    size,
    sizes,
    slot,
    span,
    spellcheck,
    src,
    srcdoc,
    srclang,
    srcset,
    start,
    step,
    style,
    summary,
    tabindex,
    target,
    title,
    translate,
    r#type,
    usemap,
    value,
    width,
    wrap,
}
