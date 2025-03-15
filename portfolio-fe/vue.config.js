module.exports = {
  publicPath: '/',
  devServer: {
    historyApiFallback: {
      rewrites: [
        // Don't rewrite URLs for documentation paths
        {
          from: /^\/docs(\/.*)?$/,
          to: context => `http://10.120.33.52/docs${context.parsedUrl.pathname.replace(/^\/docs/, '')}`
        },
        // Don't rewrite URLs for static.files paths
        {
          from: /^\/static\.files(\/.*)?$/,
          to: context => `http://10.120.33.52/static.files${context.parsedUrl.pathname.replace(/^\/static\.files/, '')}`
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
        changeOrigin: true,
        secure: false
      },
      '/static.files': {
        target: 'http://10.120.33.52',
        changeOrigin: true,
        secure: false
      },
      // Proxy CSS and JS files that might be requested by documentation
      '*.css': {
        target: 'http://10.120.33.52',
        changeOrigin: true,
        secure: false,
        bypass: function(req, res, proxyOptions) {
          // Only proxy CSS files that are likely part of the documentation
          if (req.url.includes('normalize') || req.url.includes('rustdoc')) {
            return false; // Continue with proxy
          }
          return req.url; // Skip proxy for other CSS files
        }
      },
      '*.js': {
        target: 'http://10.120.33.52',
        changeOrigin: true,
        secure: false,
        bypass: function(req, res, proxyOptions) {
          // Only proxy JS files that are likely part of the documentation
          if (req.url.includes('storage-') || req.url.includes('main-') || req.url.includes('crates.js')) {
            return false; // Continue with proxy
          }
          return req.url; // Skip proxy for other JS files
        }
      },
      // Proxy font files that might be requested by documentation
      '*.woff2': {
        target: 'http://10.120.33.52',
        changeOrigin: true,
        secure: false
      }
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
