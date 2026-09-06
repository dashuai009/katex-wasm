//! Bounded small-block reuse for the single-threaded browser renderer.
//!
//! This caches allocations, never formula results. Live allocations retain the
//! normal GlobalAlloc lifetime; no arena reset or bulk reclamation is involved.
use std::alloc::{GlobalAlloc, Layout};
use std::cell::UnsafeCell;
use std::ptr;

const SIZES: [usize; 11] = [16, 32, 64, 96, 128, 192, 256, 384, 512, 768, 1024];
const LIMIT: usize = 1024;
const ALIGN: usize = 16;

#[derive(Clone, Copy)]
struct Bucket {
    head: *mut u8,
    count: usize,
}
const EMPTY: Bucket = Bucket {
    head: ptr::null_mut(),
    count: 0,
};

pub struct SmallBlockCache<A> {
    base: A,
    buckets: UnsafeCell<[Bucket; SIZES.len()]>,
}

// This implementation is only installed in single-threaded WASM. Atomics builds
// use the normal allocator. Native tests use a local, non-shared instance.
#[cfg(all(target_family = "wasm", not(target_feature = "atomics")))]
unsafe impl<A> Sync for SmallBlockCache<A> {}

impl<A> SmallBlockCache<A> {
    pub const fn new(base: A) -> Self {
        Self {
            base,
            buckets: UnsafeCell::new([EMPTY; SIZES.len()]),
        }
    }
    fn bucket(layout: Layout) -> Option<usize> {
        if layout.align() > ALIGN {
            return None;
        }
        SIZES.iter().position(|&size| layout.size() <= size)
    }
    fn layout(index: usize) -> Layout {
        Layout::from_size_align(SIZES[index], ALIGN).unwrap()
    }
}

// SAFETY: only an unshared instance or single-threaded WASM may call this
// allocator. Cached blocks belong exclusively to this allocator and have the
// normalized size/alignment of their bucket. Excess blocks go back to `base`.
unsafe impl<A: GlobalAlloc> GlobalAlloc for SmallBlockCache<A> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if let Some(index) = Self::bucket(layout) {
            let bucket = &mut (*self.buckets.get())[index];
            if !bucket.head.is_null() {
                let block = bucket.head;
                bucket.head = block.cast::<*mut u8>().read();
                bucket.count -= 1;
                return block;
            }
            self.base.alloc(Self::layout(index))
        } else {
            self.base.alloc(layout)
        }
    }
    unsafe fn dealloc(&self, block: *mut u8, layout: Layout) {
        if let Some(index) = Self::bucket(layout) {
            let bucket = &mut (*self.buckets.get())[index];
            if bucket.count < LIMIT {
                block.cast::<*mut u8>().write(bucket.head);
                bucket.head = block;
                bucket.count += 1;
            } else {
                self.base.dealloc(block, Self::layout(index));
            }
        } else {
            self.base.dealloc(block, layout);
        }
    }
    unsafe fn realloc(&self, block: *mut u8, old: Layout, new_size: usize) -> *mut u8 {
        let new = Layout::from_size_align_unchecked(new_size, old.align());
        if let Some(index) = Self::bucket(old) {
            if Self::bucket(new) == Some(index) {
                return block;
            }
        } else if Self::bucket(new).is_none() {
            return self.base.realloc(block, old, new_size);
        }
        let replacement = self.alloc(new);
        if !replacement.is_null() {
            ptr::copy_nonoverlapping(block, replacement, old.size().min(new_size));
            self.dealloc(block, old);
        }
        replacement
    }
}

#[cfg(test)]
mod regression_tests {
    use super::*;
    #[test]
    fn cache_reallocation_preserves_live_data_and_alignment() {
        let allocator = SmallBlockCache::new(std::alloc::System);
        unsafe {
            for _round in 0..3 {
                let mut live = Vec::new();
                for i in 1..2000usize {
                    let layout = Layout::from_size_align(i % 1200 + 1, 1 << (i % 7)).unwrap();
                    let block = allocator.alloc(layout);
                    assert!(!block.is_null());
                    assert_eq!(block as usize % layout.align(), 0);
                    ptr::write_bytes(block, (i % 251) as u8, layout.size());
                    live.push((block, layout, (i % 251) as u8));
                }
                for (block, layout, value) in live.into_iter().rev() {
                    let size = layout.size() / 2 + 20;
                    let replacement = allocator.realloc(block, layout, size);
                    assert!(!replacement.is_null());
                    assert_eq!(replacement as usize % layout.align(), 0);
                    assert!(
                        std::slice::from_raw_parts(replacement, size.min(layout.size()))
                            .iter()
                            .all(|b| *b == value)
                    );
                    allocator.dealloc(
                        replacement,
                        Layout::from_size_align(size, layout.align()).unwrap(),
                    );
                }
            }
            for (i, bucket) in (*allocator.buckets.get()).iter_mut().enumerate() {
                assert!(bucket.count <= LIMIT);
                while !bucket.head.is_null() {
                    let block = bucket.head;
                    bucket.head = block.cast::<*mut u8>().read();
                    allocator
                        .base
                        .dealloc(block, SmallBlockCache::<std::alloc::System>::layout(i));
                }
            }
        }
    }
}
