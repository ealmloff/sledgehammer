#![allow(non_camel_case_types)]

// mod attrs;
pub mod attribute;
pub mod builder;
pub mod element;
pub mod event;
pub mod value;

pub use attribute::*;
pub use builder::*;
pub use element::*;

use wasm_bindgen::prelude::*;
use web_sys::Node;

use web_sys::Element;

#[used]
static mut MSG_PTR: usize = 0;
#[used]
static mut MSG_PTR_PTR: *const usize = unsafe { &MSG_PTR } as *const usize;
#[used]
static mut STR_PTR: usize = 0;
#[used]
static mut STR_PTR_PTR: *const usize = unsafe { &STR_PTR } as *const usize;
#[used]
static mut STR_LEN: usize = 0;
#[used]
static mut STR_LEN_PTR: *const usize = unsafe { &STR_LEN } as *const usize;
static mut ID_SIZE: u8 = 1;

#[wasm_bindgen(module = "/interpreter.js")]
extern "C" {
    fn work_last_created();

    fn update_last_memory(mem: JsValue);

    fn last_needs_memory() -> bool;

    pub type JsInterpreter;

    #[wasm_bindgen(constructor)]
    pub fn new(
        arg: Element,
        mem: JsValue,
        msg_ptr: usize,
        str_ptr: usize,
        str_len_ptr: usize,
    ) -> JsInterpreter;

    #[wasm_bindgen(method)]
    pub fn Work(this: &JsInterpreter);

    #[wasm_bindgen(method)]
    pub fn UpdateMemory(this: &JsInterpreter, mem: JsValue);

    #[wasm_bindgen(method)]
    pub fn NeedsMemory(this: &JsInterpreter) -> bool;

    #[wasm_bindgen(method)]
    pub fn SetNode(this: &JsInterpreter, id: u64, node: Node);
}

fn get_id_size() -> u8 {
    unsafe { ID_SIZE }
}
fn set_id_size(size: u8) {
    unsafe {
        ID_SIZE = size;
    }
}
