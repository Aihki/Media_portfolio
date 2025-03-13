module.exports = {
  publicPath: '/',
  devServer: {
    historyApiFallback: {
      rewrites: [
        // Redirect model file requests to the static directory
        { 
          from: /^\/models\/.*\.(glb|gltf|usdz|splat)$/, 
          to: context => context.parsedUrl.pathname 
        },
        // All other routes go through the single-page application
        { from: /./, to: '/index.html' }
      ]
    }
  },
  // Configure the static folder instead of public
  chainWebpack: config => {
    config
      .plugin('copy')
      .tap(args => {
        args[0].patterns.push({
          from: 'static',
          to: '',
          globOptions: {
            ignore: ['.DS_Store']
          }
        });
        return args;
      });
  }
}
