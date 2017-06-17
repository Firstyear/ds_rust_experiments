
#[no_mangle]
pub extern fn plugin_init() -> i32 {
    let xs = [0, 1, 2, 3];
    println!("{:?}", xs);
    let y = unsafe { *xs.as_ptr().offset(4) };
    // 0
    y
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
