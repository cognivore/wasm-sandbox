use std::io::{self, Read};

use wasmer::{imports, Instance, Module, Store};
// use wast;

crate::entry_point!("wevalf", main);

fn main(_: Vec<String>) {
    let mut b: Vec<u8> = vec![];
    io::stdin().read_to_end(&mut b).unwrap();
    let store = Store::default();
    let module = Module::new(&store, &b).unwrap();
    let impo = imports! {};
    let instance = Instance::new(&module, &impo).unwrap();
    let phi = instance.exports.get_function("main").unwrap();
    let y = phi.call(&[]);
    let res = y;
    dbg!("{}", res.unwrap()[0].unwrap_i32());
}
