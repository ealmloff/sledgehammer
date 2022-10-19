use std::ops::RangeInclusive;

use crate::{builder::VecLike, MsgBuilder};

pub trait IntoValue {
    const LEN: RangeInclusive<Option<usize>>;

    fn size(&self) -> usize;
    fn encode<V: VecLike<u8>>(self, v: &mut MsgBuilder<V>);
}

impl IntoValue for bool {
    const LEN: RangeInclusive<Option<usize>> = RangeInclusive::new(Some(1), Some(1));

    fn size(&self) -> usize {
        1
    }

    fn encode<V: VecLike<u8>>(self, v: &mut MsgBuilder<V>) {
        v.msg.add_element(if self { 255 } else { 0 });
    }
}

impl<S: AsRef<str>> IntoValue for &S {
    const LEN: RangeInclusive<Option<usize>> = RangeInclusive::new(Some(2), Some(256));

    fn size(&self) -> usize {
        1 + self.as_ref().as_bytes().len()
    }

    fn encode<V: VecLike<u8>>(self, v: &mut MsgBuilder<V>) {
        v.encode_str(self.as_ref());
    }
}
