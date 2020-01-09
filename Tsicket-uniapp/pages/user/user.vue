<template>
	<!-- <view class="flex-page">
		<view class="userinfo">
			<image class="usercard" src="../../static/cardback0.jpg"></image>
			<view v-if="hasUserInfo">
				<view class="padding flex align-start justify-between">
					<view class="cu-avatar xxl round" :style="{backgroundImage: userInfo.avatarUrl ? 'url(' + userInfo.avatarUrl + ')' : ''}"></view>
					<view class="text-white">
						<view class="text-right">
							<view class="text-sl text-ellipsis">
								<text class="text-bold">{{userInfo.nickName}}</text>
							</view>
						</view>
						<view class="text-right">
							<button v-if="!hasTsinghuaInfo" class="cu-btn round light bg-blue" @click="identification">清华身份认证</button>
							<text v-else>{{tsinghuaid}}</text>
						</view>
						<view class="text-left flex justify-start text-sm padding">
							<view class="padding-right-xl" @click="likePage">
								<view class="text-xl text-bold">{{like}}</view>
								喜爱
							</view>
							<view class="padding-right-xl" @click="followPage">
								<view class="text-xl text-bold">{{follow}}</view>
								关注
							</view>
							<view class="padding-right-xl" @click="historyPage">
								<view class="text-xl text-bold">{{history}}</view>
								历史
							</view>
						</view>
					</view>
				</view>
			</view>
			<view v-else class="flex justify-center getinfobtn">
				<button class="cu-btn round" open-type="getUserInfo" @getuserinfo="getUserInfo"> 授权个人信息</button>
			</view>
		</view>
		<view class="tabs">
			<view class="nav" :current="current" @change="navChange">
				<scroll-view scroll-x class="bg-white nav">
					<view class="flex text-center">
						<view class="flex-sub tab" :class="index==current?'tab-choose':''" v-for="(item,index) in tabs" :key="index" @tap="tabSelect"
						 :data-id="index">
							{{item}}
						</view>
					</view>
				</scroll-view>
			</view>
			<view class="tab-swiper-view">
				<swiper class="tab-swiper" :current="current" @change="swiperChange">
					<swiper-item>
						<scroll-view scroll-y class="tab-scroll padding">
							<view class="flex-column">
							<activity-prepare v-for="(item, index) in userActivity" :key="item.event_id" :activity="item" @clickSponsor="sponsorPage"
							 @detail="activityPage(item.event_id)" @announcement="announcementPage" @QRcode="QRcodePage" @cancel="cancelActivity(index)" @like="likeActivity(index)"
							 @unfold="unfold(index)"
							 :class="[item.delay?'animation-slide-bottom': item.cancel]"
							  :style="[{animationDelay: item.delay}]"></activity-prepare>
							 </view>
						</scroll-view>
					</swiper-item>
					<swiper-item>
						<scroll-view scroll-y class="tab-scroll padding">
							<activity-check :activity="activity" :sponsor="sponsor" :message="message" @like="like" @clickCard="activityPage"></activity-check>
						</scroll-view>
					</swiper-item>
				</swiper>
			</view>
		</view>
	</view> -->
	<view>
		<view class="userinfo flex-column">
			<image class="usercard" src="../../static/cardback0.jpg"></image>
			<view v-if="hasUserInfo">
				<view class="padding flex align-start justify-between">
					<view class="cu-avatar xxl round" :style="{backgroundImage: userInfo.avatarUrl ? 'url(' + userInfo.avatarUrl + ')' : ''}"></view>
					<view class="text-white">
						<view class="text-right">
							<view class="text-sl text-ellipsis">
								<text class="text-bold">{{userInfo.nickName}}</text>
							</view>
						</view>
						<view class="text-right">
							<button v-if="!hasTsinghuaInfo" class="cu-btn round light bg-blue" @click="identification">清华身份认证</button>
							<text v-else>{{tsinghuaid}}</text>
						</view>
						<view class="text-left flex justify-start text-sm padding">
							<view class="padding-right-xl" @click="likePage">
								<view class="text-xl text-bold">{{like}}</view>
								喜爱
							</view>
							<view class="padding-right-xl" @click="followPage">
								<view class="text-xl text-bold">{{follow}}</view>
								关注
							</view>
							<view class="padding-right-xl" @click="historyPage">
								<view class="text-xl text-bold">{{history}}</view>
								历史
							</view>
						</view>
					</view>
				</view>
			</view>
			<view v-else class="flex justify-center getinfobtn">
				<button class="cu-btn round" open-type="getUserInfo" @getuserinfo="getUserInfo"> 授权个人信息</button>
			</view>
		</view>
		<view class="tabs">
			<view class="nav" :current="current" @change="navChange">
				<scroll-view scroll-x class="bg-white nav">
					<view class="flex text-center">
						<view class="flex-sub tab" :class="index==current?'tab-choose':''" v-for="(item,index) in tabs" :key="index" @tap="tabSelect"
						 :data-id="index">
							{{item}}
						</view>
					</view>
				</scroll-view>
			</view>
		</view>
		<!-- <view class="tab-swiper-view">
			<swiper class="tab-swiper" :current="current" @change="swiperChange">
				<swiper-item> -->
		<view class="padding flex-column">
			<activity-prepare v-for="(item, index) in userActivity" :key="item.event_id" :activity="item" @clickSponsor="sponsorPage"
			 @detail="activityPage(item.event_id)" @announcement="announcementPage" @QRcode="QRcodePage" @cancel="cancelActivity(index)"
			 @like="likeActivity(index)" @unfold="unfold(index)" :class="[item.delay?'animation-slide-bottom': item.cancel]"
			 :style="[{animationDelay: item.delay}]"></activity-prepare>
		</view>
		<!-- 	</swiper-item>
				<swiper-item>
					<scroll-view scroll-y class="tab-scroll padding">
						<activity-check :activity="activity" :sponsor="sponsor" :message="message" @like="like" @clickCard="activityPage"></activity-check>
					</scroll-view>
				</swiper-item>
			</swiper>
		</view> -->
	</view>
</template>

<script>
	const app = getApp()

	export default {
		data() {
			return {
				loadinginfo: true,
				userInfo: {},
				hasUserInfo: false,
				hasTsinghuaInfo: false,
				tsinghuaid: 'xxxxxxxxxx',
				canIUse: uni.canIUse('button.open-type.getUserInfo'),
				like: 0,
				follow: 0,
				history: 0,
				current: 0,
				tabs: [
					"活动日程"
					//, "报名中"
				],
				userActivity: [],
			};
		},
		onLoad() {
			if (app.globalData.userInfo) {
				this.userInfo = app.globalData.userInfo
				console.log(this.userInfo)
				this.hasUserInfo = true
			} else if (this.canIUse) {
				// 由于 getUserInfo 是网络请求，可能会在 Page.onLoad 之后才返回
				// 所以此处加入 callback 以防止这种情况
				app.userInfoReadyCallback = res => {
					this.userInfo = res.userInfo
					console.log(this.userInfo)
					this.hasUserInfo = true
				}
			} else {
				// 在没有 open-type=getUserInfo 版本的兼容处理
				uni.getUserInfo({
					success: res => {
						app.globalData.userInfo = res.userInfo
						this.userInfo = res.userInfo
						console.log(this.userInfo)
						this.hasUserInfo = true
					}
				})
			}

			if (app.globalData.tsinghuaid) {
				this.tsinghuaid = app.globalData.tsinghuaid
				this.hasTsinghuaInfo = true
			}
			this.loadpage()
			uni.showShareMenu({})
		},
		onShow() {
			if (app.globalData.token) {
				console.log('token:' + app.globalData.token),
					uni.request({
						url: app.globalData.apiurl + 'users/tsinghuaid',
						method: 'POST',
						data: {
							token: app.globalData.token
						},
						header: {
							'content-type': 'application/json',
							'cookie': app.globalData.cookie
						},
						success: (res) => {
							console.log('调用绑定')
							console.log(res.data);
							this.hasUserInfo = true
							this.hasTsinghuaInfo = true
							this.tsinghuaid = res.data.tsinghuaid
							app.globalData.tsinghuaid = res.data.tsinghuaid
							app.globalData.token = undefined
						},
						fail: (res) => {
							console.log('绑定失败')
						}
					})
				app.globalData.token = undefined
			}
		},
		onPullDownRefresh() {
			this.loadpage()
		},
		onShareAppMessage(res) {
			return {
				title: app.globalData.sharetitle,
				path: '/pages/index/index',
				imageUrl: app.globalData.shareimg
			}
		},
		methods: {
			loadpage() {
				uni.request({
					url: app.globalData.apiurl + 'users/view', //仅为示例，并非真实接口地址。
					header: {
						'content-type': 'application/json', //自定义请求头信息
						'cookie': app.globalData.cookie
					},
					success: (res) => {
						console.log(res.data)
						this.follow = res.data.follow
						this.history = res.data.history
						this.like = res.data.like
					},
					fail: (res) => {
						uni.showToast({
							title: '错误',
							icon: 'none',
						})
					}
				})

				uni.request({
					url: app.globalData.apiurl + 'users/book', //仅为示例，并非真实接口地址。
					header: {
						'content-type': 'application/json', //自定义请求头信息
						'cookie': app.globalData.cookie
					},
					success: (res) => {
						console.log('users/book')
						console.log(res)
						res.data.events.forEach((res, index) => {
							res.isUnfold = false
							res.delay = '' + (index + 1) * 0.1 + 's'
							res.cancel = undefined
							if(!res.img_url || (res.img_url == '') ) {
								res.img_url =  app.globalData.backimg[parseInt('11' + res.event_id) % 4]
							}
							setTimeout(() => {
								res.delay = undefined
							}, (index + 11) * 100)
						})
						this.userActivity = res.data.events
						uni.stopPullDownRefresh()
						uni.showToast({
							title: '加载成功',
							icon: 'none',
						})

					},
					fail: (res) => {
						uni.showToast({
							title: '错误',
							icon: 'none',
						})
					}
				})
			},
			cardSwiper(e) {
				this.cardCur = e.detail.current
			},
			getUserInfo(e) {
				console.log(e)
				app.globalData.userInfo = e.detail.userInfo
				this.userInfo = e.detail.userInfo
				this.hasUserInfo = true
			},
			tabSelect(e) {
				this.current = e.currentTarget.dataset.id;
			},
			navChange(index) {
				this.current = index;
			},
			swiperChange(e) {
				this.current = e.detail.current;
			},
			likePage() {
				console.log("likepage")
				uni.navigateTo({
					url: "../like/like"
				})
			},
			followPage() {
				console.log("followpage")
				uni.navigateTo({
					url: "../following/following"
				})
			},
			historyPage() {
				console.log("historypage")
				// uni.navigateTo({
				// 	url: "../history/history"
				// })
			},
			sponsorPage(name) {
				uni.navigateTo({
					url: "../sponsor/sponsor?id=" + name
				})
			},
			activityPage(id) {
				uni.navigateTo({
					url: "../activity/activity?id=" + id
				})
			},
			announcementPage(id) {

			},
			QRcodePage(id) {

			},
			cancelActivity(index) {
				uni.request({
					url: app.globalData.apiurl + 'users/book/' + this.userActivity[index].event_id, //仅为示例，并非真实接口地址。
					method: 'DELETE',
					header: {
						'content-type': 'application/json', //自定义请求头信息
						'cookie': app.globalData.cookie
					},
					success: (res) => {
						console.log(res);
						this.userActivity[index].delay = undefined
						this.userActivity[index].cancel = 'animation-delete-slide-left'
						setTimeout(() => {
							this.userActivity[index].isUnfold = false
							this.userActivity.splice(index, 1)
						}, 2000)
					},
					fail: (res) => {

					}
				})
			},
			likeActivity(index) {
				uni.request({
					url: app.globalData.apiurl + 'users/like/' + this.userActivity[index].event_id,
					method: 'POST',
					header: {
						'content-type': 'application/json', //自定义请求头信息
						'cookie': app.globalData.cookie
					},
					success: (res) => {
						this.userActivity[index].like = res.data.like
						this.like += res.data.like ? 1 : -1
					}
				});
			},
			unfold(index) {
				this.userActivity[index].isUnfold = !this.userActivity[index].isUnfold
			},
			identification() {
				console.log("前往认证")
				uni.navigateToMiniProgram({
					appId: "wx1ebe3b2266f4afe0",
					path: "pages/index/index",
					envVersion: "trial",
					extraData: {
						origin: "miniapp",
						type: "id.tsinghua"
					},
					success(res) {
						// 打开成功
						console.log(res)
					},
					fail(res) {
						console.log(res)
					}
				})
			}
		}
	}
</script>

<style>
	.userinfo {
		width: 100%;
		height: 400rpx;
		padding: 40rpx 40rpx 40rpx 40rpx;
	}

	.usercard {
		position: absolute;
		width: 670rpx;
		height: 320rpx;
		border-radius: 20rpx 20rpx 20rpx 20rpx;
		z-index: -1;
	}

	.getinfobtn {
		padding: 120rpx;
	}

	.cu-avatar.xxl {
		width: 200rpx;
		height: 200rpx;
		font-size: 3em;
		border: 10rpx solid #fff;
	}

	.text-ellipsis {
		max-width: 400rpx;
		text-overflow: ellipsis;
		overflow: hidden;
	}

	.tab-swiper-view {
		height: calc(100vh - 400rpx - 80rpx)
	}


	.animation-delete-slide-left {
		animation-duration: .8s;
		animation-timing-function: ease-in-out;
		animation-fill-mode: both;
		animation-name: delete-slide-left
	}

	@keyframes delete-slide-left {
		0% {
			opacity: 1;
		}

		70% {
			opacity: 0;
			transform: translateX(-150%);
			height: 440rpx;
		}

		100% {
			transform: translateX(-150%);
			height: 1rpx;
		}
	}
</style>
