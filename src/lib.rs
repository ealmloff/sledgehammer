#![allow(non_camel_case_types)]

// mod attrs;
mod attribute;
mod builder;
mod element;
mod event;
mod value;

pub use attribute::*;
pub use builder::*;
pub use element::*;

use dioxus_interpreter_js::Interpreter;

use easybench_wasm::bench as wasm_bench;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{console, Document, HtmlHeadElement, Node, Performance};

static mut PTR: u32 = 0;
static mut PTR_PTR: *const u32 = unsafe { &PTR } as *const u32;
static mut LEN_PTR: u32 = 0;
static mut LEN_PTR_PTR: *const u32 = unsafe { &LEN_PTR } as *const u32;

#[wasm_bindgen(module = "/interpreter.js")]
extern "C" {
    fn interperter_init(mem: JsValue, ptr: u32, size: u32);

    #[wasm_bindgen(js_name = "work")]
    pub fn work_inner();

    pub fn set_node(id: u64, node: Node);

    pub fn bench(modifications: usize) -> f64;

    pub fn bench_template();

    pub fn prep();
}

pub fn work(data: &[u8]) {
    let ptr = data.as_ptr();
    let len = data.len();
    unsafe {
        PTR = ptr as u32;
        LEN_PTR = len as u32
    };
    work_inner();

    let _ = data;
}

pub fn init() {
    unsafe {
        interperter_init(wasm_bindgen::memory(), PTR_PTR as u32, LEN_PTR_PTR as u32);
    }
}
