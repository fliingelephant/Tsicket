module.exports = {
    //axios域代理，解决axios跨域问题

    publicPath: '/',
    devServer: {
        proxy: {
            '': {
                target: 'http://tsicket.xyz',
                changeOrigin: true,
                ws: true,
                pathRewrite: {
                }
            }
        }
    }
}