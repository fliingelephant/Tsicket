<template>
	<view class="flex-page">
		<view class="tabs padding-top">
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
					<swiper-item class="tab-swiper-item" style="height: 100%;">
						<scroll-view scroll-y class="tab-scroll" @scrolltolower='loadmoment'>
							<view class="flex-column">
								<message v-for="(item, index) in momentlist0" :class="[item.delay? momentanimation[0] :'']" :style="[{animationDelay: item.delay}]"
								 :key="index" :message="item" @appreciate="appreciate" @sponsorPage='sponsorPage' @activityPage='activityPage'></message>
							</view>
							<view v-if="momentmore0" style="height: 80rpx"></view>
						</scroll-view>
					</swiper-item>
					<swiper-item>
						<scroll-view scroll-y class="tab-scroll" @scrolltolower='loadmoment'>
							<view class="flex-column">
								<message v-for="(item, index) in momentlist1" :class="[item.delay? momentanimation[1] :'']" :style="[{animationDelay: item.delay}]"
								 :key="index" :message="item" @appreciate="appreciate" @sponsorPage='sponsorPage' @activityPage='activityPage'></message>
							</view>
							<view v-if="momentmore1" style="height: 80rpx"></view>
						</scroll-view>
					</swiper-item>
					<swiper-item>
						<scroll-view scroll-y class="tab-scroll" @scrolltolower='loadmoment'>
							<view class="flex-column">
								<message v-for="(item, index) in momentlist2" :class="[item.delay? momentanimation[2] :'']" :style="[{animationDelay: item.delay}]"
								 :key="index" :message="item" @appreciate="appreciate" @sponsorPage='sponsorPage' @activityPage='activityPage'></message>
							</view>
							<view v-if="momentmore2" style="height: 80rpx"></view>
						</scroll-view>
					</swiper-item>
				</swiper>
			</view>
		</view>
	</view>
</template>

<script>
	const app = getApp()

	export default {
		data() {
			return {
				momentlist0: [],
				momentmore0: true,
				momentindex0: 0,
				momentlist1: [],
				momentmore1: true,
				momentindex1: 0,
				momentlist2: [],
				momentindex2: 0,
				momentanimation: ['animation-slide-bottom', 'animation-slide-right', 'animation-slide-right'],
				current: 0,
				tabs: [
					"喜爱", "关注"//, "广场"
				]
			};
		},
		onLoad() {
			this.loadpage()
			uni.showShareMenu({})
		},
		onPullDownRefresh() {
			this.momentanimation[this.current] = 'animation-slide-bottom'
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
				if (this.current == 0) {
					this.momentlist0 = []
					this.momentmore0 = true
					this.momentindex0 = 0
				} else if (this.current == 1) {
					this.momentlist1 = []
					this.momentmore1 = true
					this.momentindex1 = 0
				} else {
					this.momentlist2 = []
					this.momentmore2 = true
					this.momentindex2 = 0
				}
				this.loadmoment()
			},
			tabSelect(e) {
				if (this.current == e.currentTarget.dataset.id) {
					this.current = e.currentTarget.dataset.id
					uni.showLoading({
						title: '刷新中'
					})
					this.loadpage()
				}
				this.current = e.currentTarget.dataset.id;
			},
			navChange(index) {
				this.current = index;
			},
			swiperChange(e) {
				this.current = e.detail.current;
				console.log(this.current)
				this.loadmoment()
			},
			loadmoment(e) {
				console.log(e)
				if ((this.current == 0) && (this.momentindex0 != -1)) {
					uni.request({
						url: app.globalData.apiurl + 'users/momentslike',
						data: {
							index: this.momentindex0
						},
						header: {
							'content-type': 'application/json', //自定义请求头信息
							'cookie': app.globalData.cookie
						},
						success: (res) => {
							console.log(res.data);
							res.data.moments.forEach((item, index) => {
								item.delay = '' + (index + 1) * 0.1 + 's'
								setTimeout(() => {
									item.delay = undefined
								}, (index + 11) * 100)
							})
							this.momentlist0 = this.momentlist0.concat(res.data.moments)
							this.momentmore0 = res.data.more
							this.momentindex0 += res.data.moments.length
							uni.hideLoading()
							uni.stopPullDownRefresh()
						}
					})
				} else if ((this.current == 1) && (this.momentindex1 != -1)) {
					uni.request({
						url: app.globalData.apiurl + 'users/momentsfollow',
						data: {
							index: this.momentindex1
						},
						header: {
							'content-type': 'application/json', //自定义请求头信息
							'cookie': app.globalData.cookie
						},
						success: (res) => {
							console.log(res.data);
							res.data.moments.forEach((item, index) => {
								item.delay = '' + (index + 1) * 0.1 + 's'
								setTimeout(() => {
									item.delay = undefined
								}, (index + 11) * 100)
							})
							this.momentlist1 = this.momentlist1.concat(res.data.moments)
							this.momentmore1 = res.data.more
							this.momentindex1 += res.data.moments.length
							uni.hideLoading()
						}
					});
				} else if ((this.current == 2) && (this.momentindex2 != -1)) {
					uni.request({
						url: app.globalData.apiurl + 'users/moments/like',
						data: {
							index: this.momentmore2
						},
						header: {
							'content-type': 'application/json', //自定义请求头信息
							'cookie': app.globalData.cookie
						},
						success: (res) => {
							console.log(res.data);
							res.data.moments.forEach((item, index) => {
								item.delay = '' + (index + 1) * 0.1 + 's'
								setTimeout(() => {
									item.delay = undefined
								}, (index + 11) * 100)
							})
							this.momentlist2 = this.momentlist2.concat(res.data.moments)
							this.momentmore2 = res.data.more
							uni.hideLoading()
						}
					});
				}
			},
			appreciate(id) {
				uni.request({
					url: app.globalData.apiurl + 'users/appreciate',
					method: 'POST',
					data: {
						openid: app.globalData.openid,
						messageid: id,
						session: '',
					},
					header: {
						'content-type': 'application/json' //自定义请求头信息
					},
					success: (res) => {
						console.log(res.data);
					}
				});
				var index = this.messagelist.findIndex((item) => {
					return item.id == id
				})
				console.log(index)
				this.messagelist[index].appreciate = !this.messagelist[index].appreciate
			},
			activityPage(id) {
				uni.navigateTo({
					url: "../activity/activity?id=" + id
				})
			},
			sponsorPage(id) {
				uni.navigateTo({
					url: "../sponsor/sponsor?id=" + id
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
		height: calc(100vh - 30rpx - 80rpx)
	}
</style>
