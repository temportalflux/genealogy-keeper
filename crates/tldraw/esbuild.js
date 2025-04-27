import * as esbuild from 'esbuild'

await esbuild.build({
	entryPoints: ['package.jsx'],
	bundle: true,
	outfile: 'src/tldraw.js',
	format: 'esm',
	minify: false,
}).catch(() => process.exit(1));
