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
						<scroll-view scroll-y class="tab-scroll">
							<activity-mini-card v-for="(item,index) in likelist" :key="index" :activity="item" @like="like" @clickCard="activityPage"></activity-mini-card>
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
				likelist: [{
						id: 0,
						name: '活动名',
						intro: '活动介绍语',
						tickets: 80,
						location: '活动地点',
						start: '2019年xx月xx日',
						end: '',
						sponsorid: 100,
						sponsorname: 'xx学生会',
						type: 1,
						state: 200,
						like: true
					},
					{
						id: 1,
						name: '活动名1',
						intro: '活动介绍语',
						tickets: 80,
						location: '活动地点',
						start: '2019年xx月xx日',
						end: '',
						sponsorid: 100,
						sponsorname: 'xx学生会',
						type: 1,
						state: 200,
						like: true
					},
					{
						id: 2,
						name: '活动名2',
						intro: '活动介绍语',
						tickets: 80,
						location: '活动地点',
						start: '2019年xx月xx日',
						end: '',
						sponsorid: 100,
						sponsorname: 'xx学生会',
						type: 1,
						state: 200,
						like: true
					},
					{
						id: 3,
						name: '活动名3',
						intro: '活动介绍语',
						tickets: 80,
						location: '活动地点',
						start: '2019年xx月xx日',
						end: '',
						sponsorid: 100,
						sponsorname: 'xx学生会',
						type: 1,
						state: 200,
						like: true
					},
					{
						id: 4,
						name: '活动名4',
						intro: '活动介绍语',
						tickets: 80,
						location: '活动地点',
						start: '2019年xx月xx日',
						end: '',
						sponsorid: 100,
						sponsorname: 'xx学生会',
						type: 1,
						state: 200,
						like: true
					}
				],
				current: 0,
				tabs: [
					"喜爱"
				]
			};
		},
		onLoad() {

		},
		methods: {
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
			activityPage(id) {
				uni.navigateTo({
					url: "../activity/activity?id=" + this.likelist.find(item => {
						return item.id == id
					}).id
				})
			},
			like(id) {
				uni.request({
					url: 'http://2019-a18.iterator-traits.com:8080/apis/users/like', //仅为示例，并非真实接口地址。
					method: 'POST',
					data: {
						openid: app.globalData.openid,
						eventid: id
					},
					header: {
						'content-type': 'application/json' //自定义请求头信息
					},
					success: (res) => {
						console.log(res.data);
						//var index = this.likelist.findIndex(item => {return item.id == id})
						//this.likelist[index].like = !this.likelist[index].like
					}
				})
				var index = this.likelist.findIndex(item => {
					return item.id == id
				})
				this.likelist[index].like = !this.likelist[index].like
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
