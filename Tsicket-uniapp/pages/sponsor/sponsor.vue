<template>
	<view class="flex-page">
		<view class="collapse">
			<image :src="url" class="idcard"></image>
			<view class="card-back"></view>
			<view class="info padding-lg flex align-start justify-between text-white">
				<view class="text-left flex-column info-flex">
					<view class="text-xxl">{{sponsor.name}}
						<view class="line-round"></view>
					</view>
					<view class="text-left flex justify-start text-sm">
						<view class="padding-right-xl">
							<view class="text-xl text-bold">{{sponsor.to_start}}</view>
							待开始
						</view>
						<view class="padding-right-xl">
							<view class="text-xl text-bold">{{sponsor.history}}</view>
							历史活动
						</view>
						<view class="padding-right-xl">
							<view class="text-xl text-bold">{{sponsor.message}}</view>
							<!-- 动态 -->
						</view>
					</view>
				</view>
				<view class="cu-avatar xxl round" :style="{backgroundImage: 'url(' + sponsor.avatar_url + ')'}"></view>
			</view>
			<view class="toolbar flex align-stretch text-center">
				<view class="flex-sub flex align-center justify-center">
					<button open-type="share" @click="share">
						<view class="flex align-center justify-center">
							<text class="cuIcon-share"></text>
						</view>
					</button>
				</view>
				<view class="flex-sub flex align-center justify-center" @click="follow">
					<text :class="sponsor.follow ? 'cuIcon-check' : 'cuIcon-friendadd'"></text>
				</view>
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
						<scroll-view scroll-y class="tab-scroll" @scrolltolower='loadactivity'>
							<view class="flex-column">
								<activity-mini-card v-for="(item,index) in activitylist" :class="[item.delay ? 'animation-slide-bottom' : '']"
								 :style="[{animationDelay: item.delay}]" :key="item.event_id" :activity="item" @like="like(index)" @clickCard="activityPage(item.event_id)"></activity-mini-card>
							</view>
							<view v-if="activitymore" style="height: 80rpx"></view>
						</scroll-view>
					</swiper-item>
					<swiper-item>
						<scroll-view scroll-y class="tab-scroll" @scrolltolower='loadmoment'>
							<view class="flex-column">
								<message v-for="(item, index) in momentlist" class="moment" :class="[item.delay?'animation-slide-right':'']"
								 :style="[{animationDelay: item.delay}]" :key="index":message="item" @appreciate="appreciate"
								 @activityPage='activityPage'></message>
							</view>
							<view v-if="momentmore" style="height: 80rpx"></view>
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
				url: "/static/cardback0.jpg",
				sponsor: {},
				activitylist: [],
				activityindex: 0,
				activitymore: true,
				momentlist: [],
				momentindex: 0,
				momentmore: true,
				current: 0,
				tabs: [
					"活动", "动态"
				],
			};
		},
		onLoad(option) {
			console.log(option)
			this.sponsor.name = option.id
			if (app.globalData.cookie != '') {
				console.log('onload')
				this.loadpage()
			} else {
				console.log('nocookie')
				app.globalData.cookieReadyCallback = this.loadpage
			}
			uni.showShareMenu({})
		},
		onShareAppMessage(res) {
			console.log(res)
			return {
				title: '清易票-' + this.sponsor.name,
				//imageUrl: app.globalData.shareimg
			}
		},
		methods: {
			loadpage() {
				this.momentindex = 0
				this.momentlist = []
				this.momentmore = true
				this.activityindex = 0
				this.activitylist = []
				this.activitymore = true
				uni.request({
					url: app.globalData.apiurl + 'sponsors/view/' + this.sponsor.name,
					//method: 'POST',
					header: {
						'content-type': 'application/json', //自定义请求头信息
						'cookie': app.globalData.cookie
					},
					success: (res) => {
						console.log(res)
						this.sponsor = res.data
					}
				})
				if (this.current == 0) {
					this.loadactivity()
				} else {
					this.loadmoment()
				}
			},
			loadactivity(e) {
				console.log(e)
				if (this.activitymore) {
					console.log('loadactivity' + this.momentindex)
					uni.request({
						url: app.globalData.apiurl + 'users/events',
						data: {
							index: this.activityindex,
							sponsor_name: this.sponsor.name,
						},
						header: {
							'content-type': 'application/json', //自定义请求头信息
							'cookie': app.globalData.cookie
						},
						success: (res) => {
							console.log(res)
							res.data.events.forEach((item, index) => {
								item.sponsor_avatar = this.sponsor.avatar_url
								item.delay = '' + (index + 1) * 0.1 + 's'
								setTimeout(() => {
									item.delay = undefined
								}, (index + 11) * 100)
							})
							this.activitylist = this.activitylist.concat(res.data.events)
							this.activitymore = res.data.more
							this.activityindex += res.data.events.length
							if(this.activityindex == 0) {
								this.activityindex = -1
							}
						}
					})
				}
			},
			loadmoment(e) {
				console.log(e)
				if (this.momentmore) {
					console.log('loadmoment' + this.momentindex)
					uni.request({
						url: app.globalData.apiurl + 'users/moments',
						data: {
							'index': this.momentindex,
							'sponsor_name': this.sponsor.name
						},
						header: {
							'content-type': 'application/json',
							'cookie': app.globalData.cookie
						},
						success: (res) => {
							console.log(res)
							res.data.moments.forEach((item, index) => {
								item.delay = '' + (index + 1) * 0.1 + 's'
								setTimeout(() => {
									item.delay = undefined
								}, (index + 11) * 100)
							})
							this.momentlist = this.momentlist.concat(res.data.moments)
							this.momentmore = res.data.more
							this.momentindex += res.data.moments.length
							if(this.momentindex == 0) {
								this.momentindex = -1
							}
						}
					})
				}
			},
			tabSelect(e) {
				this.current = e.currentTarget.dataset.id;
			},
			navChange(index) {
				this.current = index;
			},
			swiperChange(e) {
				this.current = e.detail.current;
				if (this.current == 0 && this.activityindex != -1) {
					this.loadactivity()
				} else if (this.momentindex != -1) {
					this.loadmoment()
				}
			},
			share() {

			},
			follow() {
				uni.request({
					url: app.globalData.apiurl + 'users/follow/' + this.sponsor.name,
					method: 'POST',
					header: {
						'content-type': 'application/json', //自定义请求头信息
						'cookie': app.globalData.cookie
					},
					success: (res) => {
						console.log(res.data);
						this.sponsor.follow = res.data.follow
					}
				})
			},
			appreciate(index) {
				// uni.request({
				// 	url: app.globalData.apiurl + 'users/appreciate',
				// 	method: 'POST',
				// 	data: {
				// 		openid: app.globalData.openid,
				// 		messageid: id,
				// 		session: '',
				// 	},
				// 	header: {
				// 		'content-type': 'application/json', 
				// 		'cookie': app.globalData.cookie
				// 	},
				// 	success: (res) => {
				// 		console.log(res.data);
				// 		this.momentlist[index].appreciate = !this.momentlist[index].appreciate
				// 	}
				// });
			},
			activityPage(id) {
				var page = getCurrentPages()
				page = page[page.length - 2]
				if (page.route == 'pages/activity/activity' && page.options.id == id) {
					uni.navigateBack()
				} else {
					uni.navigateTo({
						url: "../activity/activity?id=" + id
					})
				}
			},
			like(index) {
				//console.log(id)
				uni.request({
					url: app.globalData.apiurl + 'users/like/' + this.activitylist[index].event_id, //仅为示例，并非真实接口地址。
					method: 'POST',
					header: {
						'content-type': 'application/json', //自定义请求头信息
						'cookie': app.globalData.cookie
					},
					success: (res) => {
						console.log(res.data);
						this.activitylist[index].like = res.data.like
					}
				})
			}
		}
	}
</script>

<style>
	.collapse {
		width: 100%;
		padding: 40rpx 40rpx 40rpx 40rpx;
	}

	.idcard {
		position: absolute;
		width: 670rpx;
		height: 300rpx;
		border-radius: 20rpx 20rpx 20rpx 20rpx;
		z-index: -1;
	}

	.card-back {
		position: absolute;
		width: 670rpx;
		height: 400rpx;
		border-radius: 20rpx 20rpx 20rpx 20rpx;
		background-color: #F2F2F2;
		z-index: -2;
	}

	.line-round {
		margin-top: 10rpx;
		width: 70rpx;
		height: 10rpx;
		border-radius: 5rpx 5rpx 5rpx 5rpx;
		background-color: gainsboro;
	}

	.info {
		height: 300rpx;
	}

	.info-flex {
		height: 220rpx;
	}

	.flex-column {
		display: flex;
		flex-direction: column;
		justify-content: space-between;
	}

	.toolbar {
		height: 100rpx;
		width: 100%;
		font-size: 48rpx;
	}

	button {
		font-size: 48rpx;
		width: 100%;
		height: 100%;
		padding: 0;
		border: none;
		background: none;
	}

	button:after {
		border: none;
	}

	button>view {
		width: 100%;
		height: 100%;
	}

	.tab-swiper-view {
		height: calc(100vh - 480rpx - 80rpx)
	}

	.cu-avatar.xxl {
		width: 200rpx;
		height: 200rpx;
		font-size: 3em;
		border: 10rpx solid #fff;
	}
</style>
