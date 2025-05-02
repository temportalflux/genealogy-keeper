// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Listener, Manager};
use tauri_plugin_log::{Target, TargetKind};

static LOG_GENKEEPER: &'static str = "genkeeper";

fn main() -> anyhow::Result<()> {
	tauri::Builder::default()
		.plugin(
			tauri_plugin_log::Builder::default()
				.clear_targets()
				.targets([
					Target::new(TargetKind::Stdout),
					// TODO: files are never truncated
					match cfg!(debug_assertions) {
						true => Target::new(TargetKind::Folder { path: std::env::current_dir()?, file_name: Some("GenKeeper".into()) }),
						false => Target::new(TargetKind::LogDir { file_name: Some("GenKeeper".into()) }),
					},
					Target::new(TargetKind::Webview),
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

			log::info!(target: LOG_GENKEEPER, "CWD: {:?}", std::env::current_dir());
			log::info!(target: LOG_GENKEEPER, "Local Data: {:?}", app.path().app_local_data_dir());

			// TODO: in all runtimes, scan `app_local_data_dir/projects`
			if cfg!(debug_assertions) {
				spawn(LOG_GENKEEPER, load_local_projects(app.handle().clone()));
			}

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

// Loads projects on the local filesystem from the current working directory in debug builds only.
// Helpful tooling to increase iteration time, where projects can be stored in the workspace instead of local_app_data.
#[cfg(debug_assertions)]
async fn load_local_projects(_app: tauri::AppHandle) -> anyhow::Result<()> {
	use std::io::BufRead;
	let cwd = std::env::current_dir().expect("missing current working directory");
	let local_projects_path = cwd.join("local_projects");
	let reader = std::io::BufReader::new(std::fs::File::open(local_projects_path)?);
	for line in reader.lines().map_while(Result::ok) {
		if line.is_empty() {
			continue;
		}
		let Ok(project_path) = cwd.join(&line).canonicalize() else {
			log::error!(target: LOG_GENKEEPER, "Local projects provides {line:?}, but there is no directory at that location.");
			continue;
		};
		log::info!(target: LOG_GENKEEPER, "Found local project at {}", project_path.display());
	}
	Ok(())
}
