<template>
	<view class="flex-page">
		<view class="cu-bar search animation-fade">
			<!-- <text class="cuIcon-roundadd margin-left" style="font-size: 48rpx"></text> -->
			<view class="search-form round">
				<text class="cuIcon-search"></text>
				<input v-model="keyword" @blur="InputBlur" :adjust-position="false" type="text" placeholder="根据活动名,组织搜索活动"
				 confirm-type="search" @confirm="searchPage"></input>
			</view>
			<view class="action">
				<button class="cu-btn shadow-blur round" @click="searchPage">搜索</button>
			</view>
		</view>
		<swiper class="card-swiper" :class="dotStyle?'square-dot':'round-dot'" :indicator-dots="true" :circular="true"
		 :autoplay="true" interval="5000" duration="500" @change="cardSwiper" indicator-color="#8799a3"
		 indicator-active-color="#0081ff">
			<swiper-item v-for="(item,index) in swiperList" :key="index" :class="cardCur==index?'cur':''" @click="swiperActivity">
				<view class="swiper-item" :class="reload ? 'animation-fade' : '' ">
					<image :src="item.img_url" mode="aspectFill"></image>
					<!-- v-if="item.type=='image'" <video :src="item.url" autoplay loop muted :show-play-btn="false" :controls="false" objectFit="cover" v-if="item.type=='video'"></video> -->
				</view>
			</swiper-item>
		</swiper>
		<view class="padding">
			<view class="flex-column">
				<activity-mini-card v-for="(item,index) in activitylist" :class="[item.delay?'animation-slide-bottom':'']" :style="[{animationDelay: item.delay}]"
				 :key="index" :activity="item" @like="like(index)" @clickCard="activityPage"></activity-mini-card>
			</view>
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
				keyword: '',
				scrollTop: 0,
				activityindex: 0,
				reload: false,
				cardCur: 0,
				more: true,
				swiperList: [],
				dotStyle: false,
				url: "/static/cardback0.jpg",
				activitylist: [],
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

		},
		onPageScroll(res) {
			this.scrollTop = res.scrollTop
		},
		onPullDownRefresh() {
			this.more = true
			this.activityindex = 0
			this.loadpage()
		},
		onReachBottom() {
			this.loadactivity(false)
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
						this.swiperList = []
						this.reload = false
						console.log(res)
						console.log(res.data);
						this.swiperList = res.data.list
						this.reload = true
						if (this.activitylist != []) {
							uni.stopPullDownRefresh()
						}
					}
				})
				this.loadactivity(true)
			},
			loadactivity(reload) {
				if (this.more) {
					uni.request({
						url: app.globalData.apiurl + 'users/index',
						data: {
							index: this.activityindex
						},
						header: {
							'content-type': 'application/json', //自定义请求头信息
							'cookie': app.globalData.cookie
						},
						success: (res) => {
							console.log(res)
							console.log(res.data)
							if (res.data.events) {
								res.data.events.forEach((res, index) => {
									//res.like = true
									// if(res.event_picture != '' ) {
									// 	res.img_url = res.event_picture
									// } else 
									if (!res.img_url || (res.img_url == '')) {
										res.img_url = app.globalData.backimg[parseInt('11' + res.event_id) % 4]
									}
									res.delay = '' + (index + 5) * 0.1 + 's'
									setTimeout(() => {
										res.delay = undefined
									}, (index + 11) * 100)
								})
								if (reload) {
									this.activitylist = []
									uni.showToast({
										title: "加载成功",
										icon: 'none'
									})
								} else {
									setTimeout(() => {
										uni.pageScrollTo({
											scrollTop: this.scrollTop + 300,
											duration: 500,
										})
										console.log("top" + this.scrollTop)
									}, 200)
								}
								this.activitylist = this.activitylist.concat(res.data.events)
								this.activityindex += res.data.events.length
								this.more = res.data.more
								console.log(this.activitylist)
							} else {
								uni.showToast({
									title: "加载失败",
									icon: 'none'
								})
							}
							if (this.swiperList != []) {
								uni.stopPullDownRefresh()
							}
						}
					})
				}
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
			InputBlur(e) {
				console.log(e)
			},
			searchPage() {
				console.log(this.keyword)
				uni.navigateTo({
					url: "../search/search?keyword=" + this.keyword
				})
			},
			like(index) {
				uni.request({
					url: app.globalData.apiurl + 'users/like/' + this.activitylist[index].event_id,
					method: 'POST',
					header: {
						'content-type': 'application/json', //自定义请求头信息
						'cookie': app.globalData.cookie
					},
					success: (res) => {
						console.log(res.data)
						console.log(index)
						this.activitylist[index].like = res.data.like
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
