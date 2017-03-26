
use std::mem;

// Show a simple function call.

#[no_mangle]
pub extern fn ehasx_test() -> i32
{
    1
}

pub struct TestEhasx {
    internal: i32,
}

impl TestEhasx {
    // Remember, these needs to be pub to prevent dead-code elimination.
    // Use an anonymous function off a struct impl.
    #[no_mangle]
    pub fn anon_struct_fn() -> i32 {
        2
    }

    pub fn new() -> Self {
        TestEhasx{
            internal: 3,
        }
    }

    pub fn struct_internal_fn(&self) -> i32 {
        self.internal
    }

}

// Here is our C exposed API, which takes the pointer and calls internal parts.

#[no_mangle]
pub unsafe fn testehasx_new() -> *mut TestEhasx {
    let testehasx = Box::new(TestEhasx::new());
    Box::into_raw(testehasx)
}

#[no_mangle]
pub unsafe fn testehasx_struct_fn(sptr: *mut TestEhasx) -> i32 {
    let tehasx: Box<TestEhasx> = Box::from_raw(sptr);
    let retval = tehasx.struct_internal_fn();
    mem::forget(tehasx);
    retval
}

#[no_mangle]
pub unsafe fn testehasx_destroy(sptr: *mut TestEhasx) {
    let _: Box<TestEhasx> = Box::from_raw(sptr);
}

#[cfg(test)]
mod tests {
    use super::testehasx_new;
    use super::testehasx_destroy;
    use super::testehasx_struct_fn;

    #[test]
    fn unbox_pointer_call() {
        // now call things on it.
        unsafe {
            let sptr = testehasx_new();
            assert!(testehasx_struct_fn(sptr) == 3);
            testehasx_destroy(sptr);
        }
    }
}
