#![allow(non_camel_case_types)]

use crate::value::IntoValue;
use crate::MsgChannel;
use crate::attrs::Attribute;

/// Anything that can be turned into an attribute
pub trait IntoAttribue {
    /// If the attribute has a namespace
    const HAS_NS: bool;
    /// Encode the attribute into the message channel
    fn encode(self, v: &mut MsgChannel);
    /// Encode the attribute into the message channel with a u8 desciminant instead of bit packed bools
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

/// Something that can be turned into a list of attributes and values
#[allow(clippy::len_without_is_empty)]
pub trait ManyAttrs {
    /// The number of attribute value pairs
    fn len(&self) -> usize;
    /// Encode the attributes into the message channel
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
