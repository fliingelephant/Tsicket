<template>
	<view>
		<view class="cu-bar search">
			<text class="cuIcon-roundadd margin-left" style="font-size: 48rpx"></text>
			<view class="search-form round">
				<text class="cuIcon-search"></text>
				<input @focus="InputFocus" @blur="InputBlur" :adjust-position="false" type="text" placeholder="搜索活动、组织(暂未实现功能)"
				 confirm-type="search"></input>
			</view>
			<view class="action">
				<button class="cu-btn shadow-blur round">搜索</button>
			</view>
		</view>
		<swiper class="card-swiper" :class="dotStyle?'square-dot':'round-dot'" :indicator-dots="true" :circular="true"
		 :autoplay="true" interval="5000" duration="500" @change="cardSwiper" indicator-color="#8799a3"
		 indicator-active-color="#0081ff">
			<swiper-item v-for="(item,index) in swiperList" :key="index" :class="cardCur==index?'cur':''" @click="swiperActivity">
				<view class="swiper-item">
					<image :src="item.img_url" mode="aspectFill"></image>
					<!-- v-if="item.type=='image'" <video :src="item.url" autoplay loop muted :show-play-btn="false" :controls="false" objectFit="cover" v-if="item.type=='video'"></video> -->
				</view>
			</swiper-item>
		</swiper>
		<view class="padding">
			<scroll-view>
				<activity-mini-card v-for="(item,index) in activitylist" :key="index" :activity="item" @like="like" @clickCard="activityPage"></activity-mini-card>
			</scroll-view>
		</view>
	</view>
	<!-- <view class="content">
		<image class="logo" src="/static/logo.png"></image>
		<view class="text-area">
			<text class="title">{{title}}</text>
		</view>
	</view> -->
</template>

<script>
	const app = getApp()

	export default {
		data() {
			return {
				cardCur: 0,
				swiperList: [],
				dotStyle: false,
				towerStart: 0,
				direction: '',
				url: "/static/cardback0.jpg",
				activitylist: [
					//	{
					// 	event_id: 0,
					// 	event_name: '活动名',
					// 	event_introdution: '活动介绍语',
					// 	event_picture: '',
					// 	left_tickets: 80,
					// 	event_location: '活动地点',
					// 	start_time: '2019年xx月xx日',
					// 	end_time: '',
					// 	sponsor_name: 'xx学生会',
					// 	event_type: 1,
					// 	event_status: 200,
					// 	reserved: false,
					// 	like: false
					// }
				],
				current: 0,
				tabs: [
					"介绍", "动态"
				]
			};
		},
		onLoad() {
			console.log(app.globalData.cookie)
			if (app.globalData.cookie != '') {
				console.log('indexonload')
				this.loadpage()
			} else {
				console.log('nocookie')
				app.globalData.cookieReadyCallback = this.loadpage
			}
			
			uni.showShareMenu({})
			// app.globalData.cookieReadyCallback = function(res) {
			// 	uni.request({
			// 		url: 'http://2019-a18.iterator-traits.com/apis/users/broadcast',
			// 		//method: 'POST',
			// 		/* data: {
			// 			index: 0
			// 		}, */
			// 		header: {
			// 			'content-type': 'application/json', //自定义请求头信息
			// 			'cookie': app.globalData.cookie
			// 		},
			// 		success: (res) => {
			// 			console.log(res.data);
			// 		}
			// 	})
			// }
			//console.log(app.cookieReadyCallback)
		},
		onShareAppMessage(res) {
			return {
			    title: app.globalData.sharetitle,
			    path: '/pages/index/index',
				imageUrl: app.globalData.shareimg
			}
		},
		methods: {
			loadpage(cookie) {
				console.log('cookieReadyCallback')
				console.log(cookie)
				/* uni.request({
					url: 'http://2019-a18.iterator-traits.com/apis/users/newactivities',
					method: 'POST',
					data: {
						index: 0
					},
					header: {
						'content-type': 'application/json', //自定义请求头信息
						'cookie': cookie
					},
					success: (res) => {
						
						console.log(res);
					}
				}) */
				uni.showLoading({
					title: '加载中'
				})
				uni.request({
					url: app.globalData.apiurl + 'users/broadcast',
					header: {
						'content-type': 'application/json', //自定义请求头信息
						'cookie': app.globalData.cookie
					},
					success: (res) => {
						console.log(res)
						console.log(res.data);
						this.swiperList = res.data.list
						if (this.activitylist != []) {
							uni.showToast({
								title: '加载成功',
								icon: 'none',
							})
						}
					}
				})
				uni.request({
					url: app.globalData.apiurl + 'admins',
					//method: 'POST',
					/* data: {
						index: 0
					}, */
					header: {
						'content-type': 'application/json', //自定义请求头信息
						'cookie': cookie
					},
					success: (res) => {
						console.log(res)
						console.log(res.data)
						var temp = res.data.events
						temp.forEach(res => {
							res.like = true
						})
						this.activitylist = temp
						console.log(this.activitylist)
						if (this.swiperList != []) {
							uni.showToast({
								title: '加载成功',
								icon: 'none',
							})
						}
					}
				})
				/* uni.request({
					url: 'http://2019-a18.iterator-traits.com/apis/users',
					//method: 'POST',
					header: {
						'content-type': 'application/json', //自定义请求头信息
						'cookie': cookie
					},
					success: (res) => {
						console.log(res)
						console.log(res.data);
					}
				}) */


			},
			cardSwiper(e) {
				this.cardCur = e.detail.current
			},
			swiperActivity(e) {
				this.activityPage(this.swiperList[this.cardCur].event_id)
			},
			activityPage(id) {
				uni.navigateTo({
					url: "../activity/activity?id=" + id,
				})
			},
			like(id) {
				uni.request({
					url: app.globalData.apiurl + 'users/like/' + id,
					method: 'POST',
					header: {
						'content-type': 'application/json', //自定义请求头信息
						'cookie': app.globalData.cookie
					},
					success: (res) => {
						console.log(res.data)
						console.log(id)
						var index = this.activitylist.findIndex((item) => {
							return item.event_id == id
						})
						console.log(index)
						this.activitylist[index].like = res.data.like
						console.log(this.activitylist[index])
					}
				});
			}
		}
	}
</script>

<style>
	.content {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
	}

	.logo {
		height: 200rpx;
		width: 200rpx;
		margin-top: 200rpx;
		margin-left: auto;
		margin-right: auto;
		margin-bottom: 50rpx;
	}

	.text-area {
		display: flex;
		justify-content: center;
	}

	.title {
		font-size: 36rpx;
		color: #8f8f94;
	}


	.swiper-item {
		background-color: #CCE6FF;
	}

	.flex-column {
		display: flex;
		flex-direction: column;
		justify-content: space-between;
	}

	.activity-list {

		padding: 10rpx;
	}
</style>
