use tldraw;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

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

// TODO: Next - explore ways to customize the whiteboard https://tldraw.dev/examples/basic/remove-tool

#[function_component]
fn App() -> Html {
	html! {<>
		{"Genealogy Keeper: Desktop"}
		<tldraw::Tldraw
			infer_dark_mode={true}
			components={tldraw::ComponentProps {
				interface: tldraw::InterfaceComponentProps {
					style_panel: tldraw::ComponentConfig::Remove,
					..Default::default()
				},
				..Default::default()
			}}
			overrides={tldraw::Overrides {
				tools: [
					(tldraw::ToolKind::Draw, tldraw::ToolInit::Remove),
					(tldraw::ToolKind::Text, tldraw::ToolInit::Remove),
				].into(),
			}}
		/>
	</>}
}
