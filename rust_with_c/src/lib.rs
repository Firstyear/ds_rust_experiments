
extern {
    fn display_hello();
}

pub fn rs_display_hello() {
    unsafe {
        display_hello()
    }
}


#[cfg(test)]
mod tests {
    use super::rs_display_hello;

    #[test]
    fn it_works() {
        rs_display_hello();
    }
}
