

#[no_mangle]
pub extern fn alice_init() -> bool {
    println!("It works!");
    true
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
