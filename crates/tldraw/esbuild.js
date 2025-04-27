import * as esbuild from 'esbuild'

await esbuild.build({
	entryPoints: ['tldraw.jsx'],
	bundle: true,
	outfile: 'bindings/tldraw.js',
	format: 'esm',
	minify: false,
}).catch(() => process.exit(1));
