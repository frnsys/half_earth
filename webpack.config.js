const path = require('path');
const webpack = require('webpack');
const { VueLoaderPlugin } = require('vue-loader')
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const dev = process.env.NODE_ENV !== 'production';

module.exports = (env) => ({
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
      test: /\.(png|svg|jpg|jpeg|gif)$/i,
      type: 'asset/resource',
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

    new webpack.DefinePlugin({
      VERSION: JSON.stringify(env.version),
      TIMESTAMP: JSON.stringify(env.timestamp),
      PLATFORM: JSON.stringify(process.env.PLATFORM),
    }),

    new webpack.DefinePlugin({
      __VUE_OPTIONS_API__: true,
      __VUE_PROD_DEVTOOLS__: false,
    }),

    // To get rid of a warning message:
    // "Critical dependency: the request of a dependency is an expression"
    // which might be from the rand crate (crypto)
    new webpack.ContextReplacementPlugin(/engine/),
  ],
  resolve: {
    extensions: ['.js'],
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
});