
extern crate gcc;

fn main() {
    gcc::compile_library("librwcnative.a", &["src/native.c"]);
}

