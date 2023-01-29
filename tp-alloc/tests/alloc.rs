use tp_alloc::*;
use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

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
fn test_one_block_alloc() {

    // Allocator created on the heap
    let alloc = Box::new(MyAlloc::new());
    let my_alloc = alloc.as_ref();

    // Layout of one block's size
    let one_block = Layout::new::<u8>();

    // Alloc block by block all the memory 
    for i in 0..ALLOC_BLOCK_NUM {
        unsafe {
            let ret = my_alloc.alloc(one_block);
            assert_eq!(ret, my_alloc.memory.get().cast::<u8>().add(i*ALLOC_BLOCK_SIZE))
        }
    }

    // All the memory should be allocated 
    // Not possible anymore to alloc a new block
    unsafe {
        let try_alloc = my_alloc.alloc(one_block);
        assert_eq!(try_alloc, null_mut());
    }
}

#[test]
fn test_alloc_dealloc() {
    
    // Allocator created on the heap
    let alloc = Box::new(MyAlloc::new());
    let my_alloc = alloc.as_ref();

    // Layout of the maximum size of the memory
    let all_mem = Layout::new::<[u8; ALLOC_BLOCK_NUM*ALLOC_BLOCK_SIZE]>();

    // Alloc and then dealloc all the memory 3 times
    // If the memory hasn't been deallocated correctly, 
    // alloc function wouldn't return the first block of the memory
    for _i in 0..3 {

        // Alloc all the memory
        unsafe { 
            let first_alloc_block = my_alloc.alloc(all_mem);
            let first_block_of_mem = my_alloc.memory.get().cast::<u8>();
            assert_eq!(first_alloc_block, first_block_of_mem);
        }

        // Dealloc all the memory
        unsafe { my_alloc.dealloc(my_alloc.memory.get().cast::<u8>(), all_mem); }
    }
}
