<template>
	<view class="flex-page">
		<view class="userinfo">
			<image class="usercard" src="../../static/cardback0.jpg"></image>
			<view v-if="hasUserInfo">
				<view class="padding flex align-start justify-between">
					<view class="cu-avatar xxl round" :style="{backgroundImage: 'url(' + userInfo.avatarUrl + ')'}"></view>
					<view class="text-white">
						<view class="text-right">
							<view class="text-sl text-ellipsis">
								<text class="text-bold">{{userInfo.nickName}}</text>
							</view>
						</view>
						<view class="text-left flex justify-start text-sm padding">
							<view class="padding-right-xl">
								<view class="text-xl text-bold">{{like}}</view>
								喜爱
							</view>
							<view class="padding-right-xl">
								<view class="text-xl text-bold">{{follow}}</view>
								关注
							</view>
							<view class="padding-right-xl">
								<view class="text-xl text-bold">{{history}}</view>
								历史
							</view>
						</view>
					</view>
				</view>
			</view>
			<view v-else class="flex justify-center getinfobtn">
				<button class="cu-btn round" open-type="getUserInfo" @getuserinfo="getUserInfo"> 授权个人信息</button>
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
						<scroll-view scroll-y class="tab-scroll padding">
							<view class="tab-intro padding">
								测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本测试文本
							</view>
						</scroll-view>
					</swiper-item>
					<swiper-item>
						<scroll-view scroll-y class="tab-scroll">

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
				userInfo: {},
				hasUserInfo: false,
				canIUse: uni.canIUse('button.open-type.getUserInfo'),
				like: 123,
				follow: 10,
				history: 23,
				current: 0,
				tabs: [
					"活动日程", "报名中"
				]
			};
		},
		onLoad() {
			if (app.globalData.userInfo) {
				this.userInfo = app.globalData.userInfo
				this.hasUserInfo = true
			} else if (this.canIUse) {
				// 由于 getUserInfo 是网络请求，可能会在 Page.onLoad 之后才返回
				// 所以此处加入 callback 以防止这种情况
				app.userInfoReadyCallback = res => {
					this.userInfo = res.userInfo
					this.hasUserInfo = true
				}
			} else {
				// 在没有 open-type=getUserInfo 版本的兼容处理
				uni.getUserInfo({
					success: res => {
						app.globalData.userInfo = res.userInfo
						this.userInfo = res.userInfo
						this.hasUserInfo = true
					}
				})
			}
		},
		methods: {
			cardSwiper(e) {
				this.cardCur = e.detail.current
			},
			activitydetail(e) {
				uni.navigateTo({
					url: "../activity/activity"
				})
			},
			getUserInfo(e) {
				console.log(e)
				app.globalData.userInfo = e.detail.userInfo
				this.userInfo = e.detail.userInfo,
					this.hasUserInfo = true
			},
			tabSelect(e) {
				this.current = e.currentTarget.dataset.id;
			},
			navChange(index) {
				this.current = index;
			},
			swiperChange(e) {
				this.current = e.detail.current;
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
		height: calc(100vh - 400rpx - 80rpx)
	}
</style>
