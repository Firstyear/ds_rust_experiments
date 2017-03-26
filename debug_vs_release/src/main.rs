

fn main() {
    if cfg!(debug_assertions) {
        println!("Hello, Debug!");
    } else {
        println!("Hello, Release!");
    }
}
