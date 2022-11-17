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

#[derive(Clone, Copy)]
pub enum AnyAttribute<'a, 'b> {
    Attribute(Attribute),
    InNamespace(InNamespace<'a, Attribute>),
    Str(&'a str),
    InNamespaceStr(InNamespace<'a, &'b str>),
}

impl AnyAttribute<'_, '_> {
    pub(crate) unsafe fn encode_u8_discriminant_prealloc(self, v: &mut Batch) {
        match self {
            AnyAttribute::Attribute(a) => a.encode_u8_discriminant_prealloc(v),
            AnyAttribute::InNamespace(a) => a.encode_u8_discriminant_prealloc(v),
            AnyAttribute::Str(a) => a.encode_u8_discriminant_prealloc(v),
            AnyAttribute::InNamespaceStr(a) => a.encode_u8_discriminant_prealloc(v),
        }
    }

    pub(crate) fn size_with_u8_discriminant(&self) -> usize {
        match self {
            AnyAttribute::Attribute(_) => 1,
            AnyAttribute::InNamespace(_) => 1 + 1 + 2,
            AnyAttribute::Str(_) => 1 + 2,
            AnyAttribute::InNamespaceStr(_) => 1 + 2 + 2,
        }
    }
}

/// Anything that can be turned into an attribute
pub trait IntoAttribue<'a, 'b>: Sealed + Into<AnyAttribute<'a, 'b>> {
    /// If the attribute can be encoded in a single byte
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
    unsafe fn encode_u8_discriminant_prealloc(self, v: &mut Batch);
}

impl<'a, 'b> Attribute {
    /// Turn into an [`AnyAttribute`] in a const context
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

    #[inline(always)]
    unsafe fn encode_u8_discriminant_prealloc(self, v: &mut Batch) {
        v.encode_u8_prealloc(self as u8)
    }
}

impl<'a, 'b> From<Attribute> for AnyAttribute<'a, 'b> {
    fn from(a: Attribute) -> Self {
        AnyAttribute::Attribute(a)
    }
}

impl<'a, 'b> InNamespace<'a, Attribute> {
    pub const fn any_attr_const(self) -> AnyAttribute<'a, 'b> {
        AnyAttribute::InNamespace(self)
    }
}

impl<'a, 'b> IntoAttribue<'a, 'b> for InNamespace<'a, Attribute> {
    #[inline(always)]
    fn encode(self, v: &mut Batch) {
        v.encode_bool(false);
        v.msg.push(self.0 as u8);
        v.encode_bool(true);
        v.encode_str(self.1);
    }

    #[inline(always)]
    unsafe fn encode_u8_discriminant_prealloc(self, v: &mut Batch) {
        v.encode_u8_prealloc(255);
        v.encode_u8_prealloc(self.0 as u8);
        v.encode_str_prealloc(self.1);
    }
}

impl<'a, 'b> From<InNamespace<'a, Attribute>> for AnyAttribute<'a, 'b> {
    fn from(a: InNamespace<'a, Attribute>) -> Self {
        AnyAttribute::InNamespace(a)
    }
}

impl<'a, 'b> IntoAttribue<'a, 'b> for &'a str {
    fn encode(self, v: &mut Batch) {
        v.encode_bool(true);
        v.encode_cachable_str(self);
        v.encode_bool(false);
    }

    unsafe fn encode_u8_discriminant_prealloc(self, v: &mut Batch) {
        v.encode_u8_prealloc(254);
        v.encode_str_prealloc(self);
    }
}

impl<'a, 'b> From<&'a str> for AnyAttribute<'a, 'b> {
    fn from(a: &'a str) -> Self {
        AnyAttribute::Str(a)
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

    unsafe fn encode_u8_discriminant_prealloc(self, v: &mut Batch) {
        v.encode_u8_prealloc(253);
        v.encode_str_prealloc(self.0);
        v.encode_str_prealloc(self.1);
    }
}

impl<'a, 'b> From<InNamespace<'a, &'b str>> for AnyAttribute<'a, 'b> {
    fn from(a: InNamespace<'a, &'b str>) -> Self {
        AnyAttribute::InNamespaceStr(a)
    }
}

macro_rules! attributes {
    ($($i: ident),*) => {
        /// All built-in attributes
        /// These are the attributes can be encoded with a single byte so they are more efficient (but less flexable) than a &str attribute
        #[derive(Copy, Clone)]
        pub enum Attribute {
            $(
                $i
            ),*
        }

        pub struct NotElementError;

        impl std::str::FromStr for Attribute {
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

attributes! {
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
    wrap
}
