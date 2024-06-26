// Copyright (c) Microsoft Corporation
// License: MIT OR Apache-2.0

//! Allocator implementation to use with `#[global_allocator]` to allow use of
//! [`core::alloc`].
//!
//! # Example
//! ```rust, no_run
//! #[cfg(not(test))]
//! use wdk_alloc::WDKAllocator;
//!
//! #[cfg(not(test))]
//! #[global_allocator]
//! static GLOBAL_ALLOCATOR: WDKAllocator = WDKAllocator;
//! ```

#![no_std]

use core::alloc::{GlobalAlloc, Layout};

use wdk_sys::{
    ntddk::ExFreePool, POOL_TYPE, PVOID, SIZE_T, ULONG, _POOL_TYPE::NonPagedPoolExecute,
};

extern "C" {
    pub fn ExAllocatePoolWithTag(Flags: POOL_TYPE, NumberOfBytes: SIZE_T, Tag: ULONG) -> PVOID;
}

/// Allocator implementation to use with `#[global_allocator]` to allow use of
/// [`core::alloc`].
///
/// # Safety
/// This allocator is only safe to use for allocations happening at `IRQL` <=
/// `DISPATCH_LEVEL`
pub struct WDKAllocator;

// The value of memory tags are stored in little-endian order, so it is
// convenient to reverse the order for readability in tooling (ie. Windbg)
const RUST_TAG: ULONG = u32::from_ne_bytes(*b"xxrs");

// SAFETY: This is safe because the WDK allocator:
//         1. can never unwind since it can never panic
//         2. has implementations of alloc and dealloc that maintain layout
//            constraints (FIXME: Alignment of the layout is currenty not
//            supported)
unsafe impl GlobalAlloc for WDKAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr =
            // SAFETY: `ExAllocatePool2` is safe to call from any `IRQL` <= `DISPATCH_LEVEL` since its allocating from `POOL_FLAG_NON_PAGED`
            unsafe {
                ExAllocatePoolWithTag(NonPagedPoolExecute, layout.size() as SIZE_T, RUST_TAG)
            };
        if ptr.is_null() {
            return core::ptr::null_mut();
        }
        ptr.cast()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        // SAFETY: `ExFreePool` is safe to call from any `IRQL` <= `DISPATCH_LEVEL`
        // since its freeing memory allocated from `POOL_FLAG_NON_PAGED` in `alloc`
        unsafe {
            ExFreePool(ptr.cast());
        }
    }
}
