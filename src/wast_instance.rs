use wasmer::{imports, Instance, Module, Store, Value};
use wast;

crate::entry_point!("wast_instance", go);

pub fn mk(x: &str) -> Instance {
    let buf = wast::parser::ParseBuffer::new(x).unwrap();
    let mut wat = wast::parser::parse::<wast::Wat>(&buf).unwrap();
    let bs = wat.encode().unwrap();
    let store = Store::default();
    let module = Module::new(&store, &bs).unwrap();
    let import_object = imports! {};
    Instance::new(&module, &import_object).unwrap()
}

pub fn run(x: &str, f: &str) -> Box<[wasmer::Val]> {
    let instance = mk(x);
    let f = instance.exports.get_function(f).unwrap();
    let y = f.call(&[]);
    y.unwrap()
}

pub fn main(x: &str) -> Box<[wasmer::Val]> {
    run(x, "main")
}

fn go() {
    let wast = r#"(module
        (func $f (export "read") (param i64 f32 f64 i32 i32) (result f64)
            (local f32 i64 i64 f64)
            (local.set 5 (f32.const 5.5))
            (local.set 6 (i64.const 6))
            (local.set 8 (f64.const 8))
            (f64.add
            (f64.convert_i64_u (local.get 0))
            (f64.add
                (f64.promote_f32 (local.get 1))
                (f64.add
                (local.get 2)
                (f64.add
                    (f64.convert_i32_u (local.get 3))
                    (f64.add
                    (f64.convert_i32_s (local.get 4))
                    (f64.add
                        (f64.promote_f32 (local.get 5))
                        (f64.add
                        (f64.convert_i64_u (local.get 6))
                        (f64.add
                            (f64.convert_i64_u (local.get 7))
                            (local.get 8)
                        )
                        )
                    )
                    )
                )
                )
            )
            )
        )
        (func (export "main") (result f64)
            (call $f (i64.const 1) (f32.const 2) (f64.const 3.3) (i32.const 4) (i32.const 5))
        )
    )
    "#;

    let instance = mk(wast);

    let f = instance.exports.get_function("read").unwrap();
    let result = f.call(&[
        Value::I64(1),
        Value::F32(2.0),
        Value::F64(3.3),
        Value::I32(4),
        Value::I32(5),
    ]);
    let result1 = result.clone();

    assert_eq!(result.unwrap()[0], Value::F64(34.8));
    assert_eq!(main(wast)[0], result1.unwrap()[0]);
}
