#[cfg(test)]

mod unsafe_tests {

    //-------------------------------------------------------------
    // use raw pointer
    //
    //-------------------------------------------------------------
    #[tokio::test]
    async fn test_unsafe_raw_pointer() {
        let x = 42;
        let raw_ptr = &x as *const i32;
        unsafe {
            assert_eq!(*raw_ptr, 42);
            println!("raw ptr value {}", *raw_ptr);
        }
    }

    //-------------------------------------------------------------
    // call unsafe function
    //
    //-------------------------------------------------------------
    #[tokio::test]
    async fn test_unsafe_call() {
        unsafe { dangerous_function() }
    }

    unsafe fn dangerous_function() {
        println!("This is unsafe")
    }

    //-------------------------------------------------------------
    // calling and update static variable
    //
    //-------------------------------------------------------------
    static mut COUNTER: u32 = 0;

    fn increase_counter() {
        unsafe { COUNTER += 12 }
    }

    #[tokio::test]
    async fn test_unsafe_static() {
        increase_counter();
        unsafe { println!("Counter: {}", COUNTER) }
    }

    //-------------------------------------------------------------
    // implement unsafe trait and call for test
    //
    //-------------------------------------------------------------
    unsafe trait UnsafeTrait {
        fn do_something(&self);
    }

    unsafe impl UnsafeTrait for i32 {
        fn do_something(&self) {
            println!("Doing something unsafe with {}", self)
        }
    }

    #[tokio::test]
    async fn test_unsafe_trait() {
        let x = 75;
        x.do_something()
    }

    //-------------------------------------------------------------
    // implement unsafe foreign function interface (FFI)
    //
    //-------------------------------------------------------------
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    #[tokio::test]
    async fn test_unsafe_ffi() {
        let x = -36;
        unsafe { println!("absolute value {}", abs(x)) }
    }

    //-------------------------------------------------------------
    // use the unsafe to get or set memory
    //
    //-------------------------------------------------------------
    #[tokio::test]
    async fn test_unsafe_memory() {
        use std::alloc::{alloc, dealloc, Layout};

        let layout = Layout::new::<i32>();

        unsafe {
            let ptr = alloc(layout) as *mut i32;
            *ptr = 45;
            println!("memory ptr value {}", *ptr); // output 45
            dealloc(ptr as *mut u8, layout); // free the memory
            println!("memory ptr value {}", *ptr); // output -604389376 because the ptr is released
        }
    }

    //-------------------------------------------------------------
    // fill memory while uninit
    //
    //-------------------------------------------------------------
    #[tokio::test]
    async fn test_unsafe_uninit() {
        use std::mem::MaybeUninit;
        let mut x: MaybeUninit<i32> = MaybeUninit::<i32>::uninit();
        unsafe {
            x.as_mut_ptr().write(37);
            println!("unsafe uninit value write {}", x.assume_init());
        }
    }
}
