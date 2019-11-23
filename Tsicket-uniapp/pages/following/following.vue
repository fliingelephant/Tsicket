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
							<follow-list :followlist="followlist" @clickitem="sponsorPage" @follow="follow"></follow-list>
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
				followlist: [{
						username: 'xx社团',
						follow: true,
						id: 0
					},
					{
						username: 'xx学生会',
						follow: true,
						id: 0
					},
					{
						username: 'xx社团2',
						follow: true,
						id: 0
					},
					{
						username: 'xx学生会2',
						follow: true,
						id: 0
					},
					{
						username: 'xx社团3',
						follow: true,
						id: 0
					},
					{
						username: 'xx学生会3',
						follow: true,
						id: 0
					},
					{
						username: 'xx社团4',
						follow: true,
						id: 0
					},
					{
						username: 'xx学生会4',
						follow: true,
						id: 0
					},
					{
						username: 'xx社团5',
						follow: true,
						id: 0
					},
					{
						username: 'xx学生会5',
						follow: true,
						id: 0
					},
					{
						username: 'xx社团6',
						follow: true,
						id: 0
					},
					{
						username: 'xx学生会6',
						follow: true,
						id: 0
					}
				],
				current: 0,
				tabs: [
					"关注"
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
			sponsorPage(index) {
				uni.navigateTo({
					url: "../sponsor/sponsor?id=" + this.followlist[index].id
				})
			},
			follow(index) {
				uni.request({
					url: 'http://2019-a18.iterator-traits.com:8080/apis/users/like', //仅为示例，并非真实接口地址。
					method: 'POST',
					data: {
						openid: app.globalData.openid,
						sponsorid: this.followlist[index].id
					},
					header: {
						'content-type': 'application/json' //自定义请求头信息
					},
					success: (res) => {
						console.log(res.data);
						//this.followlist[index].follow = !this.followlist[index].follow
					}
				})
				this.followlist[index].follow = !this.followlist[index].follow
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
