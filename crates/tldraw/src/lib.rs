use serde::Serialize;
use yew::prelude::*;

// https://stackoverflow.com/questions/75422119/using-npm-packages-with-rust-and-webassembly
// https://tldraw.dev/quick-start
// https://tldraw.dev/reference

pub mod bindings;

#[derive(Clone, PartialEq, Properties, Serialize)]
pub struct TldrawProps {

}

// Based on the proc-macro library written at https://github.com/hobofan/reacty_yew/blob/main/reacty_yew/src/lib.rs
pub struct Tldraw(web_sys::Node);
impl yew::Component for Tldraw {
	type Message = ();
	type Properties = TldrawProps;

	fn create(_ctx: &Context<Self>) -> Self {
		// The html doc node/element that the javascript html element (potentially react component)
		// will be embedded into.
		let node = web_sys::Node::from({
			let window = web_sys::window().unwrap();
			let document = window.document().unwrap();
			let element = document.create_element("div").unwrap();
			let _ = element.set_attribute("style", "flex-grow: 1;");
			element
		});
		Self(node)
	}

	fn view(&self, ctx: &Context<Self>) -> Html {
		let props_value = serde_wasm_bindgen::to_value(ctx.props()).unwrap();
		bindings::createElementWithin_Tldraw(self.0.clone(), props_value);
		yew::virtual_dom::VNode::VRef(self.0.clone())
	}
}
