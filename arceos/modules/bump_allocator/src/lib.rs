#![no_std]
use allocator::{AllocResult, BaseAllocator, ByteAllocator, PageAllocator, AllocError};
use core::alloc::Layout;
use core::ptr::NonNull;
/// Early memory allocator
/// Use it before formal bytes-allocator and pages-allocator can work!
/// This is a double-end memory range:
/// - Alloc bytes forward
/// - Alloc pages backward
///
/// [ bytes-used | avail-area | pages-used ]
/// |            | -->    <-- |            |
/// start       b_pos        p_pos       end
///
/// For bytes area, 'count' records number of allocations.
/// When it goes down to ZERO, free bytes-used area.
/// For pages area, it will never be freed!
///
pub struct EarlyAllocator<const PAGE_SIZE:usize>{
    start:usize,
    end:usize,
    b_pos:usize,
    p_pos:usize,
    count:usize,
}



impl <const PAGE_SIZE:usize>EarlyAllocator<PAGE_SIZE> {
    pub const fn new() -> Self {
        Self {
            start:0,
            end: 0,
            b_pos: 0,
            p_pos: 0,
            count:0,
        }
    }
}

fn align_down(addr: usize, align: usize) -> usize {
    addr & !(align - 1)
}

fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

impl <const PAGE_SIZE:usize>BaseAllocator for EarlyAllocator<PAGE_SIZE> {
    /// Initialize the allocator with a free memory region.
    fn init(&mut self, start: usize, size: usize) {

        self.start = start;
        self.end = start + size;
        self.b_pos = start;
        self.p_pos = start + size;
        self.count = 0;
    }

    /// Add a free memory region to the allocator.
    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        unimplemented!()
    }
}

impl <const PAGE_SIZE:usize>ByteAllocator for EarlyAllocator<PAGE_SIZE>  {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        let size = layout.size();
        let align = layout.align();
        let addr = align_up(self.b_pos, align);
        if addr + size > self.p_pos {
            return Err(AllocError::NoMemory);
        }
        self.b_pos = addr + size;
        Ok(NonNull::new(addr as *mut u8).unwrap())
    }

    fn dealloc(&mut self, _pos: NonNull<u8>, layout: Layout) {
        let size = layout.size();
        self.b_pos = self.b_pos.saturating_sub(size);
    }

    fn total_bytes(&self) -> usize {
        self.end - self.start
    }

    fn used_bytes(&self) -> usize {
        self.b_pos - self.start
    }

    fn available_bytes(&self) -> usize {
        self.p_pos - self.b_pos
    }
}

impl <const PAGE_SIZE:usize>PageAllocator for EarlyAllocator<PAGE_SIZE> {
    const PAGE_SIZE: usize = PAGE_SIZE;
    fn alloc_pages(&mut self, _num_pages: usize, _align_pow2: usize) -> AllocResult<usize> {
        unimplemented!()
    }

    fn dealloc_pages(&mut self, _pos: usize, _num_pages: usize) {
        unimplemented!()
    }

    fn total_pages(&self) -> usize {
        unimplemented!()
    }

    fn used_pages(&self) -> usize {
        unimplemented!()
    }

    fn available_pages(&self) -> usize {
        unimplemented!()
    }
}
