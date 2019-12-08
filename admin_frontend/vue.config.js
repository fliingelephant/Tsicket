module.exports = {
    //axios域代理，解决axios跨域问题

    publicPath: '/',
    devServer: {
        proxy: {
            '': {
                target: 'http://154.8.167.168:8080',
                changeOrigin: true,
                ws: true,
                pathRewrite: {
                }
            }
        }
    }
}
