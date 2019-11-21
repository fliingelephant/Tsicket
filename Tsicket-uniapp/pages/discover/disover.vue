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
							<message v-for="(item, index) in messagelist" :key="index" :activity="item.activity" :sponsor="item.sponsor"
							 :message="item" @appreciate="appreciate" @sponsorPage='sponsorPage' @activityPage='activityPage'></message>
						</scroll-view>
					</swiper-item>
					<swiper-item>
						<scroll-view scroll-y class="tab-scroll">
							<message v-for="(item, index) in messagelist" :key="index" :activity="item.activity" :sponsor="item.sponsor"
							 :message="item" @appreciate="appreciate" @sponsorPage='sponsorPage' @activityPage='activityPage'></message>
						</scroll-view>
					</swiper-item>
					<swiper-item>
						<scroll-view scroll-y class="tab-scroll">
							<message v-for="(item, index) in messagelist.slice(1,3)" :key="index" :activity="item.activity" :sponsor="item.sponsor"
							 :message="item" @appreciate="appreciate" @sponsorPage='sponsorPage' @activityPage='activityPage'></message>
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
				messagelist: [{
						"id": 0,
						text: "测试文本123412351123",
						"appreciate": false,
						activity: {
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
							state: 200
						},
						sponsor: {
							id: 0,
							avatarUrl: '',
							name: 'xx学生会',
						}
					},
					{
						"id": 1,
						text: "测试文本12341231231245124",
						"appreciate": false,
						activity: {
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
							state: 200
						},
						sponsor: {
							id: 1,
							avatarUrl: '',
							name: 'xx学生会2',
						}
					},
					{
						"id": 2,
						text: "测试文本123532151212341233",
						"appreciate": false,
						activity: {
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
							state: 200
						},
						sponsor: {
							id: 2,
							avatarUrl: '',
							name: 'xx学生会3',
						}
					}
				],
				current: 0,
				tabs: [
					"参加", "关注收藏", "广场"
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
			appreciate(e,id) {
				console.log(e)
				console.log(id)
				uni.request({
					url: 'http://2019-a18.iterator-traits.com:8080/apis/users/appreciate',
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
