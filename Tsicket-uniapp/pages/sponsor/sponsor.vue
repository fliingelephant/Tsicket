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
							<view class="text-xl text-bold">{{sponsor.tostart}}</view>
							待开始
						</view>
						<view class="padding-right-xl">
							<view class="text-xl text-bold">{{sponsor.history}}</view>
							历史活动
						</view>
						<view class="padding-right-xl">
							<view class="text-xl text-bold">{{sponsor.message}}</view>
							动态
						</view>
					</view>
				</view>
				<view class="cu-avatar xxl round" :style="{backgroundImage: 'url(' + sponsor.avatarUrl + ')'}"></view>
			</view>
			<view class="toolbar flex align-stretch text-center">
				<view class="flex-sub flex align-center justify-center" @click="share">
					<text class="cuIcon-share" @click.stop=""></text>
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
						<scroll-view scroll-y class="tab-scroll">
							<activity-mini-card v-for="(item,index) in activitylist" :key="index" :activity="item" @like="like" @clickCard="activityPage"></activity-mini-card>
						</scroll-view>
					</swiper-item>
					<swiper-item>
						<scroll-view scroll-y class="tab-scroll">
							<message v-for="(item, index) in messagelist" :key="index" :activity="item.activity" :sponsor="sponsor" :message="item" @appreciate="appreciate" @activityPage='activityPage'></message>
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
				activitylist: [{
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
						like: false
					},
					{
						id: 1,
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
						like: false
					},
					{
						id: 2,
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
						like: false
					}
				],
				current: 0,
				tabs: [
					"活动", "动态"
				],
				sponsor: {
					id: 0,
					avatarUrl: '',
					name: 'xx学生会',
					tostart: 2,
					history: 13,
					message: 28,
					follow: true,
				},
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
						}
					}
				]
			};
		},
		onLoad(option) {
			console.log(option)
			// uni.request({
			// 	url: 'http://154.8.167.168:8080', //仅为示例，并非真实接口地址。
			// 	data: {
			// 		id: option.id
			// 	},
			// 	header: {
			// 		'content-type': 'application/json' //自定义请求头信息
			// 	},
			// 	success: (res) => {
			// 		console.log(res.data);
			// 	}
			// });
			return 0
		},
		methods: {
			tabSelect(e) {
				this.current = e.currentTarget.dataset.id;
			},
			navChange(index) {
				this.current = index;
			},
			swiperChange(e) {
				this.current = e.detail.current;
			},
			reserve() {
				uni.request({
					url: 'http://154.8.167.168:8080', //仅为示例，并非真实接口地址。
					data: {
						activityid: this.id,
						userid: app.globalData.userInfo
					},
					header: {
						'content-type': 'application/json' //自定义请求头信息
					},
					success: (res) => {
						console.log(res.data);
					}
				});
			},
			share() {

			},
			follow() {
				uni.request({
					url: 'http://2019-a18.iterator-traits.com:8080/apis/users/follow',
					method: 'POST',
					data: {
						openid: app.globalData.openid,
						sponsorid: this.sponsor.id,
						session: '',
					},
					header: {
						'content-type': 'application/json' //自定义请求头信息
					},
					success: (res) => {
						console.log(res.data);
					}
				});
				this.sponsor.follow = !this.sponsor.follow
			},
			appreciate(id){
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
				var index = this.messagelist.findIndex((item) => {return item.id == id})
				console.log(index)
				this.messagelist[index].appreciate = !this.messagelist[index].appreciate
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
			like(id) {
				uni.request({
					url: 'http://2019-a18.iterator-traits.com:8080/apis/users/like',
					method: 'POST',
					data: {
						openid: app.globalData.openid,
						eventid: id,
						session: '',
					},
					header: {
						'content-type': 'application/json' //自定义请求头信息
					},
					success: (res) => {
						console.log(res.data);
					}
				});
				var index = this.activitylist.findIndex((item) => {return item.id == id})
				console.log(index)
				this.activitylist[index].like = !this.activitylist[index].like
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
