use std::{fmt::Arguments, ops::RangeInclusive};

use crate::{builder::VecLike, MsgChannel};

pub trait IntoValue {
    const LEN: RangeInclusive<Option<usize>>;

    fn encode<V: VecLike>(self, v: &mut MsgChannel<V>);
}

impl IntoValue for bool {
    const LEN: RangeInclusive<Option<usize>> = RangeInclusive::new(Some(1), Some(1));

    fn encode<V: VecLike>(self, v: &mut MsgChannel<V>) {
        v.msg.add_element(if self { 255 } else { 0 });
    }
}

impl<S: AsRef<str>> IntoValue for &S {
    const LEN: RangeInclusive<Option<usize>> = RangeInclusive::new(Some(2), Some(256));

    fn encode<V: VecLike>(self, v: &mut MsgChannel<V>) {
        v.encode_str(format_args!("{}", self.as_ref()));
    }
}

impl IntoValue for Arguments<'_> {
    const LEN: RangeInclusive<Option<usize>> = RangeInclusive::new(Some(2), Some(256));

    fn encode<V: VecLike>(self, v: &mut MsgChannel<V>) {
        v.encode_str(self);
    }
}
