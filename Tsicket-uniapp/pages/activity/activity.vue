<template>
	<view class="flex-page">
		<view class="collapse">
			<image :src="url" class="idcard"></image>
			<view class="card-back"></view>
			<view class="info padding flex align-start justify-between text-white">
				<view class="text-left flex-column info-flex">
					<view class="text-xxl">{{activity.name}}
						<view class="line-round"></view>
					</view>
					<view class="text-df">{{activity.intro}}</view>
					<view class="text-df">
						<view>{{activity.location}}</view>
						<view>{{activity.start}}</view>
					</view>
				</view>
				<view class="text-right flex-column info-flex">
					<view>
						<view class="text-sm">抢票制</view>
						<view class="text-sm">余票:{{activity.tickets}}</view>
					</view>
					<view class="flex align-end justify-end">
						<view class="padding-right-xs text-sm">{{activity.sponsorname}}</view>
						<view class="cu-avatar round solids"></view>
					</view>
				</view>
			</view>
			<view class="toolbar flex align-stretch text-center">
				<view class="flex-sub flex align-center justify-center" @click="reserve">
					<text class="cuIcon-share" @click.stop=""></text>
				</view>
				<view class="flex-sub flex align-center justify-center" @click="share">
					<text class="cuIcon-share" @click.stop=""></text>
				</view>
				<view class="flex-sub flex align-center justify-center" @click="like">
					<text :class="like ? 'cuIcon-likefill' : 'cuIcon-like'" @click.stop="$emit('like', activity.id)"></text>
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
							<!-- <view class="bg-message padding-top padding-lr">
								<view class="flex justify-between">
									<view class="flex text-left">
										<view class="cu-avatar round" :style="{backgroundImage: 'url(' + userInfo.avatarUrl + ')'}"></view>
										<view class="flex-column padding-left-sm">
											<view class="text-sm text-bold">{{activity.orgnizationname}}</view>
											<view class="text-xs">{{activity.start}}</view>
										</view>
									</view>
									<view class="text-right">{{activity.name}}</view>
								</view>
								<view class="message-content padding-tb-sm solid-bottom">123</view>
								<view class="toolbar flex align-stretch text-center">
									<view class="flex-sub flex align-center justify-center" @click="reserve">
										<view class="">加入</view>
									</view>
									<view class="flex-sub flex align-center justify-center" @click="share">
										<view class="">分享</view>
									</view>
									<view class="flex-sub flex align-center justify-center" @click="like">
										<view class="">喜爱</view>
									</view>
								</view>
							</view> -->
							<message :activity="activity" :sponsor="sponsor" :message="message"></message>
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
				like: false,
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
			like() {

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
	}

	.tab-swiper-view {
		height: calc(100vh - 480rpx - 80rpx)
	}
</style>
