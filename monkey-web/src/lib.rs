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
        let monkey::InterpreterResult { stdout, result } = self.0.evaluate(s);

        // TODO figure out how to pass structs to JS instead of using this hacky delimiter.
        match result {
            Ok(x) => Ok(format!("{}|{}", stdout, x)),
            Err(x) => Err(JsValue::from_str(&format!("{}|{}", stdout, x))),
        }
    }
}
