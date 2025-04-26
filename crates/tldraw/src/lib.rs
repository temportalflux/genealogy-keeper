use serde::Serialize;
use yew::prelude::*;

// https://stackoverflow.com/questions/75422119/using-npm-packages-with-rust-and-webassembly
// https://tldraw.dev/quick-start
// https://tldraw.dev/reference

mod bindings;

/*
#[wasm_bindgen]
pub fn my_format(date: &Date, format_string: &str) -> String {
	Dateformat(date, format_string)
}
*/

#[derive(Clone, PartialEq, Properties, Serialize)]
pub struct TlDrawProps {

}

#[function_component]
pub fn TlDraw(props: &TlDrawProps) -> Html {
	//let output = bindings::Tldraw::new();
	bindings::Tldraw(serde_wasm_bindgen::to_value(&props).unwrap());
	//log::debug!(target: "tldraw", "{output:?}");
	html!()
}
