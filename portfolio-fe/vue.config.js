module.exports = {
  publicPath: '/',
  devServer: {
    historyApiFallback: {
      rewrites: [
        // Handle /docs/ route and all its assets - redirect to backend
        {
          from: /^\/(docs|static\.files)\/.*/,
          to: context => `http://10.120.33.52${context.parsedUrl.pathname}`
        },
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
    proxy: {
      // Proxy all doc-related requests to the backend
      '/docs': {
        target: 'http://10.120.33.52',
        changeOrigin: true
      },
      '/static.files': {
        target: 'http://10.120.33.52',
        changeOrigin: true
      },
      // Proxy other potential doc asset paths
      '/normalize-*.css': {
        target: 'http://10.120.33.52',
        changeOrigin: true
      },
      '/rustdoc-*.css': {
        target: 'http://10.120.33.52',
        changeOrigin: true
      },
      '/storage-*.js': {
        target: 'http://10.120.33.52',
        changeOrigin: true
      },
      '/main-*.js': {
        target: 'http://10.120.33.52',
        changeOrigin: true
      },
      '/crates.js': {
        target: 'http://10.120.33.52',
        changeOrigin: true
      },
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
