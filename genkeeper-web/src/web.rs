use genkeeper_web::*;

#[cfg(target_family = "wasm")]
fn main() {
	logging::wasm::init(logging::wasm::Config::default().prefer_target());
	console_error_panic_hook::set_once();

	let _ = tracing::subscriber::set_global_default({
		use tracing_subscriber::layer::SubscriberExt;
		let layer = tracing_subscriber::Registry::default();
		let layer = layer.with(tracing_wasm::WASMLayer::new(tracing_wasm::WASMLayerConfig::default()));
		let layer = layer.with(tracing_subscriber::filter::filter_fn(|metadata| !metadata.target().starts_with("yew")));
		layer
	});
	yew::Renderer::<App>::new().render();
}

#[cfg(target_family = "windows")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
	Ok(())
}
