

fn main() {
	println!("cargo:rerun-if-changed=package.jsx");
	// TODO: execute `npm run esbuild` or `node esbuild.js`
}
