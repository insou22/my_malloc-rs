mod alloc {
    #![deny(unsafe_op_in_unsafe_fn)]

    use std::alloc::{GlobalAlloc, Layout};

    pub struct Alloc1521;

    impl Alloc1521 {
        pub unsafe fn initialise_heap(heap_size: u32) {
            unsafe { init_heap(heap_size) }
        }
    }
    
    unsafe impl GlobalAlloc for Alloc1521 {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            unsafe { my_malloc(layout.size().try_into().expect("max allocation size is u32::MAX") ) }
        }
    
        unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
            unsafe { my_free(ptr) }
        }
    }

    #[link(name = "allocator")]
    extern "C" {
        fn init_heap(heap_size: u32);

        fn my_malloc(size: u32) -> *mut u8;

        fn my_free(ptr: *mut u8);
    }
}

use alloc::Alloc1521;

#[global_allocator]
static GLOBAL: Alloc1521 = Alloc1521;
const HEAP_SIZE: u32 = 4096;

startup::on_startup! {
    unsafe { Alloc1521::initialise_heap(HEAP_SIZE) }
}

fn main() {
    let values = vec![1, 2, 3, 4, 5];

    let tripled = values.into_iter()
        .map(|value| [value, value, value])
        .flatten()
        .collect::<Vec<_>>();

    println!("{tripled:?}");
}
