use std::panic;

use wasmer::{imports, Instance, Module, Store, Value};

crate::entry_point!("just_wat", main);

fn main(_ : Vec<String>) {
    // let module_wat = r#"
    // (module
    //   (type $t0 (func (param i32) (result i32)))
    //   (func $add_one (export "add_one") (type $t0) (param $p0 i32) (result i32)
    //     local.get $p0
    //     i32.const 1
    //     i32.add))
    // "#;

    let module_wat = r#"
    (module

      (func $f (export "f") (param $x f32) (result v128)
        (local $a f32) (local $b f32) (local $c f32) (local $d f32) (local $y v128)

        f32.const 0
        f32x4.splat
        local.set $y

        local.get $x
        f32.neg
        local.set $a

        local.get $x
        f32.nearest
        local.set $b

        local.get $x
        f32.trunc
        local.set $c

        local.get $x
        f32.ceil
        local.set $d

        local.get $y
        local.get $a
        f32x4.replace_lane 0
        local.set $y

        local.get $y
        local.get $b
        f32x4.replace_lane 1
        local.set $y

        local.get $y
        local.get $c
        f32x4.replace_lane 2
        local.set $y

        ;; local.get $y
        ;; local.get $d
        (f32x4.replace_lane 3 (local.get $y) (local.get $d))
        local.set $y

        local.get $y
        return

      )

      (func $f32_unv (export "f32.unv") (param $x v128) (param $i i32) (result f32)
        ;; f32x4.extract_lane 0
        (block
          (block
            (block
              (block
                (block (local.get $i)
                       (br_table 0 1 2 3 4)
                )
                local.get $x
                f32x4.extract_lane 0
                return
              )
              local.get $x
              f32x4.extract_lane 1
              return
            )
            local.get $x
            f32x4.extract_lane 2
            return
          )
          local.get $x
          f32x4.extract_lane 3
          return
        )
        f32.const nan
        return
      )
    )
    "#;

    let store = Store::default();
    let module = Module::new(&store, &module_wat);
    match &module {
        Ok(_) => dbg!("Success!"),
        Err(x) => dbg!(format!("Fail: {}", x)).as_str(),
    };
    let module = module.unwrap();
    // The module doesn't import anything, so we create an empty import object.
    let import_object = imports! {};
    let instance = Instance::new(&module, &import_object).unwrap();

    let f = instance.exports.get_function("f").unwrap();
    let f32_unv = instance.exports.get_function("f32.unv").unwrap();

    let result = f.call(&[Value::F32(3.1415926)]).unwrap();

    let x0 = match result[0] {
        Value::V128(x) => f32_unv.call(&[Value::V128(x), Value::I32(0)]),
        _ => panic!(),
    };

    let x1 = match result[0] {
        Value::V128(x) => f32_unv.call(&[Value::V128(x), Value::I32(1)]),
        _ => panic!(),
    };

    let x2 = match result[0] {
        Value::V128(x) => f32_unv.call(&[Value::V128(x), Value::I32(2)]),
        _ => panic!(),
    };

    let x3 = match result[0] {
        Value::V128(x) => f32_unv.call(&[Value::V128(x), Value::I32(3)]),
        _ => panic!(),
    };

    assert_eq!(x0.unwrap()[0], Value::F32(-3.1415926));
    // https://github.com/WebAssembly/gc-js-customization/blob/8dfabda9b7925543dcb9afe1fda7d5038374dbdd/test/core/float_misc.wast#L647
    assert_eq!(x1.unwrap()[0], Value::F32(3.00));
    assert_eq!(x2.unwrap()[0], Value::F32(3.00));
    assert_eq!(x3.unwrap()[0], Value::F32(4.00));

    ()
}
