use bumpalo::Bump;
use bytes::{buf::UninitSlice, BufMut, TryGetError};

fn panic_advance(error_info: &TryGetError) -> ! {
    panic!(
        "advance out of bounds: the len is {} but advancing by {}",
        error_info.available, error_info.requested
    );
}

pub(crate) struct BumpBytesMut<'a> {
    inner: bumpalo::collections::Vec<'a, u8>,
}

impl<'a> BumpBytesMut<'a> {
    #[inline(always)]
    pub(crate) fn with_capacity_in(capacity: usize, bump: &'a Bump) -> Self {
        Self { inner: bumpalo::collections::Vec::with_capacity_in(capacity, bump) }
    }

    #[inline(always)]
    pub(crate) fn into_inner(self) -> bumpalo::collections::Vec<'a, u8> {
        self.inner
    }

    #[inline(always)]
    pub(crate) fn len(&self) -> usize {
        self.inner.len()
    }
}

unsafe impl BufMut for BumpBytesMut<'_> {
    #[inline(always)]
    fn remaining_mut(&self) -> usize {
        isize::MAX as usize - self.inner.len()
    }

    #[inline(always)]
    unsafe fn advance_mut(&mut self, cnt: usize) {
        let len = self.inner.len();
        let remaining = self.inner.capacity() - len;

        if remaining < cnt {
            panic_advance(&TryGetError { requested: cnt, available: remaining });
        }

        // Addition will not overflow since the sum is at most the capacity.
        self.inner.set_len(len + cnt);
    }

    #[inline(always)]
    fn chunk_mut(&mut self) -> &mut bytes::buf::UninitSlice {
        if self.inner.capacity() == self.inner.len() {
            self.inner.reserve(64); // Grow the vec
        }

        let cap = self.inner.capacity();
        let len = self.inner.len();

        let ptr = self.inner.as_mut_ptr();
        // SAFETY: Since `ptr` is valid for `cap` bytes, `ptr.add(len)` must be
        // valid for `cap - len` bytes. The subtraction will not underflow since
        // `len <= cap`.
        unsafe { UninitSlice::from_raw_parts_mut(ptr.add(len), cap - len) }
    }
}
