use std::ops::RangeInclusive;

use crate::{MsgChannel, Writable};

pub trait IntoValue {
    const LEN: RangeInclusive<Option<usize>>;

    fn encode(self, v: &mut MsgChannel);
}

impl<W> IntoValue for W
where
    W: Writable,
{
    const LEN: RangeInclusive<Option<usize>> = RangeInclusive::new(Some(2), Some(256));

    fn encode(self, v: &mut MsgChannel) {
        v.encode_str(self);
    }
}
