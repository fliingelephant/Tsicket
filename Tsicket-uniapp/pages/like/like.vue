<template>
	<view>
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
		</view>
		<view class="padding flex-column" id="likelist">
			<activity-mini-card v-for="(item,index) in likelist" :class="[item.delay ? 'animation-slide-bottom' : '']" :style="[{animationDelay: item.delay}]"
			 :key="item.event_id" :activity="item" @like="like(index)" @clickCard="activityPage(index)"></activity-mini-card>
		</view>
	</view>
</template>

<script>
	const app = getApp()

	export default {
		data() {
			return {
				scrollTop: 0,
				likelist: [],
				likeindex: 0,
				current: 0,
				more: true,
				tabs: [
					"喜爱"
				]
			};
		},
		onLoad() {
			this.loadpage()
			uni.showShareMenu({})
		},
		onPageScroll(res) {
			this.scrollTop = res.scrollTop
		},
		onPullDownRefresh() {
			this.likeindex = 0
			this.more = true
			this.loadpage()
		},
		onReachBottom() {
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
				if (this.more) {
					uni.request({
						url: app.globalData.apiurl + 'users/like', //仅为示例，并非真实接口地址。
						data: {
							index: this.likeindex
						},
						header: {
							'content-type': 'application/json', //自定义请求头信息
							'cookie': app.globalData.cookie
						},
						success: (res) => {
							console.log(res);
							console.log(res.data.list.length)
							if (this.likeindex == 0) {
								this.likelist = []
							}
							res.data.list.forEach((item, index) => {
								if(!item.img_url || (item.img_url == '') ) {
									item.img_url =  app.globalData.backimg[parseInt('11' + item.event_id) % 4]
								}
								item.like = true
								item.event_introduction = ''
								item.delay = '' + (index + 5) * 0.1 + 's'
								setTimeout(() => {
									item.delay = undefined
								}, (index + 11) * 100)
							})
							this.likelist = this.likelist.concat(res.data.list)
							if (this.likeindex != 0) {
								setTimeout(() => {
									uni.pageScrollTo({
										scrollTop: this.scrollTop + 300,
										duration: 500,
									})
									console.log("top" + this.scrollTop)
								}, 200)
							}
							this.more = res.data.more
							this.likeindex += res.data.list.length
							uni.stopPullDownRefresh()
						}
					})
				}
			},
			cardSwiper(e) {
				this.cardCur = e.detail.current
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
			activityPage(index) {
				uni.navigateTo({
					url: "../activity/activity?id=" + this.likelist[index].event_id
				})
			},
			like(index) {
				//console.log(id)
				uni.request({
					url: app.globalData.apiurl + 'users/like/' + this.likelist[index].event_id, //仅为示例，并非真实接口地址。
					method: 'POST',
					header: {
						'content-type': 'application/json', //自定义请求头信息
						'cookie': app.globalData.cookie
					},
					success: (res) => {
						console.log(res.data);
						this.likelist[index].like = res.data.like
						this.likeindex += res.data.like.length
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
		height: calc(100vh - 30rpx - 80rpx)
	}
</style>
