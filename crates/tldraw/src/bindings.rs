use wasm_bindgen::prelude::*;

#[allow(long_running_const_eval)]
#[wasm_bindgen(module = "/src/package.js")]
extern "C" {
	#[wasm_bindgen]
	pub fn Tldraw(props: JsValue);
}
