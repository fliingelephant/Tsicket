<template>
	<!-- 	<view class="flex-page">
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
						<scroll-view scroll-y class="tab-scroll">
							<follow-list :followlist="followlist" @clickitem="sponsorPage" @follow="follow"></follow-list>
						</scroll-view>
					</swiper-item>
				</swiper>
			</view>
		</view>
	</view> -->
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
			<follow-list :followlist="followlist" @clickitem="sponsorPage" @follow="follow"></follow-list>
		</view>
	</view>
</template>

<script>
	const app = getApp()

	export default {
		data() {
			return {
				scrollTop: 0,
				followlist: [],
				followindex: 0,
				current: 0,
				more: true,
				tabs: [
					"关注"
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
			this.followindex = 0
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
						url: app.globalData.apiurl + 'users/follow', //仅为示例，并非真实接口地址。
						data: {
							index: this.followindex
						},
						header: {
							'content-type': 'application/json', //自定义请求头信息
							'cookie': app.globalData.cookie
						},
						success: (res) => {
							console.log(res);
							if (this.followindex == 0) {
								this.followlist = []
							}
							res.data.list.forEach((item, index) => {
								item.follow = true
								item.delay = '' + (index + 5) * 0.1 + 's'
								setTimeout(() => {
									item.delay = undefined
								}, (index + 11) * 100)
							})
							this.followlist = this.followlist.concat(res.data.list)
							if (this.followindex != 0) {
								setTimeout(() => {
									uni.pageScrollTo({
										scrollTop: this.scrollTop + 300,
										duration: 500,
									})
									console.log("top" + this.scrollTop)
								}, 200)
							}
							this.more = res.data.more
							this.followindex += res.data.list.length
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
			sponsorPage(index) {
				uni.navigateTo({
					url: "../sponsor/sponsor?id=" + this.followlist[index].name
				})
			},
			follow(index) {
				uni.request({
					url: app.globalData.apiurl + 'users/follow/' + this.followlist[index].name, //仅为示例，并非真实接口地址。
					method: 'POST',
					header: {
						'content-type': 'application/json', //自定义请求头信息
						'cookie': app.globalData.cookie
					},
					success: (res) => {
						console.log(res.data);
						this.followlist[index].follow = res.data.follow
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
