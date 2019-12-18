<template>
	<view class="follow-list">
		<view v-for="(item, index) in followlist" :key="index" class="listitem padding-lr flex align-center justify-between text-black"
		 :class="[item.delay?'animation-slide-right':'']" :style="[{animationDelay: item.delay}]" @click="$emit('clickitem',index)">
			<view class="flex align-center">
				<view class="cu-avatar round margin-right" :style="{backgroundImage: 'url(' + item.avatar_url + ')'}"></view>
				<text>{{item.name}}</text>
			</view>
			<text class="follow-icon" :class="item.follow ? 'cuIcon-check' : 'cuIcon-friendadd'" @click.stop="$emit('follow',index)"></text>
		</view>
	</view>
</template>

<script>
	export default {
		name: 'activity-mini-card',
		props: {
			followlist: {
				type: Array
			}
		},
		methods: {
			sponsorPage() {
				return 0
			}
		}
	}
</script>

<style>
	.listitem {
		margin-bottom: 20rpx;
		width: 100%;
		height: 100rpx;
		border-radius: 20rpx 20rpx 20rpx 20rpx;
		background-color: #F2F2F2;
	}

	.collapse {
		width: 100%;
		padding: 40rpx 40rpx 40rpx 40rpx;
	}

	.imgcard {
		position: absolute;
		width: 670rpx;
		height: 250rpx;
		border-radius: 20rpx 20rpx 20rpx 20rpx;
		z-index: -1;
	}

	.card-back {
		position: absolute;
		width: 670rpx;
		height: 250rpx;
		border-radius: 20rpx 20rpx 20rpx 20rpx;
		background-color: #007AFF;
		z-index: -2;
	}

	.line-round {
		margin-top: 5rpx;
		margin-bottom: 15rpx;
		width: 70rpx;
		height: 10rpx;
		border-radius: 5rpx 5rpx 5rpx 5rpx;
		background-color: gainsboro;
	}

	.info {
		height: 250rpx;
	}

	.info-flex {
		height: 190rpx;
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

	.like {
		font-size: 60rpx;
	}

	.follow-icon {
		font-size: 48rpx;
	}

	.cuIcon-likefill {
		transition: opacity 1s ease-in 0s;
	}
</style>
