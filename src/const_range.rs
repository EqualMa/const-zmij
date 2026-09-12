use core::ops::RangeInclusive;

pub(crate) struct ConstRange<T>(pub T);

impl ConstRange<RangeInclusive<i32>> {
    /// const version of [`RangeInclusive::contains`]
    #[allow(clippy::trivially_copy_pass_by_ref)]
    #[inline]
    pub(crate) const fn contains(&self, item: &i32) -> bool {
        let Self(this) = self;
        *this.start() <= *item && *item <= *this.end()
    }
}
