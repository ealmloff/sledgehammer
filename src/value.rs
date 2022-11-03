use std::ops::RangeInclusive;

use crate::{builder::WritableText, MsgChannel};

use self::sealed::Sealed;

/// Anything that can be turned into a value
pub trait IntoValue: Sealed {
    const LEN: RangeInclusive<Option<usize>>;

    fn encode(self, v: &mut MsgChannel);
}

impl<W> IntoValue for W
where
    W: WritableText,
{
    const LEN: RangeInclusive<Option<usize>> = RangeInclusive::new(Some(2), Some(256));

    fn encode(self, v: &mut MsgChannel) {
        v.encode_str(self);
    }
}

mod sealed {
    use crate::builder::WritableText;

    pub trait Sealed {}

    impl<W: WritableText> Sealed for W {}
}
