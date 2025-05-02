// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Listener, Manager};
use tauri_plugin_log::{Target, TargetKind};

fn main() -> anyhow::Result<()> {
	tauri::Builder::default()
		.plugin(
			tauri_plugin_log::Builder::default()
				.targets([
					Target::new(TargetKind::Stdout),
					Target::new(TargetKind::LogDir { file_name: Some("GenKeeper".into()) }),
					Target::new(TargetKind::Webview)
				])
				.filter(|record| {
					static IGNORED_TARGETS: [&'static str; 2] = ["hyper_util", "tao"];
					for ignored in IGNORED_TARGETS {
						if record.target().contains(ignored) {
							return false;
						}
					}
					true
				})
				.build(),
		)
		.plugin(tauri_plugin_positioner::init())
		.plugin(tauri_plugin_clipboard::init())
		.setup(|app| {
			// Listen for logging from the frontend
			app.listen("log", |event| {
				let Ok(record) = serde_json::from_str::<shared::log::LogRecord>(event.payload()) else {
					return;
				};
				log::log!(target: record.target.as_str(), record.level, "{}", record.args);
			});
			// Wait for the frontend to become ready
			app.listen("ready", {
				move |_| {
					log::info!("received ready event from frontened");
				}
			});

			log::info!(target: "genkeeper", "CWD: {:?}", std::env::current_dir());
			log::info!(target: "genkeeper", "Local Data: {:?}", app.path().app_local_data_dir());

			Ok(())
		})
		.run(tauri::generate_context!())?;
	Ok(())
}

pub fn spawn<F, E>(target: &'static str, future: F)
where
	F: futures::Future<Output = Result<(), E>> + 'static + Send,
	E: 'static + std::fmt::Debug,
{
	tauri::async_runtime::spawn(async move {
		let Err(err) = future.await else { return };
		log::error!(target: target, "{err:?}");
	});
}
