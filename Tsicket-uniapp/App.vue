<script>
	export default {
		globalData: {
			hasuserInfo: false,
			openid: 0,
			cookie: '',
			apiurl: 'http://2019-a18.iterator-traits.com:8080/apis/',
			sharetitle: '清易票tsicket-校园活动抢票平台',
			shareimg: '/static/tsicket.png'
		},
		onLaunch: function() {
			console.log('App Launch')
			console.log(this.globalData.hasuserInfo)
			//登录
			uni.login({
				success: res => {
					console.log(res)
					uni.request({
						url: this.globalData.apiurl + 'users/login', //仅为示例，并非真实接口地址。
						method: "POST",
						data: {
							code: res.code,
						},
						header: {
							'content-type': 'application/json' //自定义请求头信息
						},
						success: (res) => {
							console.log(res);
							this.globalData.openid = res.data.openid
							if(res.header["set-cookie"]) {
								this.globalData.cookie = res.header["set-cookie"].slice(0, res.header["set-cookie"].indexOf(';'))
							} else {
								this.globalData.cookie = res.header["Set-Cookie"].slice(0, res.header["Set-Cookie"].indexOf(';'))
							}
							console.log(this.globalData.cookie)
							if (this.globalData.cookieReadyCallback) {
								console.log('callback')
								this.globalData.cookieReadyCallback(this.globalData.cookie)
							}
							else {
								console.log('nocallback')
							}
							uni.request({
								url: this.globalData.apiurl + 'users/tsinghuaid', //仅为示例，并非真实接口地址。
								header: {
									'content-type': 'application/json', //自定义请求头信息
									'cookie': this.globalData.cookie
								},
								success: (res) => {
									console.log(res.data)
									this.globalData.tsinghuaid = res.data.tsinghuaid
								}
							})
						},
						fail() {
							console.log('loginfail')
						}
					})
				}
			})
			/* uni.authorize({
				scope: 'scope.userInfo',
				success() {
					uni.getUserInfo({
						success:function(e){
							console.log(e)
							this.globalData.userInfo = e.userInfo
						}
					})
				}
			}) */
			// 获取用户信息
			uni.getSetting({
				success: res => {
					if (res.authSetting['scope.userInfo']) {
						// 已经授权，可以直接调用 getUserInfo 获取头像昵称，不会弹框
						uni.getUserInfo({
							success: res => {
								// 可以将 res 发送给后台解码出 unionId
								this.globalData.userInfo = res.userInfo
								this.globalData.hasuserInfo = true

								// 由于 getUserInfo 是网络请求，可能会在 Page.onLoad 之后才返回
								// 所以此处加入 callback 以防止这种情况
								if (this.userInfoReadyCallback) {
									this.userInfoReadyCallback(res)
								}
							}
						})
					}
				}
			})
		},
		onShow: function(res) {
			if (res.referrerInfo && res.referrerInfo.appId) {
				if (res.referrerInfo.extraData) {
					this.globalData.token = res.referrerInfo.extraData.token
				}
			}
		},
		onHide: function() {
			console.log('App Hide')
		}
	}
</script>

<style>
	/*每个页面公共css */
	@import "colorui/main.css";
	@import "colorui/icon.css";
	@import "colorui/animation.css";
	@import "app.css";
	/*你的项目css */
</style>
