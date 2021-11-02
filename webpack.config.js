const HtmlWebpackPlugin = require("html-webpack-plugin");
const path = require("path");

module.exports = (env, args) => {
  const isProduction = args.mode === "production";

  return {
    entry: "./frontend/bootstrap.js",
    output: {
      path: path.resolve(__dirname, "dist"),
      filename: isProduction ? "[name].[contenthash].js" : "[name].[hash].js",
    },
    module: {
      rules: [
        {
          test: /\.m?jss$/,
          exclude: /node_modules/,
          use: {
            loader: "swc-loader",
          },
        },
        {
          test: /\.ts$/,
          exclude: /node_modules/,
          use: {
            loader: "swc-loader",
            options: {
              jsc: {
                parser: {
                  syntax: "typescript",
                },
              },
            },
          },
        },
        {
          test: /\.html$/,
          use: [
            {
              loader: "html-loader",
              options: { minimize: true },
            },
          ],
        },
        {
          test: /\.css/i,
          use: ["style-loader", "css-loader"],
        },
      ],
    },
    plugins: [
      new HtmlWebpackPlugin({
        template: "index.html",
      }),
    ],
    resolve: {
      extensions: [".ts", ".js"],
    },
    experiments: {
      asyncWebAssembly: true,
    },
  };
};
