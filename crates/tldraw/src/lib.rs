use derivative::Derivative;
use serde::{Deserialize, Serialize};
use web_sys::js_sys;
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
	($srcfile:expr, $module_name:ident, $type_name:ident, $props_name:ty) => {
		mod $module_name {
			use wasm_bindgen::prelude::*;
			use yew::prelude::*;

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
				type Properties = $props_name;

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

mod inner {
	use super::*;

	#[derive(Clone, Debug, PartialEq, Serialize)]
	pub struct TLComponents {
		#[serde(rename = "StylePanel",
			skip_serializing_if = "wasm_bindgen::JsValue::is_undefined",
			with = "serde_wasm_bindgen::preserve"
		)]
		pub style_panel: wasm_bindgen::JsValue,
		#[serde(rename = "PageMenu",
			skip_serializing_if = "wasm_bindgen::JsValue::is_undefined",
			with = "serde_wasm_bindgen::preserve"
		)]
		pub page_menu: wasm_bindgen::JsValue,
	}

	#[derive(Clone, PartialEq, Serialize)]
	pub struct TLUiOverrides {
		#[serde(with = "serde_wasm_bindgen::preserve")]
		pub tools: js_sys::Function,
	}

	#[derive(Clone, PartialEq, Properties, Serialize)]
	pub struct TldrawProps {
		#[prop_or_default]
		#[serde(rename = "inferDarkMode")]
		pub infer_dark_mode: bool,
		pub components: TLComponents,
		pub overrides: TLUiOverrides,
	}
}
component_binding!("/bindings/tldraw.js", tldraw_comp, Tldraw, crate::inner::TldrawProps);

#[derive(Clone, PartialEq)]
pub enum ToolInit {
	Default,
	Remove,
	Change(yew::Callback<(Editor, ToolItem, ToolsHelpers), ToolItem>),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Overrides {
	#[prop_or_default]
	pub tools: std::collections::HashMap<ToolKind, ToolInit>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
	#[prop_or_default]
	pub infer_dark_mode: bool,
	#[prop_or_default]
	pub components: ComponentProps,
	#[prop_or_default]
	pub overrides: Option<Overrides>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ToolKind {
	#[serde(rename = "select")]
	Select,
	#[serde(rename = "hand")]
	Hand,
	#[serde(rename = "eraser")]
	Eraser,
	#[serde(rename = "draw")]
	Draw,
	#[serde(rename = "arrow")]
	Arrow,
	#[serde(rename = "line")]
	Line,
	#[serde(rename = "frame")]
	Frame,
	#[serde(rename = "text")]
	Text,
	#[serde(rename = "asset")]
	Asset,
	#[serde(rename = "note")]
	Note,
	#[serde(rename = "laser")]
	Laser,
	#[serde(rename = "embed")]
	Embed,
	#[serde(rename = "highlight")]
	Highlight,
	#[serde(untagged)]
	Shape(GeometricShape),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum GeometricShape {
	#[serde(rename = "cloud")]
	Cloud,
	#[serde(rename = "rectangle")]
	Rectangle,
	#[serde(rename = "ellipse")]
	Ellipse,
	#[serde(rename = "triangle")]
	Triangle,
	#[serde(rename = "diamond")]
	Diamond,
	#[serde(rename = "pentagon")]
	Pentagon,
	#[serde(rename = "hexagon")]
	Hexagon,
	#[serde(rename = "octagon")]
	Octagon,
	#[serde(rename = "star")]
	Star,
	#[serde(rename = "rhombus")]
	Rhombus,
	#[serde(rename = "rhombus-2")]
	Rhombus2,
	#[serde(rename = "oval")]
	Oval,
	#[serde(rename = "trapezoid")]
	Trapezoid,
	#[serde(rename = "arrow-right")]
	ArrowRight,
	#[serde(rename = "arrow-left")]
	ArrowLeft,
	#[serde(rename = "arrow-up")]
	ArrowUp,
	#[serde(rename = "arrow-down")]
	ArrowDown,
	#[serde(rename = "x-box")]
	XBox,
	#[serde(rename = "check-box")]
	CheckBox,
	#[serde(rename = "heart")]
	Heart,
}
// https://github.com/tldraw/tldraw/blob/v3.13.0/packages/tldraw/src/lib/ui/hooks/useTools.tsx#L12
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolItem {
	pub id: String,
	pub label: String,
	#[serde(rename = "shortcutsLabel")]
	pub shortcuts_label: Option<String>,
	pub icon: String,
	#[serde(rename = "onSelect", with = "serde_wasm_bindgen::preserve")]
	pub on_select: wasm_bindgen::JsValue,
	#[serde(rename = "kbd")]
	pub keyboard_shortcut: Option<String>,
	#[serde(rename = "readonlyOk")]
	pub readonly_accessible: Option<bool>,
	#[serde(default, with = "serde_wasm_bindgen::preserve")]
	pub meta: wasm_bindgen::JsValue,
}

#[derive(Clone, Default, PartialEq, Properties)]
pub struct ComponentProps {
	#[prop_or_default]
	pub editor: EditorComponentProps,
	#[prop_or_default]
	pub interface: InterfaceComponentProps,
}

#[derive(Clone, PartialEq, Derivative)]
#[derivative(Default)]
pub enum ComponentConfig {
	#[derivative(Default)]
	Default,
	Remove,
	Replace(/*TODO: React.ComponentType<TProps>*/ Callback<(), Html>),
}
impl Into<wasm_bindgen::JsValue> for &ComponentConfig {
	fn into(self) -> wasm_bindgen::JsValue {
		match self {
			ComponentConfig::Default => wasm_bindgen::JsValue::undefined(),
			ComponentConfig::Remove => wasm_bindgen::JsValue::null(),
			ComponentConfig::Replace(callback) => {
				type Closure = wasm_bindgen::prelude::Closure::<dyn Fn(wasm_bindgen::JsValue) -> wasm_bindgen::JsValue>;
				let callback = callback.clone();
				let closure = Closure::new(move |props_js: wasm_bindgen::JsValue| {
					let _html = callback.emit(());
					// TODO: Convert html to JsValue so it can be handed off to tldraw
					wasm_bindgen::JsValue::undefined()
				});
				closure.into_js_value()
			}
		}
	}
}

// TLEditorComponents
#[derive(Clone, Default, PartialEq, Properties)]
pub struct EditorComponentProps {
/*
	Background?: ComponentType | null
	Brush?: ComponentType<TLBrushProps> | null
	Canvas?: ComponentType<TLCanvasComponentProps> | null
	CollaboratorBrush?: ComponentType<TLBrushProps> | null
	CollaboratorCursor?: ComponentType<TLCursorProps> | null
	CollaboratorHint?: ComponentType<TLCollaboratorHintProps> | null
	CollaboratorScribble?: ComponentType<TLScribbleProps> | null
	CollaboratorShapeIndicator?: ComponentType<TLShapeIndicatorProps> | null
	Cursor?: ComponentType<TLCursorProps> | null
	Grid?: ComponentType<TLGridProps> | null
	Handle?: ComponentType<TLHandleProps> | null
	Handles?: ComponentType<TLHandlesProps> | null
	InFrontOfTheCanvas?: ComponentType | null
	LoadingScreen?: ComponentType | null
	OnTheCanvas?: ComponentType | null
	Overlays?: ComponentType | null
	Scribble?: ComponentType<TLScribbleProps> | null
	SelectionBackground?: ComponentType<TLSelectionBackgroundProps> | null
	SelectionForeground?: ComponentType<TLSelectionForegroundProps> | null
	ShapeIndicator?: ComponentType<TLShapeIndicatorProps> | null
	ShapeIndicators?: ComponentType | null
	SnapIndicator?: ComponentType<TLSnapIndicatorProps> | null
	Spinner?: ComponentType | null
	SvgDefs?: ComponentType | null
	ZoomBrush?: ComponentType<TLBrushProps> | null
	// These will always have defaults
	ErrorFallback?: TLErrorFallbackComponent
	ShapeErrorFallback?: TLShapeErrorFallbackComponent
	ShapeIndicatorErrorFallback?: TLShapeIndicatorErrorFallbackComponent
*/
}
// TLUiComponents
#[derive(Clone, Default, PartialEq, Properties)]
pub struct InterfaceComponentProps {
	#[prop_or_default]
	pub style_panel: ComponentConfig,
	#[prop_or_default]
	pub page_menu: ComponentConfig,
/*
	ContextMenu?: ComponentType<TLUiContextMenuProps> | null
	ActionsMenu?: ComponentType<TLUiActionsMenuProps> | null
	HelpMenu?: ComponentType<TLUiHelpMenuProps> | null
	ZoomMenu?: ComponentType<TLUiZoomMenuProps> | null
	MainMenu?: ComponentType<TLUiMainMenuProps> | null
	Minimap?: ComponentType | null
	StylePanel?: ComponentType<TLUiStylePanelProps> | null
	PageMenu?: ComponentType | null
	NavigationPanel?: ComponentType | null
	Toolbar?: ComponentType | null
	RichTextToolbar?: ComponentType<TLUiRichTextToolbarProps> | null
	KeyboardShortcutsDialog?: ComponentType<TLUiKeyboardShortcutsDialogProps> | null
	QuickActions?: ComponentType<TLUiQuickActionsProps> | null
	HelperButtons?: ComponentType<TLUiHelperButtonsProps> | null
	DebugPanel?: ComponentType | null
	DebugMenu?: ComponentType | null
	MenuPanel?: ComponentType | null
	TopPanel?: ComponentType | null
	SharePanel?: ComponentType | null
	CursorChatBubble?: ComponentType | null
	Dialogs?: ComponentType | null
	Toasts?: ComponentType | null
	A11y?: ComponentType | null
*/
}

#[derive(Clone, Deserialize)]
pub struct Editor;
pub type ToolsContext = std::collections::HashMap<ToolKind, ToolItem>;
#[derive(Clone, Deserialize)]
pub struct ToolsHelpers;

type FnOverrideTools =
	dyn Fn(wasm_bindgen::JsValue, wasm_bindgen::JsValue, wasm_bindgen::JsValue) -> wasm_bindgen::JsValue;

#[function_component]
pub fn Tldraw(props: &Props) -> Html {
	use wasm_bindgen::{JsCast, closure::Closure};

	let components = {
		inner::TLComponents {
			style_panel: (&props.components.interface.style_panel).into(),
			page_menu: (&props.components.interface.page_menu).into(),
		}
	};
	log::debug!("{components:?}");

	let tool_changes = {
		let config = props.overrides.as_ref();
		config.map(|overrides| overrides.tools.clone())
	};

	let tools = use_memo((), move |_| Closure::<FnOverrideTools>::new(
		move |editor_js, context_js: wasm_bindgen::JsValue, helpers_js| {
			let Some(tool_config) = &tool_changes else { return context_js };
			//let Some(callback) = &callback else { return context_js };
			// log::debug!("{:?}", context_js);
			let editor = Editor; // serde_wasm_bindgen::from_value(editor_js).expect("failed to parse editor_js");
			//let context =
			//	serde_wasm_bindgen::from_value::<ToolsContext>(context_js).expect("failed to parse context_js");
			let helpers = ToolsHelpers; // serde_wasm_bindgen::from_value(helpers_js).expect("failed to parse context_js");
			
			let mut context_js = context_js;
			for (kind, init) in tool_config {
				let key = serde_wasm_bindgen::to_value(&kind).unwrap();
				match init {
					ToolInit::Default => continue,
					ToolInit::Remove => {
						let Some(context_obj) = context_js.dyn_ref::<js_sys::Object>() else { continue };
						let _ = js_sys::Reflect::delete_property(context_obj, &key);
						context_js = context_obj.into();
					}
					ToolInit::Change(callback) => {
						let Ok(tool_item_js) = js_sys::Reflect::get(&context_js, &key) else { continue };
						let Ok(tool_item) = serde_wasm_bindgen::from_value::<ToolItem>(tool_item_js) else { continue };
						let tool_item = callback.emit((editor.clone(), tool_item, helpers.clone()));
						let Ok(tool_item_js) = serde_wasm_bindgen::to_value(&tool_item) else { continue };
						let _ = js_sys::Reflect::set(&context_js, &key, &tool_item_js);
					}
				}
			}
			//log::debug!("{:?}", context);
			context_js
		}
	));

	html!(<tldraw_comp::Tldraw
		infer_dark_mode={props.infer_dark_mode}
		{components}
		overrides={inner::TLUiOverrides {
			tools: (*tools).as_ref().unchecked_ref::<js_sys::Function>().clone(),
		}}
	/>)
}
