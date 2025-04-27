use serde::Serialize;
use yew::prelude::*;

// https://stackoverflow.com/questions/75422119/using-npm-packages-with-rust-and-webassembly
// https://tldraw.dev/quick-start
// https://tldraw.dev/reference

// The html doc node/element that the javascript html element (potentially react component) will be embedded into.
fn create_component_root() -> web_sys::Node {
	web_sys::Node::from({
		let window = web_sys::window().unwrap();
		let document = window.document().unwrap();
		let element = document.create_element("div").unwrap();
		let _ = element.set_attribute("style", "flex-grow: 1;");
		element
	})
}

// Based on the proc-macro library written at https://github.com/hobofan/reacty_yew/blob/main/reacty_yew/src/lib.rs
macro_rules! component_binding {
	($srcfile:expr, $module_name:ident, $type_name:ident, $props_name:ident) => {
		mod $module_name {
			use yew::prelude::*;
			use wasm_bindgen::prelude::*;
		
			paste::paste! {
				#[wasm_bindgen(module = $srcfile)]
				extern "C" {
					#[wasm_bindgen]
					fn [<createElementWithin_ $type_name>](node: web_sys::Node, props: JsValue);
				}
			}
		
			pub struct $type_name(web_sys::Node);
			impl yew::Component for $type_name {
				type Message = ();
				type Properties = $crate::$props_name;
			
				fn create(_ctx: &Context<Self>) -> Self {
					Self($crate::create_component_root())
				}
			
				fn view(&self, ctx: &Context<Self>) -> Html {
					let props_value = serde_wasm_bindgen::to_value(ctx.props()).unwrap();
					paste::paste! {
						[<createElementWithin_ $type_name>](self.0.clone(), props_value);
					}
					yew::virtual_dom::VNode::VRef(self.0.clone())
				}
			}
		}
	};
}

#[derive(Clone, PartialEq, Properties, Serialize)]
pub struct TldrawProps {
	#[prop_or_default]
	#[serde(rename="inferDarkMode")]
	pub infer_dark_mode: bool,
}
component_binding!("/bindings/tldraw.js", tldraw_comp, Tldraw, TldrawProps);
pub use tldraw_comp::Tldraw;
