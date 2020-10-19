(async () => {
    const {
        startService,
        build,
    } = require("esbuild")
    const service = await startService()

    try {
        const res = await service.build({
            entryPoints: ["./src/bind.ts"],
            outfile: './src/katex.js',
            format: 'cjs',
            minify: true,
            bundle: true,
        })
    } finally {
        service.stop()
    }
})()
