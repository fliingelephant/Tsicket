module.exports = {
    //axios域代理，解决axios跨域问题

    publicPath: '/',
    devServer: {
        proxy: {
            '': {
                target: 'https://2019-a18.iterator-traits.com',
                changeOrigin: true,
                ws: true,
                pathRewrite: {
                }
            }
        }
    }
}
