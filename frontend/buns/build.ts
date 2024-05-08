// IMPORTANT: Should be ran from `frontend/` dir
await Bun.build({
	entrypoints: ['src-ui/src/index.ts'],
	outdir: './dist-ui/js',
	naming: 'app-bundle-bun.js',
	sourcemap: 'external'
});


// Support for build watch: https://github.com/oven-sh/bun/issues/5866
// const srcWatcher = watch(
// 	`${import.meta.dir}/src`,
// 	{ recursive: true },
// 	(event: any, filename: any) => {
// 		Bun.build({
// 			entrypoints: ["./src/entry.ts"],
// 			outdir: "./dist",
// 		});

// 		console.log(`Detected ${event} in ${filename} (src)`);
// 	}
// );

// process.on("SIGINT", () => {
// 	srcWatcher.close();
// 	process.exit(0);
// });
