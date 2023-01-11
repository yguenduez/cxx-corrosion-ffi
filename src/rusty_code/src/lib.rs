#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type MyRustType;
        fn give_me_42(&self)->usize;
        fn new_type() -> Box<MyRustType>;
    }
}

fn new_type() -> Box<MyRustType>{
    Box::new(MyRustType)
}

struct MyRustType;

impl MyRustType{
    fn give_me_42(&self) -> usize{
        42
    }
}
