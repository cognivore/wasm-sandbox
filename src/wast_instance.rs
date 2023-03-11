use std::{fs::File, io::Write};

use wasmer::{imports, Instance, Module, Store, Value};
use wast;

crate::entry_point!("dump_complex", complex, _EP_GO0);
crate::entry_point!("dump_bytes", go, _EP_GO1);
crate::entry_point!("wast_example", go2, _EP_GO2);
crate::entry_point!("wast2bytes", go_prime, _EP_GO_PRIME);
crate::entry_point!("run_main", main_wrapped, _EP_MAIN_WRAPPED);

pub fn atob(x: &str) -> Vec<u8> {
    let buf = wast::parser::ParseBuffer::new(x).unwrap();
    let mut wat = wast::parser::parse::<wast::Wat>(&buf).unwrap();
    wat.encode().unwrap()
}

pub fn mk(x: &str) -> Instance {
    let bs = atob(x);
    let store = Store::default();
    let module = Module::new(&store, &bs);
    match &module {
        Ok(_) => "",
        Err(x) => dbg!(format!("Fail: {}", x)).as_str(),
    };
    let module = module.unwrap();
    let import_object = imports! {};
    Instance::new(&module, &import_object).unwrap()
}

pub fn run(x: &str, f: &str) -> Box<[wasmer::Val]> {
    let instance = mk(x);
    let phi = instance.exports.get_function(f).unwrap();
    let y = phi.call(&[]);
    y.unwrap()
}

pub fn main(x: &str) -> Box<[wasmer::Val]> {
    run(x, "main")
}

fn main_wrapped(args: Vec<String>) {
    let res = main(&args[0]);
    // Now we print into STDOUT the result of the computation.
    println!("{}", res[0].unwrap_i32());
}

fn complex(_: Vec<String>) {
    let wast = [
        // r#"
        // (module
        //     (func (export "main") (result f32)
        //         (block (result f32) (f32.neg (br 0)))
        //     )
        // )"#,
        r#"
        (module
            (func (export "main") (result f32)
              f32.const 0.42
              return
              f32.neg
            )
        )
        "#,
        r#"
        (module
            (func (export "main")
                (block (br 0))
            )
        )"#,
        r#"
        (module
            (func (export "main")
                (block (drop (f32.neg (br 0))))
            )
        )"#,
        r#"
        (module
            (func (export "main")
                (block
                    br 0
                    f32.neg
                    drop
                )
            )
        )"#,
        r#"
        (module
            (func (export "main") (result f32)
                (block (result f32)
                    f32.const 0.42  ;; First, determine the type of this instruction: [] -> [f32]
                    br 0            ;; Third, determine t^{*} to be [f32], based on the function reduction of block at label,
                                    ;; Fourth, respect the context C through stack-polymorphism. Determine t^{*}_1 to be [], and t^{*}_2 to be [f32].
                                    ;; Finally, determine the concrete type of br 0 for validation purposes to be [f32] -> [f32]
                    f32.neg         ;; Second, determine the type of this instruction: [f32] -> [f32]
                )
            )
        )"#,
    ];

    let mut i = 0;
    for x in wast {
        let b = atob(x);
        let n = b.len();
        main(x);
        let mut f =
            File::create(format!("/tmp/complexwast{i}.{n}.bytes")).expect("Can't create file");
        f.write_all(&b).expect("Can't write file");
        i += 1;
    }
}

fn go(_: Vec<String>) {
    let wast = [
        r#"(module
        (func (export "main_fst") (export "main_snd_")
            (param $x i32)
            (param i32)
            (result i32)

            (i32.add
                (i32.const 1499550000)
                (i32.add (i32.const 9000) (i32.const 17))
            )
        )
    )

    "#,
        r#"(module
        (func (export "two_ints")
            (result i32) (result i32)
            (i32.add
                (i32.const 1499550000)
                (i32.add (i32.const 9000) (i32.const 17))
            )
            (i32.add (i32.const -1) (i32.const 1))
        )
    )
    "#,
        r#"(module
        (func (export "main")
            (result i32)
            (i32.add
                (i32.const 1499550000)
                (i32.add (i32.const 9000) (i32.const 17))
            )
        )
    )
    "#,
        r#"
    (module
        (func (param $x_one i32) (param $three i32) (param $y_one i32) (result i32) (i32.add (i32.const 40) (i32.const 2)))
        (func (param $x_two f32) (param f32) (param f32) (result f32) (local $y_two f32) (f32.add (f32.const 40.0) (f32.const 2.0)))
    )
    "#,
        r#"
    (module
        (func (param $x_one i32) (param $three i32) (param $y_one i32) (result i32) (i32.add (i32.const 40) (i32.const 2)))
        (func (param $x_two f32) (param f32) (param f32) (result i32) (i32.add (i32.const 12) (i32.const 30)))
    )
    "#,
        r#"
    (module
        (func (param $x i32) (param i32) (result i32) (i32.add (i32.const 40) (i32.const 2)))
    )
    "#,
        r#"
    (module
        (func (param $x i32) (param i32) (result i32) (i32.const 42))
    )
    "#,
        r#"
    (module
        (func (param $x i32) (param i32))
    )
    "#,
        r#"
    (module
        (func (param $x i32))
    )
    "#,
        r#"
    (module
        (func)
    )
    "#,
    ];

    for x in wast {
        let b = atob(x);
        let n = b.len();
        let mut f = File::create(format!("/tmp/simplewast.{n}.bytes")).expect("Can't create file");
        f.write_all(&b).expect("Can't write file");
    }
}

// This function is like `go`, but instead it takes the code to be compiled as a string, and then transforms it to binary with atob, finally, counts the sum of the bytes in the original string, adds "." and the length of the original string, adds ".bytes" and writes the file with such name into the current working directory.
pub fn go_prime(args: Vec<String>) {
    // Wast is stored in the 1st argument:
    let wast = &args[0];
    // Calculate the length of the original string
    let wast_len = wast.len();
    // Calculate the sum of the bytes in the original string
    let mut wast_sum_bytes = 0;
    for c in wast.chars() {
        wast_sum_bytes += c as u32;
    }
    // Make the filename
    let filename = format!("./wast-dump-{}L{}.bytes", wast_sum_bytes, wast_len);
    // Convert wast to binary representation
    let b = atob(wast);
    let mut f = File::create(filename).expect("Can't create file");
    f.write_all(&b).expect("Can't write file");
}

pub fn go2(_: Vec<String>) {
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

// Testing https://zulip.yatima.io/#narrow/stream/20-meta/topic/WAST.20pair.20prog/near/28079

#[test]
#[should_panic(
    expected = r#"called `Result::unwrap()` on an `Err` value: Validate("type mismatch: expected v128, found f32 (at offset 28)")"#
)]
fn q11() {
    main(
        r#"(module
            (func $f (param $y f32) (param $p v128) (result f32)
                local.get $y
                f32x4.extract_lane 1
                local.get $p
            )
        )
        "#,
    );
}

#[test]
#[should_panic(
    expected = r#"called `Result::unwrap()` on an `Err` value: Validate("type mismatch: expected v128, found f32 (at offset 28)")"#
)]
fn q12() {
    main(
        r#"(module
            (func $f (param $y f32) (param $p v128) (result f32)
                local.get $y
                f32x4.extract_lane 1 (local.get $p)
            )
        )
        "#,
    );
}

#[test]
#[should_panic(
    expected = r#"called `Result::unwrap()` on an `Err` value: Validate("type mismatch: values remaining on stack at end of block (at offset 33)")"#
)]
fn q13() {
    main(
        r#"(module
            (func $f (param $y f32) (param $p v128) (result f32)
                local.get $y
                (f32x4.extract_lane 1 (local.get $p))
            )
        )
        "#,
    );
}

#[test]
fn q14() {
    main(
        r#"(module
            (func $f (param $y f32) (param $p v128) (result f32)
                local.get $y
                (f32x4.extract_lane 1 (local.get $p))
                f32.add
            )
            (func (export "main") (result f32)
                (call $f (f32.const 0.1) (v128.const f32x4 41.9 0.0 0.0 0.0))
            )
        )
        "#,
    );
}

#[test]
fn q14_0() {
    main(
        r#"(module
            (func $f (param $y f32) (param $p v128) (result f32)
                (local.get $y)
                (f32x4.extract_lane 1 (local.get $p))
                (f32.add)
            )
            (func (export "main") (result f32)
                (call $f (f32.const 0.1) (v128.const f32x4 41.9 0.0 0.0 0.0))
            )
        )
        "#,
    );
}

// Wasm

#[test]
fn q14_1() {
    main(
        r#"(module
            (func $$(param $y f32)(param $p v128)(result f32)local.get $p f32x4.extract_lane 1(local.get $y)f32.add)
            (func (export "main") (result f32)
                (call $$ (f32.const 0.1) (v128.const f32x4 41.9 0.0 0.0 0.0))
            )
        )
        "#,
    );
}

#[test]
fn q14_2() {
    main(
        r#"(module
            (func $f (param $y f32) (param $y1 f32) (result f32)
                local.get $y
                (f32.add (local.get $y1) (local.get $y1))
                f32.add
            )
            (func (export "main") (result f32)
                (call $f (f32.const 0.1) (f32.const 20.95))
            )
        )
        "#,
    );
}

#[test]
fn q14_2_1() {
    let y = main(
        r#"(module $test
            (func)
            (func $f (export "(module (func))") (param $y f32) (param $y1 f32) (result f32)
                (local $dummy i32)
                i32.const 42
                (local.set 2)
                local.get $y1
                (f32.add (local.get $y1))
                local.get $y
                f32.add
            )
            (func (export "main") (result f32)
                (local $x f32) (local $y f32)
                (local.set $x (f32.const 0.1))
                (local.set $y (f32.const 20.95))
                (call $f (local.get $x) (local.get $y))
            )
        )
        "#,
    );
    assert_eq!(y[0], Value::F32(42.0));
}

#[test]
fn q14_2_2() {
    let y = main(
        r#"(module
            (func $f (param $y f32) (param $y1 f32) (result f32)
                local.get $y
                (f32.add (local.get $y1) (local.get $y1))
                (f32.add)
            )
            (func (export "main") (result f32)
                (call $f (f32.const 0.1) (f32.const 20.95))
            )
        )
        "#,
    );
    assert_eq!(y[0], Value::F32(42.0));
}

#[test]
fn q14_2_3() {
    let y = main(
        r#"(module
            (func $f (param $y f32) (param $y1 f32) (result f32)
                (f32.add (local.get $y1) (local.get $y1))
                (f32.add (local.get $y))
            )
            (func (export "main") (result f32)
                (call $f (f32.const 0.1) (f32.const 20.95))
            )
        )
        "#,
    );
    assert_eq!(y[0], Value::F32(42.0));
}

#[test]
#[should_panic(
    expected = r#"called `Result::unwrap()` on an `Err` value: Validate("type mismatch: values remaining on stack at end of block (at offset 48)")"#
)]
fn q14_2_4() {
    let _y = main(
        r#"(module
            (func $f (param $y f32) (param $y1 f32) (result f32)
                (f32.add (local.get $y) (local.get $y) (local.get $y))
            )
            (func (export "main") (result f32)
                (call $f (f32.const 0.1) (f32.const 20.95))
            )
        )
        "#,
    );
    //assert_eq!(y[0], Value::F32(0.1));
}

#[test]
fn q14_2_5() {
    let y = main(
        r#"(module
            (func $f (param $y f32) (param $y1 f32) (result f32)
                (f32.add (local.get $y) local.get $y)
            )
            (func (export "main") (result f32)
                (call $f (f32.const 0.1) (f32.const 20.95))
            )
        )
        "#,
    );
    assert_eq!(y[0], Value::F32(0.2));
}

#[test]
fn q14_2_6() {
    let y = main(
        r#"(module
            (func $f (param $y f32) (param $y1 f32) (result f32)
                (f32.add (f32.const -1.0) local.get $y) ;; 1.0 <~~ goes on top of the stack
                (f32.add local.get $y local.get $y1 local.get $y1) ;; f32.add local.get $y local.get $y1 = 2.0 + 10.0 ;; local.get $y1 = 10.0 <~~ two values go on top of the stack
                f32.add ;; 12.0 + 10.0 <~~ goes on top of the stack
                f32.mul ;; 22.0 * 1.0 <~~ multiplies the two values
            )
            (func (export "main") (result f32)
                (call $f (f32.const 2.0) (f32.const 10.0))
            )
        )
        "#,
    );
    assert_eq!(y[0], Value::F32(22.0));
}

#[test]
#[should_panic(
    expected = r#"called `Result::unwrap()` on an `Err` value: Validate("type mismatch: values remaining on stack at end of block (at offset 57)")"#
)]
fn stack_test() {
    let _y = main(
        r#"(module
            (func $f (param $y f32) (result i32)
                (local $l i32)
                (local $r i32)
                (i32.const 0)
                (local.set $l)
                (i32.const 1)
                (local.set $r)
                (local.get $r)
                (local.get $l)
                (i32.add)
                (local.get $r)
            )
            (func (export "main") (result i32)
                (call $f (f32.const -4.0))
            )
        )
        "#,
    );
}

#[test]
fn many_results() {
    let y = main(
        r#"(module
            (func $f (param $y f32) (param $y1 f32) (result f32) (result i32)
              (local.get $y)
              (i32.const 42)
            )
            (func (export "main") (result f32) (result i32)
                (call $f (f32.const 2.0) (f32.const 10.0))
            )
        )
        "#,
    );
    assert_eq!(y[1], Value::I32(42));
}

#[test]
fn sub() {
    let y = main(
        r#"(module
            (func $f (param $y i32) (result i32)
              (i32.const 42)
              (i32.add)
            )
            (func (export "main") (result i32) (result i32)
                (call $f (i32.const 0))
            )
        )
        "#,
    );
    assert_eq!(y[0], Value::I32(42));
}

#[test]
#[should_panic(
    expected = r#"called `Result::unwrap()` on an `Err` value: Validate("type mismatch: expected i32 but nothing on stack (at offset 41)")"#
)]
fn params_arent_on_stack() {
    let y = main(
        r#"(module
            (func $f (param i32) (param i32) (result i32)
                (i32.add)
            )
            (func (export "main") (result i32)
                (call $f (i32.const 0) (i32.const 42))
            )
        )
        "#,
    );
    assert_eq!(y[0], Value::I32(42));
}

#[test]
fn params_are_locals() {
    let y = main(
        r#"(module
            (func $f (param i32) (param i32) (result i32)
              (local.get 0)
              (local.get 1)
              (i32.add)
            )
            (func (export "main") (result i32)
                (call $f (i32.const 0) (i32.const 42))
            )
        )
        "#,
    );
    assert_eq!(y[0], Value::I32(42));
}
