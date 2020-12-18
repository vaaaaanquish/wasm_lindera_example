mod utils;
use std::os::raw::c_char;
use wasm_bindgen::prelude::*;
use lindera::tokenizer::Tokenizer;
use lindera_core::core::viterbi::Mode;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn tokenize(x: Option<String>) -> String {
    let mut tokenizer = Tokenizer::new(Mode::Normal, "");
    let result = tokenizer.tokenize(&x.unwrap()).iter().map(|x| x.text).collect::<Vec<&str>>().join(" ");
    result.into()
}
