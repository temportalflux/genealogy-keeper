use wasm_bindgen::prelude::*;

#[allow(long_running_const_eval)]
#[wasm_bindgen(module = "/src/tldraw.js")]
extern "C" {
	#[wasm_bindgen]
	pub fn createElementWithin_Tldraw(node: web_sys::Node, props: JsValue);
}
