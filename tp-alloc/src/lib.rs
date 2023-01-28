use parking_lot::Mutex;
use std::cell::UnsafeCell;
use std::alloc::{GlobalAlloc, Layout};
use std::ptr::null_mut;

const ALLOC_BLOCK_SIZE: usize = 64;       // 64 bytes in each blocks
const ALLOC_BLOCK_NUM:  usize = 16384;    // 16_384 blocks available

pub struct MyAllocData {
    block_status: [bool; ALLOC_BLOCK_NUM],
}

pub struct MyAlloc {
    memory: UnsafeCell<[u8; ALLOC_BLOCK_NUM*ALLOC_BLOCK_SIZE]>,
    data: Mutex<MyAllocData>,
}

unsafe impl Sync for MyAlloc {}

impl MyAlloc {

    // Initilize statically the allocator 
    pub const fn new() -> Self {
        MyAlloc { 
            memory: UnsafeCell::new([0; ALLOC_BLOCK_NUM*ALLOC_BLOCK_SIZE]), 
            data: Mutex::new(MyAllocData::new()),
        }
    }

}

unsafe impl GlobalAlloc for MyAlloc {

    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // usize will round down the result of the div, that's why we add 1
        let num_blocks = (layout.size() / ALLOC_BLOCK_SIZE) + 1;
        let first_block = self.data.lock().find_blocks(num_blocks);
        match first_block {
            Some(index)  => {
                self.data.lock().mark_blocks(index, num_blocks, false);
                self.memory.get().cast::<u8>().add(index)
            }
            None => null_mut(),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // Get the addr of the beginning of the memory 
        let first_ptr = self.memory.get().cast::<u8>();
        // Get the lenght from first_ptr to ptr
        let offset = ptr.offset_from(first_ptr);
        // Here we don't add 1 because we want to round down
        // First block to dealloc 
        let first_block = offset as usize / ALLOC_BLOCK_SIZE;
        // usize will round down the result of the div, that's why we add 1
        let num_blocks = (layout.size() / ALLOC_BLOCK_SIZE) + 1;
        // Free the blocks
        self.data.lock().mark_blocks(first_block, num_blocks, true);
    }
}

impl MyAllocData {

    // Create a struct MyAllocData. At the beginning all blocks are free
    const fn new() -> Self {
        MyAllocData { block_status: [true; ALLOC_BLOCK_NUM] }
    }

    // Function to label num_blocks of block_status by state (free or occupied)
    fn mark_blocks(&mut self, first_block: usize, num_blocks: usize, state: bool) {

        if first_block >= ALLOC_BLOCK_NUM { panic!("Invalid first block index"); }
        else if first_block + num_blocks >= ALLOC_BLOCK_NUM { panic!("Some blocks are out of bounds"); }
        else {
            for i in first_block..(first_block + num_blocks) {
                self.block_status[i] = state;
            }
        }
    }

    // Function which returns the index of the first blocks of a set of num_blocks free blocks
    fn find_blocks(&self, num_blocks: usize) -> Option<usize> {

        if num_blocks > ALLOC_BLOCK_NUM { panic!("Invalid blocks number"); }

        let mut count = 0;
        for i in 0..ALLOC_BLOCK_NUM {
            if self.block_status[i] { 
                count += 1;
                if count == num_blocks {
                    return Some(i - num_blocks + 1);
                }
            }
            else {
                count = 0;
            }
        }
        None
    }
}


//#[cfg(test)]
//mod tests {
//
//    #[test]
//    fn it_works() {
//        let test: usize = 112/64;
//        println!("{}", test);
//        assert_eq!(1, test);
//    }
//}
