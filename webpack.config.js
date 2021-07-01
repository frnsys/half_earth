let path = require('path');
const dev = process.env.NODE_ENV !== 'production';

module.exports = {
  entry: {
    'main': ['@babel/polyfill', './src/main'],
  },
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: '[name].js'
  },
  devtool: dev ? 'inline-source-map' : 'source-map',
  module: {
    rules: [{
      test: /\.js$/,
      exclude: /node_modules/,
      use: {
        loader: 'babel-loader',
        options: {
          presets: ['@babel/preset-env']
        }
      }
    }, {
      test: /\.glsl/,
      use: {
        loader: 'webpack-glsl-loader'
      }
    }]
  },
  resolve: {
    extensions: ['.js']
  },
  devServer: {
    writeToDisk: true
  }
};