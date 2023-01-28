use tp_alloc::*;
use std::alloc::{GlobalAlloc, Layout};
use std::ptr::null_mut;

#[test]
fn test_max_alloc() {

    // Allocator created on the heap
    let alloc = Box::new(MyAlloc::new());
    let my_alloc = alloc.as_ref();

    // Layout of the maximum size of the memory
    let all = Layout::new::<[u8; ALLOC_BLOCK_NUM*ALLOC_BLOCK_SIZE]>();
    // Layout of one byte
    let one_byte = Layout::new::<u8>();

    // Verify the allocator function : we allocate all the memory
    unsafe {
        let first_alloc_block = my_alloc.alloc(all);
        let first_block_of_mem = my_alloc.memory.get().cast::<u8>();
        assert_eq!(first_alloc_block, first_block_of_mem);
    }
    
    // Verify if indeed, all the memory has been allocated
    // Should not be possible to allocate one more byte
    unsafe {
        let try_alloc = my_alloc.alloc(one_byte);
        assert_eq!(try_alloc, null_mut());
    }
}

#[test]
fn one_block_alloc() {

    // Allocator created on the heap
    let alloc = Box::new(MyAlloc::new());
    let my_alloc = alloc.as_ref();

    // Layout of one block's size
    let one_block = Layout::new::<[u8; ALLOC_BLOCK_SIZE]>();

    // Alloc block by block all the memory 
    for _i in 0..ALLOC_BLOCK_NUM {
        unsafe {
            my_alloc.alloc(one_block);
        }
    }

    // All the memory should be allocated 
    // Not possible anymore to alloc a new block
    unsafe {
        let try_alloc = my_alloc.alloc(one_block);
        assert_eq!(try_alloc, null_mut());
    }

}
