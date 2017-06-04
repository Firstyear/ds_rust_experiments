
extern crate dylib;

use std::path::Path;
use dylib::DynamicLibrary;


fn main() {

    let libpath = Path::new("../plugin/.libs/libplugin.so");

    match DynamicLibrary::open(Some(libpath)) {
        Ok(handle) => {
            println!("Opened!");
            unsafe {
                // Now we get dangerous: We have to unsafe access
                // the raw pointer of the function.
                let mfptr: *mut () = handle.symbol("plugin_init").unwrap();
                let fptr = std::mem::transmute::<*mut (), fn() -> i32>(mfptr);
                println!("Fn returns: {}", fptr());
            }
        },
        Err(e) => println!("Failed to open: {}", e),
    }

    println!("Hello, world!");
}

