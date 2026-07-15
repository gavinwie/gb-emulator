// In wasm/src/lib.rs

use gb_core::cpu::Cpu;

use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
pub struct GB {
    cpu: Cpu,
    ctx: CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl GB {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<GB, JsValue> {
        let cpu = Cpu::new();

        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let ctx = canvas.get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let gb = GB { cpu, ctx };
        Ok(gb)
    }

    #[wasm_bindgen]
    pub fn load_rom(&mut self, data: Uint8Array) {
        let mut rom: Vec<u8> = Vec::new();

        for i in 0..data.byte_length() {
            rom.push(data.get_index(i));
        }
        self.cpu.load_rom(&rom);
    }
}