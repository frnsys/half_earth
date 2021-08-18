let path = require('path');
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
    new MiniCssExtractPlugin()
  ],
  resolve: {
    extensions: ['.js'],
    alias: {
      'lib': path.resolve('./src/lib'),

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