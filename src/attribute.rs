#![allow(non_camel_case_types)]

use crate::value::IntoValue;
use crate::MsgChannel;

pub trait IntoAttribue {
    const HAS_NS: bool;
    fn encode(self, v: &mut MsgChannel);
    fn encode_u8_discriminant(self, v: &mut MsgChannel);
}

impl IntoAttribue for Attribute {
    const HAS_NS: bool = false;
    fn encode(self, v: &mut MsgChannel) {
        v.encode_bool(false);
        v.msg.push(self as u8)
    }
    fn encode_u8_discriminant(self, v: &mut MsgChannel) {
        v.msg.push(self as u8)
    }
}

impl<S: AsRef<str>> IntoAttribue for S {
    const HAS_NS: bool = false;
    fn encode(self, v: &mut MsgChannel) {
        v.encode_bool(true);
        v.encode_cachable_str(self.as_ref());
    }
    fn encode_u8_discriminant(self, v: &mut MsgChannel) {
        v.msg.push(255);
        v.encode_cachable_str(self.as_ref());
    }
}

#[allow(clippy::len_without_is_empty)]
pub trait ManyAttrs {
    fn len(&self) -> usize;
    fn encode(self, v: &mut MsgChannel);
}

impl ManyAttrs for () {
    fn len(&self) -> usize {
        0
    }

    fn encode(self, v: &mut MsgChannel) {
        v.msg.push(<Self as ManyAttrs>::len(&self) as u8);
    }
}

macro_rules! impl_many_attrs {
    ( $( (($t:ident, $i:ident):($v:ident, $m:ident)) ,)+:$l:literal ) => {
        impl< $($t, $v),+ > ManyAttrs for ($(($t, $v),)+)
        where $($t: IntoAttribue, $v: IntoValue),+ {
            fn len(&self) -> usize {
                $l
            }

            fn encode(self, v: &mut MsgChannel) {
                v.msg.push(self.len() as u8);
                let ($(($i, $m),)+) = self;
                $($i.encode_u8_discriminant(v);$m.encode(v);)+
            }
        }
    };
}

impl_many_attrs!(((T1, t1): (A1, a1)),:1);
impl_many_attrs!(((T1, t1): (A1, a1)), ((T2, t2): (A2, a2)),:2);
impl_many_attrs!(
    ((T1, t1): (A1, a1)),
    ((T2, t2): (A2, a2)),
    ((T3, t3): (A3, a3)),:3
);
impl_many_attrs!(
    ((T1, t1): (A1, a1)),
    ((T2, t2): (A2, a2)),
    ((T3, t3): (A3, a3)),
    ((T4, t4): (A4, a4)),:4
);
impl_many_attrs!(
    ((T1, t1): (A1, a1)),
    ((T2, t2): (A2, a2)),
    ((T3, t3): (A3, a3)),
    ((T4, t4): (A4, a4)),
    ((T5, t5): (A5, a5)),:5
);
impl_many_attrs!(
    ((T1, t1): (A1, a1)),
    ((T2, t2): (A2, a2)),
    ((T3, t3): (A3, a3)),
    ((T4, t4): (A4, a4)),
    ((T5, t5): (A5, a5)),
    ((T6, t6): (A6, a6)),:6
);
impl_many_attrs!(
    ((T1, t1): (A1, a1)),
    ((T2, t2): (A2, a2)),
    ((T3, t3): (A3, a3)),
    ((T4, t4): (A4, a4)),
    ((T5, t5): (A5, a5)),
    ((T6, t6): (A6, a6)),
    ((T7, t7): (A7, a7)),:7
);
impl_many_attrs!(
    ((T1, t1): (A1, a1)),
    ((T2, t2): (A2, a2)),
    ((T3, t3): (A3, a3)),
    ((T4, t4): (A4, a4)),
    ((T5, t5): (A5, a5)),
    ((T6, t6): (A6, a6)),
    ((T7, t7): (A7, a7)),
    ((T8, t8): (A8, a8)),:8
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
    ((T9, t9): (A9, a9)),:9
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
    ((T10, t10): (A10, a10)),:10
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
    ((T11, t11): (A11, a11)),:11
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
