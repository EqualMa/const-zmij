pub(crate) struct ConstArray<T>(pub T);

impl<T, const N: usize> ConstArray<[T; N]> {
    pub(crate) const fn as_ptr(&self) -> *const T {
        self.0.as_ptr()
    }
    pub(crate) const unsafe fn get_unchecked(&self, i: usize) -> &T {
        debug_assert!(i < self.0.len());
        unsafe {
            ::core::hint::assert_unchecked(i < self.0.len());
            &self.0[i]
        }
    }
}
