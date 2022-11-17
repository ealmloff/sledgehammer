pub mod attribute;
pub mod batch;
pub mod element;

use std::{fmt::Arguments, io::Write};

pub use attribute::{Attribute, IntoAttribue};
pub use batch::Op;
pub use element::{Element, ElementBuilder, IntoElement, NodeBuilder, TextBuilder};

/// Something that lives in a namespace like a tag or attribute
#[derive(Clone, Copy)]
pub struct InNamespace<'a, T>(pub T, pub &'a str);

/// Something that can live in a namespace
pub trait WithNsExt {
    /// Moves the item into a namespace
    fn in_namespace(self, namespace: &str) -> InNamespace<Self>
    where
        Self: Sized,
    {
        InNamespace(self, namespace)
    }
}

impl WithNsExt for Element {}
impl WithNsExt for Attribute {}
impl<'a> WithNsExt for &'a str {}

/// An id that may be either the last node or a node with an assigned id.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MaybeId {
    /// The last node that was created or navigated to.
    LastNode,
    /// A node that was created and stored with an id
    Node(NodeId),
}

impl MaybeId {
    #[inline(always)]
    pub(crate) const fn encoded_size(&self) -> u8 {
        match self {
            MaybeId::LastNode => 0,
            MaybeId::Node(_) => 4,
        }
    }
}

/// A node that was created and stored with an id
/// It is recommended to create and store ids with a slab allocator with an exposed slab index for example the excellent [slab](https://docs.rs/slab) crate.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(pub u32);

/// Something that can be written as a utf-8 string to a buffer
pub trait WritableText {
    fn write_as_text(self, to: &mut Vec<u8>);
}

impl WritableText for char {
    fn write_as_text(self, to: &mut Vec<u8>) {
        to.push(self as u8);
    }
}

impl<'a> WritableText for &'a str {
    #[inline(always)]
    fn write_as_text(self, to: &mut Vec<u8>) {
        let len = self.len();
        to.reserve(len);
        let old_len = to.len();
        #[allow(clippy::uninit_vec)]
        unsafe {
            let ptr = to.as_mut_ptr();
            let bytes = self.as_bytes();
            let str_ptr = bytes.as_ptr();
            for o in 0..len {
                *ptr.add(old_len + o) = *str_ptr.add(o);
            }
            to.set_len(old_len + len);
        }
        // let _ = to.write(self.as_bytes());
    }
}

impl WritableText for Arguments<'_> {
    fn write_as_text(self, to: &mut Vec<u8>) {
        let _ = to.write_fmt(self);
    }
}

impl<F> WritableText for F
where
    F: FnOnce(&mut Vec<u8>),
{
    fn write_as_text(self, to: &mut Vec<u8>) {
        self(to);
    }
}

macro_rules! write_unsized {
    ($t: ty) => {
        impl WritableText for $t {
            fn write_as_text(self, to: &mut Vec<u8>) {
                let mut n = self;
                let mut n2 = n;
                let mut num_digits = 0;
                while n2 > 0 {
                    n2 /= 10;
                    num_digits += 1;
                }
                let len = num_digits;
                to.reserve(len);
                let ptr = to.as_mut_ptr().cast::<u8>();
                let old_len = to.len();
                let mut i = len - 1;
                loop {
                    unsafe { ptr.add(old_len + i).write((n % 10) as u8 + b'0') }
                    n /= 10;

                    if n == 0 {
                        break;
                    } else {
                        i -= 1;
                    }
                }

                #[allow(clippy::uninit_vec)]
                unsafe {
                    to.set_len(old_len + (len - i));
                }
            }
        }
    };
}

macro_rules! write_sized {
    ($t: ty) => {
        impl WritableText for $t {
            fn write_as_text(self, to: &mut Vec<u8>) {
                let neg = self < 0;
                let mut n = if neg {
                    match self.checked_abs() {
                        Some(n) => n,
                        None => <$t>::MAX / 2 + 1,
                    }
                } else {
                    self
                };
                let mut n2 = n;
                let mut num_digits = 0;
                while n2 > 0 {
                    n2 /= 10;
                    num_digits += 1;
                }
                let len = if neg { num_digits + 1 } else { num_digits };
                to.reserve(len);
                let ptr = to.as_mut_ptr().cast::<u8>();
                let old_len = to.len();
                let mut i = len - 1;
                loop {
                    unsafe { ptr.add(old_len + i).write((n % 10) as u8 + b'0') }
                    n /= 10;

                    if n == 0 {
                        break;
                    } else {
                        i -= 1;
                    }
                }

                if neg {
                    i -= 1;
                    unsafe { ptr.add(old_len + i).write(b'-') }
                }

                #[allow(clippy::uninit_vec)]
                unsafe {
                    to.set_len(old_len + (len - i));
                }
            }
        }
    };
}

write_unsized!(u8);
write_unsized!(u16);
write_unsized!(u32);
write_unsized!(u64);
write_unsized!(u128);
write_unsized!(usize);

write_sized!(i8);
write_sized!(i16);
write_sized!(i32);
write_sized!(i64);
write_sized!(i128);
write_sized!(isize);
