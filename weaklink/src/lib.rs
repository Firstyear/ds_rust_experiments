
#[no_mangle]
pub fn external_symbol() {
    println!("Hello From Rust!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
