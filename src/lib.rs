use rayon::prelude::*;
use wasm_bindgen::prelude::*;
pub use wasm_bindgen_rayon::init_thread_pool;

#[wasm_bindgen]
pub fn hello(mut v: Vec<i32>) -> Vec<i32> {
    v.par_iter_mut().for_each(|x| *x = *x * 3);
    v
}
