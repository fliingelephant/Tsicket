<script>
	export default {
		globalData:{
			hasuserInfo: false
		},
		onLaunch: function() {
			console.log('App Launch')
			//登录
			uni.login({
				success: res => {
					console.log(res.code)
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
		onShow: function() {
			console.log('App Show')
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
	@import "app.css";  /*你的项目css */
</style>
