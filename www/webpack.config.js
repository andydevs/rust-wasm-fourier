const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  devtool: 'inline-source-map',
  devServer: {
    allowedHosts: "all",
    host: "0.0.0.0",
    port: 8080,
    client: {
      webSocketURL: 'ws://0.0.0.0:80/ws',
    }
  },
  experiments: {
    asyncWebAssembly: true
  },
  plugins: [
    new CopyWebpackPlugin(['index.html'])
  ],
};
