use std::{fmt::Arguments, ops::RangeInclusive};

use crate::MsgChannel;

pub trait IntoValue {
    const LEN: RangeInclusive<Option<usize>>;

    fn encode(self, v: &mut MsgChannel);
}

impl IntoValue for bool {
    const LEN: RangeInclusive<Option<usize>> = RangeInclusive::new(Some(1), Some(1));

    fn encode(self, v: &mut MsgChannel) {
        v.msg.push(if self { 255 } else { 0 });
    }
}

impl<S: AsRef<str>> IntoValue for &S {
    const LEN: RangeInclusive<Option<usize>> = RangeInclusive::new(Some(2), Some(256));

    fn encode(self, v: &mut MsgChannel) {
        v.encode_str(format_args!("{}", self.as_ref()));
    }
}

impl IntoValue for Arguments<'_> {
    const LEN: RangeInclusive<Option<usize>> = RangeInclusive::new(Some(2), Some(256));

    fn encode(self, v: &mut MsgChannel) {
        v.encode_str(self);
    }
}
