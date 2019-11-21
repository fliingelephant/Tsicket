<template>
	<view class="activity-mini-card">
		<image :src="activity.bgUrl" class="imgcard"></image>
		<view class="card-back"></view>
		<view class="info padding flex align-start justify-between text-white" @click="$emit('clickCard', activity.id)">
			<view class="text-left flex-column info-flex">
				<view class="text-xxl">{{activity.name}}
					<view class="line-round"></view>
					<view class="text-df">{{activity.intro}}</view>
				</view>
				<view class="text-df">{{activity.start}}</view>
			</view>
			<view class="text-right flex-column info-flex">
				<view></view>
				<text class="like" :class="activity.like ? 'cuIcon-likefill' : 'cuIcon-like'" @click.stop="$emit('like', activity.id)"></text>
			</view>
		</view>
	</view>
</template>

<script>
	export default {
		name: 'activity-mini-card',
		props: {
			activity: {
				type: Object
			}
		},
		methods: {
		}
	}
</script>

<style>
	.activity-mini-card{
		width: 100%;
		margin-bottom: 40rpx;
	}
	
	.collapse {
		width: 100%;
		padding: 40rpx 40rpx 40rpx 40rpx;
	}

	.imgcard {
		position: absolute;
		width: 100%;
		height: 250rpx;
		border-radius: 20rpx 20rpx 20rpx 20rpx;
		z-index: -1;
	}

	.card-back {
		position: absolute;
		width: 100%;
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
	
	.cuIcon-likefill {
		transition: opacity 1s ease-in 0s;
	}
	
	
</style>
