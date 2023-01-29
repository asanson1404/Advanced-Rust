use tp_alloc::*;

#[global_allocator]
static ALLOCATOR: MyAlloc = MyAlloc::new();

fn main() {

    let my_box: Box<u8> = Box::new(5);
    println!("my_box = {}", my_box);

    let mut my_vec: Vec<u8> = Vec::with_capacity(10);
    for i in 0..10 {
        my_vec.push(i);
    }
    println!("my_vec = {:?}", my_vec);
}