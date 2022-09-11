#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

use struct_format::format;
#[derive(format)]
struct kkk{
    a:String,
    b:String
}

#[wasm_bindgen_test]
fn k(){
    assert_eq!(r#"kkk ("s", "f")"#,format!("{}",kkk{a:String::from("s"),b:String::from("f")}));
}
