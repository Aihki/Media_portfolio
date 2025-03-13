module.exports = {
  publicPath: '/',
  devServer: {
    historyApiFallback: {
      rewrites: [
        // Serve model files directly from static directory
        { 
          from: /^\/static\/models\/.*\.(glb|gltf|usdz|splat)$/, 
          to: context => context.parsedUrl.pathname 
        },
        // Also serve directly from models path (redirected to static)
        { 
          from: /^\/models\/.*\.(glb|gltf|usdz|splat)$/, 
          to: context => `/static${context.parsedUrl.pathname}` 
        },
        // All other routes go through the single-page application
        { from: /./, to: '/index.html' }
      ]
    },
    static: {
      // Explicitly serve the static directory
      directory: './static',
      publicPath: '/static'
    }
  },
  // Configure the static folder copying for production builds
  chainWebpack: config => {
    config
      .plugin('copy')
      .tap(args => {
        args[0].patterns.push({
          from: 'static',
          to: 'static', // Copy to static subfolder in output
          globOptions: {
            ignore: ['.DS_Store']
          }
        });
        return args;
      });
  }
}
