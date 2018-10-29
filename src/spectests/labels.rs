// Rust test file autogenerated with cargo build (src/build_spectests.rs).
// Please do NOT modify it by hand, as it will be reseted on next build.
// Test based on spectests/labels.wast
#![allow(
    warnings,
    dead_code
)]
use crate::webassembly::{instantiate, compile, ImportObject, ResultObject, VmCtx, Export};
use super::_common::spectest_importobject;
use wabt::wat2wasm;


// Line 1
fn create_module_1() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func (result i32)))
      (type (;1;) (func (param i32) (result i32)))
      (func (;0;) (type 0) (result i32)
        block (result i32)  ;; label = @1
          i32.const 1
          br 0 (;@1;)
          i32.const 0
        end)
      (func (;1;) (type 0) (result i32)
        (local i32)
        i32.const 0
        set_local 0
        block (result i32)  ;; label = @1
          loop (result i32)  ;; label = @2
            get_local 0
            i32.const 1
            i32.add
            set_local 0
            get_local 0
            i32.const 5
            i32.eq
            if  ;; label = @3
              get_local 0
              br 2 (;@1;)
            end
            br 0 (;@2;)
          end
        end)
      (func (;2;) (type 0) (result i32)
        (local i32)
        i32.const 0
        set_local 0
        block (result i32)  ;; label = @1
          loop (result i32)  ;; label = @2
            get_local 0
            i32.const 1
            i32.add
            set_local 0
            get_local 0
            i32.const 5
            i32.eq
            if  ;; label = @3
              br 1 (;@2;)
            end
            get_local 0
            i32.const 8
            i32.eq
            if  ;; label = @3
              get_local 0
              br 2 (;@1;)
            end
            get_local 0
            i32.const 1
            i32.add
            set_local 0
            br 0 (;@2;)
          end
        end)
      (func (;3;) (type 0) (result i32)
        (local i32)
        i32.const 0
        set_local 0
        block (result i32)  ;; label = @1
          loop (result i32)  ;; label = @2
            get_local 0
            i32.const 1
            i32.add
            set_local 0
            get_local 0
            i32.const 5
            i32.eq
            if  ;; label = @3
              get_local 0
              br 2 (;@1;)
            end
            get_local 0
          end
        end)
      (func (;4;) (type 1) (param i32) (result i32)
        (local i32)
        i32.const 1
        set_local 1
        block (result i32)  ;; label = @1
          loop (result i32)  ;; label = @2
            get_local 1
            get_local 1
            i32.add
            set_local 1
            get_local 1
            get_local 0
            i32.gt_u
            if  ;; label = @3
              get_local 1
              br 2 (;@1;)
            end
            br 0 (;@2;)
          end
        end)
      (func (;5;) (type 0) (result i32)
        loop (result i32)  ;; label = @1
          i32.const 1
        end
        i32.const 1
        i32.add)
      (func (;6;) (type 0) (result i32)
        loop (result i32)  ;; label = @1
          i32.const 0
          br_if 0 (;@1;)
          i32.const 3
        end)
      (func (;7;) (type 0) (result i32)
        (local i32)
        i32.const 0
        set_local 0
        block  ;; label = @1
          i32.const 1
          if  ;; label = @2
            br 0 (;@2;)
            i32.const 666
            set_local 0
          end
          get_local 0
          i32.const 1
          i32.add
          set_local 0
          i32.const 1
          if  ;; label = @2
            br 0 (;@2;)
            i32.const 666
            set_local 0
          else
            i32.const 888
            set_local 0
          end
          get_local 0
          i32.const 1
          i32.add
          set_local 0
          i32.const 1
          if  ;; label = @2
            br 0 (;@2;)
            i32.const 666
            set_local 0
          else
            i32.const 888
            set_local 0
          end
          get_local 0
          i32.const 1
          i32.add
          set_local 0
          i32.const 0
          if  ;; label = @2
            i32.const 888
            set_local 0
          else
            br 0 (;@2;)
            i32.const 666
            set_local 0
          end
          get_local 0
          i32.const 1
          i32.add
          set_local 0
          i32.const 0
          if  ;; label = @2
            i32.const 888
            set_local 0
          else
            br 0 (;@2;)
            i32.const 666
            set_local 0
          end
          get_local 0
          i32.const 1
          i32.add
          set_local 0
        end
        get_local 0)
      (func (;8;) (type 0) (result i32)
        (local i32)
        i32.const 0
        set_local 0
        block  ;; label = @1
          i32.const 1
          if  ;; label = @2
            br 0 (;@2;)
            i32.const 666
            set_local 0
          end
          get_local 0
          i32.const 1
          i32.add
          set_local 0
          i32.const 1
          if  ;; label = @2
            br 0 (;@2;)
            i32.const 666
            set_local 0
          else
            i32.const 888
            set_local 0
          end
          get_local 0
          i32.const 1
          i32.add
          set_local 0
          i32.const 1
          if  ;; label = @2
            br 0 (;@2;)
            i32.const 666
            set_local 0
          else
            i32.const 888
            set_local 0
          end
          get_local 0
          i32.const 1
          i32.add
          set_local 0
          i32.const 0
          if  ;; label = @2
            i32.const 888
            set_local 0
          else
            br 0 (;@2;)
            i32.const 666
            set_local 0
          end
          get_local 0
          i32.const 1
          i32.add
          set_local 0
          i32.const 0
          if  ;; label = @2
            i32.const 888
            set_local 0
          else
            br 0 (;@2;)
            i32.const 666
            set_local 0
          end
          get_local 0
          i32.const 1
          i32.add
          set_local 0
        end
        get_local 0)
      (func (;9;) (type 1) (param i32) (result i32)
        block (result i32)  ;; label = @1
          i32.const 10
          block (result i32)  ;; label = @2
            block  ;; label = @3
              block  ;; label = @4
                block  ;; label = @5
                  block  ;; label = @6
                    block  ;; label = @7
                      get_local 0
                      br_table 4 (;@3;) 0 (;@7;) 1 (;@6;) 2 (;@5;) 3 (;@4;)
                    end
                  end
                  i32.const 2
                  br 3 (;@2;)
                end
                i32.const 3
                br 3 (;@1;)
              end
            end
            i32.const 5
          end
          i32.mul
        end)
      (func (;10;) (type 1) (param i32) (result i32)
        block  ;; label = @1
          block  ;; label = @2
            block  ;; label = @3
              get_local 0
              br_table 0 (;@3;) 1 (;@2;)
              br 2 (;@1;)
            end
            i32.const 0
            return
          end
        end
        i32.const 2)
      (func (;11;) (type 0) (result i32)
        (local i32)
        i32.const 0
        set_local 0
        block (result i32)  ;; label = @1
          block  ;; label = @2
            i32.const 0
            br_if 0 (;@2;)
            get_local 0
            i32.const 1
            i32.or
            set_local 0
            i32.const 1
            br_if 0 (;@2;)
            get_local 0
            i32.const 2
            i32.or
            set_local 0
          end
          block (result i32)  ;; label = @2
            get_local 0
            i32.const 4
            i32.or
            set_local 0
            get_local 0
          end
          i32.const 0
          br_if 0 (;@1;)
          drop
          get_local 0
          i32.const 8
          i32.or
          set_local 0
          block (result i32)  ;; label = @2
            get_local 0
            i32.const 16
            i32.or
            set_local 0
            get_local 0
          end
          i32.const 1
          br_if 0 (;@1;)
          drop
          get_local 0
          i32.const 32
          i32.or
          set_local 0
          get_local 0
        end)
      (func (;12;) (type 0) (result i32)
        block (result i32)  ;; label = @1
          block (result i32)  ;; label = @2
            i32.const 1
            br 0 (;@2;)
          end
          i32.const 1
          br_if 0 (;@1;)
          drop
          i32.const 1
        end)
      (func (;13;) (type 0) (result i32)
        block (result i32)  ;; label = @1
          i32.const 1
          if  ;; label = @2
            block (result i32)  ;; label = @3
              i32.const 1
              br 0 (;@3;)
            end
            br 1 (;@1;)
          end
          i32.const 1
        end)
      (func (;14;) (type 0) (result i32)
        (local i32)
        block (result i32)  ;; label = @1
          block (result i32)  ;; label = @2
            i32.const 1
            set_local 0
            get_local 0
          end
          block (result i32)  ;; label = @2
            i32.const 2
            set_local 0
            get_local 0
          end
          br_if 0 (;@1;)
          drop
          i32.const 0
        end
        i32.const 0
        i32.add
        drop
        get_local 0)
      (func (;15;) (type 0) (result i32)
        block (result i32)  ;; label = @1
          i32.const 1
          if  ;; label = @2
            block (result i32)  ;; label = @3
              i32.const 1
              br 0 (;@3;)
            end
            br 1 (;@1;)
          else
            block  ;; label = @3
              block (result i32)  ;; label = @4
                i32.const 1
                br 0 (;@4;)
              end
              drop
            end
          end
          i32.const 1
        end)
      (func (;16;) (type 0) (result i32)
        block (result i32)  ;; label = @1
          i32.const 1
          br 0 (;@1;)
          i32.const 2
          i32.xor
        end)
      (func (;17;) (type 0) (result i32)
        block (result i32)  ;; label = @1
          block (result i32)  ;; label = @2
            i32.const 2
          end
          block (result i32)  ;; label = @2
            i32.const 3
            br 0 (;@2;)
          end
          i32.add
        end)
      (export \"block\" (func 0))
      (export \"loop1\" (func 1))
      (export \"loop2\" (func 2))
      (export \"loop3\" (func 3))
      (export \"loop4\" (func 4))
      (export \"loop5\" (func 5))
      (export \"loop6\" (func 6))
      (export \"if\" (func 7))
      (export \"if2\" (func 8))
      (export \"switch\" (func 9))
      (export \"return\" (func 10))
      (export \"br_if0\" (func 11))
      (export \"br_if1\" (func 12))
      (export \"br_if2\" (func 13))
      (export \"br_if3\" (func 14))
      (export \"br\" (func 15))
      (export \"shadowing\" (func 16))
      (export \"redefinition\" (func 17)))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

// Line 284
fn l284_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "l284_action_invoke");
    let func_index = match result_object.module.info.exports.get("block") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 1 as i32);
}

// Line 285
fn l285_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "l285_action_invoke");
    let func_index = match result_object.module.info.exports.get("loop1") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 5 as i32);
}

// Line 286
fn l286_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "l286_action_invoke");
    let func_index = match result_object.module.info.exports.get("loop2") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 8 as i32);
}

// Line 287
fn l287_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "l287_action_invoke");
    let func_index = match result_object.module.info.exports.get("loop3") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 1 as i32);
}

// Line 288
fn l288_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "l288_action_invoke");
    let func_index = match result_object.module.info.exports.get("loop4") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i32, &VmCtx) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(8 as i32, &vm_context);
    assert_eq!(result, 16 as i32);
}

// Line 289
fn l289_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "l289_action_invoke");
    let func_index = match result_object.module.info.exports.get("loop5") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 2 as i32);
}

// Line 290
fn l290_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "l290_action_invoke");
    let func_index = match result_object.module.info.exports.get("loop6") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 3 as i32);
}

// Line 291
fn l291_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "l291_action_invoke");
    let func_index = match result_object.module.info.exports.get("if") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 5 as i32);
}

// Line 292
fn l292_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "l292_action_invoke");
    let func_index = match result_object.module.info.exports.get("if2") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 5 as i32);
}

// Line 293
fn l293_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "l293_action_invoke");
    let func_index = match result_object.module.info.exports.get("switch") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i32, &VmCtx) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(0 as i32, &vm_context);
    assert_eq!(result, 50 as i32);
}

// Line 294
fn l294_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "l294_action_invoke");
    let func_index = match result_object.module.info.exports.get("switch") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i32, &VmCtx) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(1 as i32, &vm_context);
    assert_eq!(result, 20 as i32);
}

// Line 295
fn l295_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "l295_action_invoke");
    let func_index = match result_object.module.info.exports.get("switch") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i32, &VmCtx) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(2 as i32, &vm_context);
    assert_eq!(result, 20 as i32);
}

// Line 296
fn l296_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "l296_action_invoke");
    let func_index = match result_object.module.info.exports.get("switch") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i32, &VmCtx) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(3 as i32, &vm_context);
    assert_eq!(result, 3 as i32);
}

// Line 297
fn l297_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "l297_action_invoke");
    let func_index = match result_object.module.info.exports.get("switch") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i32, &VmCtx) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(4 as i32, &vm_context);
    assert_eq!(result, 50 as i32);
}

// Line 298
fn l298_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "l298_action_invoke");
    let func_index = match result_object.module.info.exports.get("switch") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i32, &VmCtx) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(5 as i32, &vm_context);
    assert_eq!(result, 50 as i32);
}

// Line 299
fn l299_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "l299_action_invoke");
    let func_index = match result_object.module.info.exports.get("return") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i32, &VmCtx) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(0 as i32, &vm_context);
    assert_eq!(result, 0 as i32);
}

// Line 300
fn l300_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "l300_action_invoke");
    let func_index = match result_object.module.info.exports.get("return") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i32, &VmCtx) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(1 as i32, &vm_context);
    assert_eq!(result, 2 as i32);
}

// Line 301
fn l301_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "l301_action_invoke");
    let func_index = match result_object.module.info.exports.get("return") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i32, &VmCtx) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(2 as i32, &vm_context);
    assert_eq!(result, 2 as i32);
}

// Line 302
fn l302_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "l302_action_invoke");
    let func_index = match result_object.module.info.exports.get("br_if0") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 29 as i32);
}

// Line 303
fn l303_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "l303_action_invoke");
    let func_index = match result_object.module.info.exports.get("br_if1") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 1 as i32);
}

// Line 304
fn l304_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "l304_action_invoke");
    let func_index = match result_object.module.info.exports.get("br_if2") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 1 as i32);
}

// Line 305
fn l305_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "l305_action_invoke");
    let func_index = match result_object.module.info.exports.get("br_if3") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 2 as i32);
}

// Line 306
fn l306_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "l306_action_invoke");
    let func_index = match result_object.module.info.exports.get("br") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 1 as i32);
}

// Line 307
fn l307_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "l307_action_invoke");
    let func_index = match result_object.module.info.exports.get("shadowing") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 1 as i32);
}

// Line 308
fn l308_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "l308_action_invoke");
    let func_index = match result_object.module.info.exports.get("redefinition") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 5 as i32);
}

// Line 311
#[test]
fn l311_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 13, 1, 11, 0, 2, 64, 65, 1, 13, 0, 140, 1, 11, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 315
#[test]
fn l315_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 16, 1, 14, 0, 2, 64, 67, 0, 0, 0, 0, 65, 1, 13, 0, 11, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 319
#[test]
fn l319_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 16, 1, 14, 0, 2, 64, 67, 0, 0, 0, 0, 65, 1, 13, 0, 11, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

#[test]
fn test_module_1() {
    let result_object = create_module_1();
    let vm_context = result_object.instance.generate_context();
    // We group the calls together
    l284_action_invoke(&result_object, &vm_context);
    l285_action_invoke(&result_object, &vm_context);
    l286_action_invoke(&result_object, &vm_context);
    l287_action_invoke(&result_object, &vm_context);
    l288_action_invoke(&result_object, &vm_context);
    l289_action_invoke(&result_object, &vm_context);
    l290_action_invoke(&result_object, &vm_context);
    l291_action_invoke(&result_object, &vm_context);
    l292_action_invoke(&result_object, &vm_context);
    l293_action_invoke(&result_object, &vm_context);
    l294_action_invoke(&result_object, &vm_context);
    l295_action_invoke(&result_object, &vm_context);
    l296_action_invoke(&result_object, &vm_context);
    l297_action_invoke(&result_object, &vm_context);
    l298_action_invoke(&result_object, &vm_context);
    l299_action_invoke(&result_object, &vm_context);
    l300_action_invoke(&result_object, &vm_context);
    l301_action_invoke(&result_object, &vm_context);
    l302_action_invoke(&result_object, &vm_context);
    l303_action_invoke(&result_object, &vm_context);
    l304_action_invoke(&result_object, &vm_context);
    l305_action_invoke(&result_object, &vm_context);
    l306_action_invoke(&result_object, &vm_context);
    l307_action_invoke(&result_object, &vm_context);
    l308_action_invoke(&result_object, &vm_context);
}
