const path = require('path');
const { VueLoaderPlugin } = require('vue-loader')
const dev = process.env.NODE_ENV !== 'production';

module.exports = {
  entry: {
    'main': ['./src/main'],
  },
  output: {
    path: path.resolve(__dirname, 'static/dist'),
    filename: '[name].js'
  },
  devtool: dev ? 'inline-source-map' : 'source-map',
  module: {
    rules: [{
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
    }]
  },
  plugins: [
    new VueLoaderPlugin(),
  ],
  resolve: {
    extensions: ['.js']
  },
  devServer: {
    compress: true,
    writeToDisk: true,
  }
};