use std::io::{self, Read};

use wasmer::{imports, Engine, Instance, Module, Store};

crate::entry_point!("wevalf", main, _WE_MAIN);

fn main(_: Vec<String>) {
    let mut b: Vec<u8> = vec![];
    io::stdin().read_to_end(&mut b).unwrap();
    let store = Store::default();
    let module = Module::new(&store, &b).unwrap();
    let impo = imports! {};
    let mut store = Store::new(Engine::default());
    let instance = Instance::new(&mut store, &module, &impo).unwrap();
    let phi = instance.exports.get_function("main").unwrap();
    let y = phi.call(&mut store, &[]);
    let res = y;
    dbg!("{}", res.unwrap()[0].unwrap_i32());
}
