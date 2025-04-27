
fn main() {
	println!("cargo:rerun-if-changed=build.rs");
	println!("cargo:rerun-if-changed=esbuild.js");
	println!("cargo:rerun-if-changed=package.json");
	println!("cargo:rerun-if-changed=src/tldraw.jsx");
	std::process::Command::new("node").args(["esbuild.js"]).output().expect("failed to build esm module");
}
