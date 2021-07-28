let path = require('path');
const { VueLoaderPlugin } = require('vue-loader')
const dev = process.env.NODE_ENV !== 'production';

module.exports = {
  entry: {
    'main': ['./src/main'],
    'prototype': ['./prototype/src/main'],
  },
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: '[name].js'
  },
  devtool: dev ? 'inline-source-map' : 'source-map',
  module: {
    rules: [{
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
        'vue-style-loader',
        'css-loader'
      ]
    }, {
      test: /\.s[ac]ss$/i,
      use: [
        'style-loader',
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
    new VueLoaderPlugin()
  ],
  resolve: {
    extensions: ['.js'],
    alias: {
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