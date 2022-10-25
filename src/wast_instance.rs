use wasmer::{imports, Instance, Module, Store, Value};
use wast;

crate::entry_point!("wast_instance", main);

fn main() {
    let wast = r#"(module
        (func (export "read") (param i64 f32 f64 i32 i32) (result f64)
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
    )
    "#;
    let buf = wast::parser::ParseBuffer::new(wast).unwrap();
    let mut wat = wast::parser::parse::<wast::Wat>(&buf).unwrap();
    let bs = wat.encode().unwrap();
    let store = Store::default();
    let module = Module::new(&store, &bs).unwrap();
    let import_object = imports! {};
    let instance = Instance::new(&module, &import_object).unwrap();

    let f = instance.exports.get_function("read").unwrap();
    let result = f.call(&[
        Value::I64(1),
        Value::F32(2.0),
        Value::F64(3.3),
        Value::I32(4),
        Value::I32(5),
    ]);

    assert_eq!(result.unwrap()[0], Value::F64(0.0));

    ()
}
