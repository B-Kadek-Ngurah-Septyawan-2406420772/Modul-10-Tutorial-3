const path = require('path');
const CopyWebpackPlugin = require('copy-webpack-plugin');

const distPath = path.resolve(__dirname, 'dist');

module.exports = {
    mode: 'production',
    devServer: {
        port: 8000,
        client: {
            overlay: {
                warnings: false,
                errors: true,
            },
        },
    },
    entry: './bootstrap.js',
    output: {
        path: distPath,
        filename: 'yewchat.js',
        webassemblyModuleFilename: 'yewchat_bg.wasm',
    },
    plugins: [
        new CopyWebpackPlugin({
            patterns: [{ from: './static', to: distPath }],
        }),
    ],
    experiments: {
        asyncWebAssembly: true,
    },
    performance: {
        hints: false,
    },
};
