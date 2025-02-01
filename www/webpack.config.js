const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  devServer: {
    allowedHosts: "all",
    host: "0.0.0.0"
  },
  experiments: {
    asyncWebAssembly: true
  },
  plugins: [
    new CopyWebpackPlugin(['index.html'])
  ],
};
