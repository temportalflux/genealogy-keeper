use wasm_bindgen::prelude::*;
use yew::prelude::*;
use tldraw;

mod logging;
pub mod utility;

#[wasm_bindgen(module = "/glue.js")]
extern "C" {
	#[wasm_bindgen(js_name = isBound)]
	pub fn is_bound() -> bool;
}

#[cfg(target_family = "wasm")]
fn main() {
	if false {
	//if is_bound() {
		let _ = ::log::set_boxed_logger(Box::new(logging::tauri::Logger));
		::log::set_max_level(log::LevelFilter::Trace);
	} else {
		use logging::wasm::*;
		init(Config::default().prefer_target());
	}
	yew::Renderer::<App>::new().render();
}

#[cfg(target_family = "windows")]
fn main() {}

#[function_component]
fn App() -> Html {
	html! {<>
		{"GenFlux Desktop"}
		<tldraw::Tldraw />
	</>}
}
