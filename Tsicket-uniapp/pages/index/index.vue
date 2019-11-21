<template>
	<view>
		<view class="cu-bar search">
			<text class="cuIcon-roundadd margin-left" style="font-size: 48rpx"></text>
			<view class="search-form round">
				<text class="cuIcon-search"></text>
				<input @focus="InputFocus" @blur="InputBlur" :adjust-position="false" type="text" placeholder="搜索活动、组织"
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
					<image :src="item.url" mode="aspectFill" v-if="item.type=='image'"></image>
					<video :src="item.url" autoplay loop muted :show-play-btn="false" :controls="false" objectFit="cover" v-if="item.type=='video'"></video>
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
				swiperList: [{
					id: 0,
					type: 'image',
					url: ''
					// url: 'https://ossweb-img.qq.com/images/lol/web201310/skin/big37006.jpg',
				}, {
					id: 1,
					type: 'image',
					url: ''
					// url: 'https://ossweb-img.qq.com/images/lol/web201310/skin/big39000.jpg'
				}, {
					id: 2,
					type: 'image',
					url: ''
					// url: 'https://ossweb-img.qq.com/images/lol/web201310/skin/big10001.jpg'
				}],
				dotStyle: false,
				towerStart: 0,
				direction: '',
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
					},
					{
						id: 5,
						name: '活动名5',
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
						id: 6,
						name: '活动名6',
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
					"介绍", "动态"
				],
				sponsor: {
					avatarUrl: '',
					name: 'xx学生会'
				},
				message: {

				}
			};
		},
		onLoad() {

		},
		methods: {
			cardSwiper(e) {
				this.cardCur = e.detail.current
			},
			swiperActivity(e) {
				this.activityPage(this.swiperList[this.cardCur].id)
			},
			activityPage(id) {
				uni.navigateTo({
					url: "../activity/activity?id=" + id
				})
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
				var index = this.activitylist.findIndex((item) => {
					return item.id == id
				})
				console.log(index)
				this.activitylist[index].like = !this.activitylist[index].like
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
