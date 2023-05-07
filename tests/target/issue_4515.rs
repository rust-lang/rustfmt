pub unsafe fn as_chunks_mut_unchecked<const N: usize>(&mut self) -> &mut [[T; N]] {
    debug_assert_ne!(N, 0);
    debug_assert_eq!(self.len() % N, 0);
    let new_len =
        // SAFETY: Our precondition is exactly what's needed to call this
        unsafe { crate::intrinsics::exact_div(self.len(), N) };
    // SAFETY: We cast a slice of `new_len * N` elements into
    // a slice of `new_len` many `N` elements chunks.
    unsafe { from_raw_parts_mut(self.as_mut_ptr().cast(), new_len) }
}
