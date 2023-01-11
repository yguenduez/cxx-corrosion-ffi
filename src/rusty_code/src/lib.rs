mod modbus;

use modbus::ModbusSender;

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type MyRustType;
        fn give_me_42(self: &MyRustType)->usize;
        fn new_type() -> Box<MyRustType>;

        type ModbusSender;
        fn new_modbus_sender() -> Box<ModbusSender>;
        fn send(self: &ModbusSender, bytes: Vec<u8>);
    }
}

fn new_type() -> Box<MyRustType>{
    Box::new(MyRustType)
}
fn new_modbus_sender() -> Box<ModbusSender> {Box::new(ModbusSender)}

struct MyRustType;

impl MyRustType{
    fn give_me_42(&self) -> usize{
        42
    }
}
