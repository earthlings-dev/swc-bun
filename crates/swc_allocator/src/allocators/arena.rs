use std::ops::{Deref, DerefMut};

use bumpalo::Bump;

/// Arena allocator
#[derive(Default)]
pub struct Arena {
    inner: Bump,
}

impl Deref for Arena {
    type Target = Bump;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Arena {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl From<Bump> for Arena {
    fn from(inner: Bump) -> Self {
        Self { inner }
    }
}

unsafe impl allocator_api2::alloc::Allocator for &'_ Arena {
    #[inline]
    fn allocate(
        &self,
        layout: std::alloc::Layout,
    ) -> Result<std::ptr::NonNull<[u8]>, allocator_api2::alloc::AllocError> {
        let ptr = self
            .inner
            .try_alloc_layout(layout)
            .map_err(|_| allocator_api2::alloc::AllocError)?;
        let slice = std::ptr::slice_from_raw_parts_mut(ptr.as_ptr(), layout.size());
        Ok(unsafe { std::ptr::NonNull::new_unchecked(slice) })
    }

    #[inline]
    unsafe fn deallocate(&self, ptr: std::ptr::NonNull<u8>, layout: std::alloc::Layout) {
        let _ = (ptr, layout);
    }

    #[inline]
    fn allocate_zeroed(
        &self,
        layout: std::alloc::Layout,
    ) -> Result<std::ptr::NonNull<[u8]>, allocator_api2::alloc::AllocError> {
        let ptr = self
            .inner
            .try_alloc_layout(layout)
            .map_err(|_| allocator_api2::alloc::AllocError)?;
        if layout.size() != 0 {
            unsafe {
                std::ptr::write_bytes(ptr.as_ptr(), 0, layout.size());
            }
        }
        let slice = std::ptr::slice_from_raw_parts_mut(ptr.as_ptr(), layout.size());
        Ok(unsafe { std::ptr::NonNull::new_unchecked(slice) })
    }

    #[inline]
    unsafe fn grow(
        &self,
        ptr: std::ptr::NonNull<u8>,
        old_layout: std::alloc::Layout,
        new_layout: std::alloc::Layout,
    ) -> Result<std::ptr::NonNull<[u8]>, allocator_api2::alloc::AllocError> {
        unsafe {
            let new_ptr = self
                .inner
                .try_alloc_layout(new_layout)
                .map_err(|_| allocator_api2::alloc::AllocError)?;
            let copy_len = old_layout.size().min(new_layout.size());
            if copy_len != 0 {
                std::ptr::copy_nonoverlapping(ptr.as_ptr(), new_ptr.as_ptr(), copy_len);
            }
            let slice = std::ptr::slice_from_raw_parts_mut(new_ptr.as_ptr(), new_layout.size());
            Ok(std::ptr::NonNull::new_unchecked(slice))
        }
    }

    #[inline]
    unsafe fn grow_zeroed(
        &self,
        ptr: std::ptr::NonNull<u8>,
        old_layout: std::alloc::Layout,
        new_layout: std::alloc::Layout,
    ) -> Result<std::ptr::NonNull<[u8]>, allocator_api2::alloc::AllocError> {
        unsafe {
            let new_ptr = self
                .inner
                .try_alloc_layout(new_layout)
                .map_err(|_| allocator_api2::alloc::AllocError)?;
            let old_size = old_layout.size();
            let new_size = new_layout.size();
            let copy_len = old_size.min(new_size);
            if copy_len != 0 {
                std::ptr::copy_nonoverlapping(ptr.as_ptr(), new_ptr.as_ptr(), copy_len);
            }
            if new_size > old_size {
                std::ptr::write_bytes(new_ptr.as_ptr().add(old_size), 0, new_size - old_size);
            }
            let slice = std::ptr::slice_from_raw_parts_mut(new_ptr.as_ptr(), new_size);
            Ok(std::ptr::NonNull::new_unchecked(slice))
        }
    }

    #[inline]
    unsafe fn shrink(
        &self,
        ptr: std::ptr::NonNull<u8>,
        old_layout: std::alloc::Layout,
        new_layout: std::alloc::Layout,
    ) -> Result<std::ptr::NonNull<[u8]>, allocator_api2::alloc::AllocError> {
        unsafe {
            let _ = old_layout;
            let slice = std::ptr::slice_from_raw_parts_mut(ptr.as_ptr(), new_layout.size());
            Ok(std::ptr::NonNull::new_unchecked(slice))
        }
    }
}
