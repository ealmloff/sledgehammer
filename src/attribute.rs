#![allow(non_camel_case_types)]

use self::sealed::Sealed;
use crate::{batch::Batch, InNamespace};

mod sealed {
    use crate::{Attribute, InNamespace};

    pub trait Sealed {}

    impl Sealed for Attribute {}
    impl<'a> Sealed for InNamespace<'a, Attribute> {}
    impl<'a> Sealed for &'a str {}
    impl<'a, 'b> Sealed for InNamespace<'b, &'a str> {}
}

pub enum AnyAttribute<'a, 'b> {
    Attribute(Attribute),
    InNamespace(InNamespace<'a, Attribute>),
    Str(&'a str),
    InNamespaceStr(InNamespace<'a, &'b str>),
}

impl AnyAttribute<'_, '_> {
    pub fn encode_u8_discriminant(&self, v: &mut Batch) {
        match self {
            AnyAttribute::Attribute(a) => a.encode_u8_discriminant(v),
            AnyAttribute::InNamespace(a) => a.encode_u8_discriminant(v),
            AnyAttribute::Str(a) => a.encode_u8_discriminant(v),
            AnyAttribute::InNamespaceStr(a) => a.encode_u8_discriminant(v),
        }
    }
}

/// Anything that can be turned into an attribute
pub trait IntoAttribue<'a, 'b>: Sealed {
    const SINGLE_BYTE: bool = false;

    /// Encode the attribute into the message channel
    fn encode(self, v: &mut Batch);

    /// Encode the attribute into the message channel with memory pre-allocated
    /// # Safety
    ///
    /// This is only safe if the batch is preallocated to the correct size
    unsafe fn encode_prealloc(self, v: &mut Batch)
    where
        Self: Sized,
    {
        self.encode(v);
    }
    /// Encode the attribute into the message channel with a u8 desciminant instead of bit packed bools
    fn encode_u8_discriminant(&self, v: &mut Batch);

    /// Turn into an [`AnyAttribute`]
    fn any_attr(self) -> AnyAttribute<'a, 'b>;
}

impl<'a, 'b> Attribute {
    pub const fn any_attr_const(self) -> AnyAttribute<'a, 'b> {
        AnyAttribute::Attribute(self)
    }
}

impl<'a, 'b> IntoAttribue<'a, 'b> for Attribute {
    const SINGLE_BYTE: bool = true;

    #[inline(always)]
    fn encode(self, v: &mut Batch) {
        v.encode_bool(false);
        v.encode_bool(false);
        v.msg.push(self as u8);
    }

    #[inline(always)]
    unsafe fn encode_prealloc(self, v: &mut Batch) {
        v.encode_bool(false);
        v.encode_bool(false);
        unsafe {
            let ptr: *mut u8 = v.msg.as_mut_ptr();
            *ptr.add(v.msg.len()) = self as u8;
            v.msg.set_len(v.msg.len() + 1);
        }
    }

    fn encode_u8_discriminant(&self, v: &mut Batch) {
        v.msg.push(*self as u8)
    }

    fn any_attr(self) -> AnyAttribute<'a, 'b> {
        AnyAttribute::Attribute(self)
    }
}

impl<'a, 'b> InNamespace<'a, Attribute> {
    pub const fn any_attr_const(self) -> AnyAttribute<'a, 'b> {
        AnyAttribute::InNamespace(self)
    }
}

impl<'a, 'b> IntoAttribue<'a, 'b> for InNamespace<'a, Attribute> {
    fn encode(self, v: &mut Batch) {
        v.encode_bool(false);
        v.msg.push(self.0 as u8);
        v.encode_bool(true);
        v.encode_str(self.1);
    }

    fn encode_u8_discriminant(&self, v: &mut Batch) {
        v.msg.push(255);
        v.msg.push(self.0 as u8);
        v.encode_str(self.1);
    }

    fn any_attr(self) -> AnyAttribute<'a, 'b> {
        AnyAttribute::InNamespace(self)
    }
}

impl<'a, 'b> IntoAttribue<'a, 'b> for &'a str {
    fn encode(self, v: &mut Batch) {
        v.encode_bool(true);
        v.encode_cachable_str(self);
        v.encode_bool(false);
    }

    fn encode_u8_discriminant(&self, v: &mut Batch) {
        v.msg.push(254);
        v.encode_cachable_str(*self);
    }

    fn any_attr(self) -> AnyAttribute<'a, 'b> {
        AnyAttribute::Str(self)
    }
}

impl<'a, 'b> InNamespace<'a, &'b str> {
    pub const fn any_attr_const(self) -> AnyAttribute<'a, 'b> {
        AnyAttribute::InNamespaceStr(self)
    }
}

impl<'a, 'b> IntoAttribue<'a, 'b> for InNamespace<'a, &'b str> {
    fn encode(self, v: &mut Batch) {
        v.encode_bool(true);
        v.encode_cachable_str(self.0);
        v.encode_bool(true);
        v.encode_cachable_str(self.1);
    }

    fn encode_u8_discriminant(&self, v: &mut Batch) {
        v.msg.push(253);
        v.encode_cachable_str(self.0);
        v.encode_cachable_str(self.1);
    }

    fn any_attr(self) -> AnyAttribute<'a, 'b> {
        AnyAttribute::InNamespaceStr(self)
    }
}

#[derive(Copy, Clone)]
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
