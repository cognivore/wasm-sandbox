crate::entry_point!("run_bin", main);

use wasmer::{imports, Instance, Module, Store};

pub fn run(x: Instance, fname: &str) -> Box<[wasmer::Val]> {
    let f = x.exports.get_function(fname).unwrap();
    f.call(&[]).unwrap()
}

/*

// Sum type for the different types of values we can get from the interpreter.
#[derive(Debug)]
pub enum SimpleVal {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    V128(u128),
    // AnyRef(Option<Box<Val>>),
    // FuncRef(Option<Box<Val>>),
    // ExternRef(Option<Box<Val>>),
    // NullRef,
    // Tagged(Box<Val>, String),
}

pub fn unwrap_tagged(y: Box<[wasmer::Val]>, tag: String) -> SimpleVal {
    // match tag with some known simple values
    match tag.as_str() {
        "i32" => SimpleVal::I32(y[0].unwrap_i32()),
        "i64" => SimpleVal::I64(y[0].unwrap_i64()),
        "f32" => SimpleVal::F32(y[0].unwrap_f32()),
        "f64" => SimpleVal::F64(y[0].unwrap_f64()),
        "v128" => SimpleVal::V128(y[0].unwrap_v128()),
        _ => panic!("Unknown tag: {}", tag),
    }
}

*/

fn main(args: Vec<String>) {
    // Zeroth argument is the path to the binary
    let bin = args[0].clone();
    // First argument is optional. If it's set, it's the name of the function we need to run. Otherwise, the name of the function is `main`.
    let func_name = if args.len() > 1 {
        args[1].clone()
    } else {
        "main".to_string()
    };

    // Read the binary from the file as a new Vec<u8>
    let bin = std::fs::read(bin).unwrap();

    // Now we can use Module::new from wasmer to create a new module from the binary.
    let module = Module::new(&Store::default(), &bin).unwrap();

    // We don't use any imports, so we can create an empty import object.
    let import_object = imports! {};

    // Instantiate the module with the import object.
    let instance = Instance::new(&module, &import_object).unwrap();

    // Run the function and debug its result.
    let y = run(instance, &func_name);
    // Debug the result now.
    println!("{:?}", y);
}
