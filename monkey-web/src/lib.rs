extern crate cfg_if;
extern crate monkey;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub struct Interpreter(monkey::Interpreter);

#[wasm_bindgen]
impl Interpreter {
    pub fn new() -> Self {
        Interpreter(monkey::Interpreter::new())
    }

    pub fn evaluate(&mut self, s: String) -> Result<String, JsValue> {
        match self.0.evaluate(s) {
            Ok(x) => Ok(format!("{}", x)),
            Err(x) => Err(JsValue::from_str(&format!("{}", x))),
        }
    }
}
