<template>
	<view class="flex-page">
		<view class="collapse">
			<image :src="url" class="idcard"></image>
			<view class="card-back"></view>
			<view class="info padding flex align-start justify-between text-white">
				<view class="text-left flex-column info-flex">
					<view class="text-xxl">{{activity.event_name}}
						<view class="line-round"></view>
					</view>
					<view class="text-df">{{activity.event_introduction}}</view>
					<view class="text-df">
						<view>{{activity.event_location}}</view>
						<view>{{activity.start_time}}</view>
					</view>
				</view>
				<view class="text-right flex-column info-flex">
					<view>
						<view class="text-sm">抢票制</view>
						<view class="text-sm">余票:{{activity.left_tickets}}</view>
					</view>
					<view class="flex align-end justify-end" @click="sponsorPage">
						<view class="padding-right-xs text-sm">{{activity.sponsor_name}}</view>
						<view class="cu-avatar round solids"></view>
					</view>
				</view>
			</view>
			<view class="toolbar flex align-stretch text-center">
				<view class="flex-sub flex align-center justify-center" @click="reserve">
					<text :class="isreserved ? 'cuIcon-delete' : 'cuIcon-add'"></text>
				</view>
				<view class="flex-sub flex align-center justify-center">
					<button open-type="share" @click="share">
						<view class= "flex align-center justify-center sharbutton">
							<text class="cuIcon-share"></text>
						</view>
					</button>
				</view>
				<view class="flex-sub flex align-center justify-center" @click="like">
					<text :class="islike ? 'cuIcon-likefill' : 'cuIcon-like'"></text>
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
						<scroll-view scroll-y class="tab-scroll">
							<view class="tab-intro padding">
								测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本
							</view>
						</scroll-view>
					</swiper-item>
					<swiper-item>
						<scroll-view scroll-y class="tab-scroll">
							<message v-for="(item, index) in messagelist" :key="index" :activity="activity" :sponsor="sponsor" :message="item"
							 @appreciate="appreciate" @sponsorPage="sponsorPage"></message>
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
				id: 0,
				url: "/static/cardback0.jpg",
				activity: {
					event_id: 0,
					event_name: '活动名',
					event_introdution: '活动介绍语',
					event_picture: '',
					left_tickets: 80,
					event_location: '活动地点',
					start_time: '2019年xx月xx日',
					end_time: '',
					sponsor_name: 'xx学生会',
					event_type: 1,
					event_status: 200,
					reserved: false,
					like: false
				},
				islike: false,
				isreserved: false,
				current: 0,
				tabs: [
					"介绍", "动态"
				],
				sponsor: {
					avatarUrl: '',
					name: 'xx学生会',
					id: 0,
				},
				messagelist: []
				//[{
				// 	"id": 0,
				// 	"text": '1231241524',
				// 	"appreciate": false
				// },{
				// 	"id": 1,
				// 	"text": '1231241524124125',
				// 	"appreciate": false
				// }]
			};
		},
		onLoad(option) {
			console.log(option)
			this.id = option.id
			if (app.globalData.cookie != '') {
				console.log('indexonload')
				this.loadpage()
			} else {
				console.log('nocookie')
				app.globalData.cookieReadyCallback = this.loadpage
			}
			uni.showShareMenu({})
		},
		onShow() {
			console.log(getCurrentPages())
		},
		onShareAppMessage(res) {
			return {
				title: '清易票-' + this.activity.event_name,
				//imageUrl:
			}
		},
		methods: {
			loadpage() {
				uni.request({
					url: app.globalData.apiurl + 'events/view',
					method: 'POST',
					data: {
						event_id: this.id,
					},
					header: {
						'content-type': 'application/json', //自定义请求头信息
						'cookie': app.globalData.cookie
					},
					success: (res) => {
						this.activity = res.data
						this.sponsor.name = this.activity.sponsor_name
						console.log(res)
						console.log(this.activity.event_status)
					}
				})
				uni.request({
					url: app.globalData.apiurl + 'users/like/' + this.id,
					header: {
						'content-type': 'application/json', //自定义请求头信息
						'cookie': app.globalData.cookie
					},
					success: (res) => {
						console.log(res.data)
						this.islike = res.data.like
					}
				})
				uni.request({
					url: app.globalData.apiurl + 'users/book/' + this.id,
					header: {
						'content-type': 'application/json', //自定义请求头信息
						'cookie': app.globalData.cookie
					},
					success: (res) => {
						console.log(res)
						this.isreserved = res.data.enrolled
					}
				})
			},
			tabSelect(e) {
				this.current = e.currentTarget.dataset.id;
				console.log(this.current)
				if ((this.current == 1) && (!this.messagelist[0])) {
					uni.request({
						url: app.globalData.apiurl + 'events/moments/' + this.activity.event_id, //仅为示例，并非真实接口地址。
						header: {
							'content-type': 'application/json', //自定义请求头信息
							'cookie': app.globalData.cookie
						},
						success: (res) => {
							console.log(res.data);
							this.messagelist = res.data.moments
						}
					});
				}
			},
			navChange(index) {
				this.current = index;
			},
			swiperChange(e) {
				this.current = e.detail.current;
				console.log(this.current)
				if ((this.current == 1) && (!this.messagelist[0])) {
					uni.request({
						url: app.globalData.apiurl + 'events/moments/' + this.activity.event_id, //仅为示例，并非真实接口地址。
						method: 'GET',
						header: {
							'content-type': 'application/json', //自定义请求头信息
							'cookie': app.globalData.cookie
						},
						success: (res) => {
							console.log(res.data);
							this.messagelist = res.data.moments
						}
					});
				}
			},
			reserve() {
				console.log('reserve')
				if (this.isreserved) {
					uni.showModal({
						title: '',
						content: '请确认是否取消参加\r\n' + this.activity.event_name,
						success: (res) => {
							if (res.confirm) {
								uni.request({
									url: app.globalData.apiurl + 'users/book/' + this.activity.event_id, //仅为示例，并非真实接口地址。
									method: 'DELETE',
									header: {
										'content-type': 'application/json', //自定义请求头信息
										'cookie': app.globalData.cookie
									},
									success: (res) => {
										console.log(res);
										if (res.data.success) {
											this.isreserved = false
											uni.showToast({
												icon: 'none',
												title: '取消成功'
											})
										} else {
											uni.showToast({
												icon: 'none',
												title: '操作失败'
											})
										}
									},
									fail: (res) => {
										console.log(res)
									}
								})
							}
						}
					})
				} else {
					uni.showModal({
						title: '',
						content: '请确认是否参加\r\n' + this.activity.event_name,
						success: (res) => {
							if (res.confirm) {
								uni.request({
									url: app.globalData.apiurl + 'users/book/' + this.activity.event_id, //仅为示例，并非真实接口地址。
									method: 'POST',
									header: {
										'content-type': 'application/json', //自定义请求头信息
										'cookie': app.globalData.cookie
									},
									success: (res) => {
										console.log(res);
										if (res.data.success) {
											this.isreserved = true
											uni.showToast({
												icon: 'none',
												title: '抢票成功'
											})
										} else {
											uni.showToast({
												icon: 'none',
												title: '抢票失败'
											})
										}
									},
									fail: (res) => {
										console.log(res);
									}
								})
							}
						}
					})
				}
			},
			share() {
				return 0
			},
			like() {
				uni.request({
					url: app.globalData.apiurl + 'users/like/' + this.activity.event_id,
					method: 'POST',
					header: {
						'content-type': 'application/json', //自定义请求头信息
						'cookie': app.globalData.cookie
					},
					success: (res) => {
						console.log(res.data)
						this.islike = res.data.like
					}
				});
			},
			appreciate(id) {
				uni.request({
					url: app.globalData.apiurl + 'users/like',
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
			sponsorPage() {
				var page = getCurrentPages()
				page = page[page.length - 2]
				if (page.route == 'pages/sponsor/sponsor' && page.options.id == this.sponsor.name) {
					uni.navigateBack()
				} else {
					uni.navigateTo({
						url: "../sponsor/sponsor?id=" + this.sponsor.name
					})
				}
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
		height: 240rpx;
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
</style>
