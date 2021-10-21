const path = require('path');
const webpack = require('webpack');
const { VueLoaderPlugin } = require('vue-loader')
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const dev = process.env.NODE_ENV !== 'production';

module.exports = {
  entry: {
    'main': ['./src/main'],
  },
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: '[name].js'
  },
  devtool: dev ? 'inline-source-map' : 'source-map',
  module: {
    rules: [{
      test: /\.ts$/,
      loader: 'ts-loader',
      options: {
        appendTsSuffixTo: [/\.vue$/],
      },
      exclude: /node_modules/,
    }, {
      test: /\.glsl$/,
      use: {
        loader: 'webpack-glsl-loader'
      }
    }, {
      test: /\.vue$/,
      loader: 'vue-loader'
    }, {
      test: /\.css$/,
      use: [
        dev ? 'vue-style-loader' : MiniCssExtractPlugin.loader,
        'css-loader'
      ]
    }, {
      test: /\.s[ac]ss$/i,
      use: [
        MiniCssExtractPlugin.loader,
        'css-loader',
        'sass-loader',
      ]
    }, {
      // This needs to be loaded as a regular file (asset)
      // to work correctly
      test: /hector\.wasm/,
      type: 'asset'
    }]
  },
  plugins: [
    new VueLoaderPlugin(),
    new MiniCssExtractPlugin(),

    // To get rid of a warning message:
    // "Critical dependency: the request of a dependency is an expression"
    // which might be from the rand crate (crypto)
    new webpack.ContextReplacementPlugin(/engine/),
  ],
  resolve: {
    extensions: ['.ts', '.js'],
    alias: {
      'lib': path.resolve('./src/lib'),
      'components': path.resolve('./src/components'),

      // Proxy three.js exports to reduce bundle size
      'three$': path.resolve('./src/3d/three.js')
    }
  },
  experiments: {
    asyncWebAssembly: true
  },
  devServer: {
    compress: true,
    writeToDisk: true,
    disableHostCheck: true,
    headers: {
      // Required for SharedArrayBuffer
      'Cross-Origin-Opener-Policy': 'same-origin',
      'Cross-Origin-Embedder-Policy': 'require-corp'
    }
  }
};