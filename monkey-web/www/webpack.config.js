const CopyWebpackPlugin = require('copy-webpack-plugin');
const path = require('path');

const devMode = process.env.NODE_ENV !== 'production';

module.exports = {
  entry: './bootstrap.js',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'bootstrap.js',
  },
  mode: devMode ? 'development' : 'production',
  plugins: [
    new CopyWebpackPlugin(['index.html', 'styles.css', 'normalize.css']),
  ],
};
