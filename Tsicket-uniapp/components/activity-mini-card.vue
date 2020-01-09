<template>
	<view class="activity-mini-card" :style="{backgroundImage : activity.img_url ? 'url(' + activity.img_url + ')' : undefined}">
		<!-- <image :src="activity.img_url" class="imgcard"> -->
		<view class="padding flex align-start justify-between text-white" @click="$emit('clickCard', activity.event_id)">
			<view class="text-left flex-column info-flex">
				<view class="text-xxl">{{activity.event_name}}
					<view class="line-round"></view>
					<view class="text-df">{{activity.event_introduction}}</view>
				</view>
				<view class="text-df">{{activity.event_time}}</view>
			</view>
			<view class="text-right flex-column info-flex">
				<view></view>
				<view>
					<text class="like cuIcon-likefill" :class="{ likefill : activity.like} " @click.stop="$emit('like', activity.event_id)"></text>
					<text class="like cuIcon-like" :class="{ likefill : activity.like}" @click.stop="$emit('like', activity.event_id)"></text>
				</view>
			</view>
		</view>
		<!-- </image> -->
	</view>
</template>

<script>
	export default {
		name: 'activity-mini-card',
		computed: {
			backgroundImage: function() {
				if (this.activity.img_url) {
					return {
						'background-image': 'url(' + this.activity.img_url + ')',
					}
				} else {
					return {}
				}
			}
		},
		props: {
			activity: {
				type: Object
			}
		},
		methods: {}
	}
</script>

<style>
	.activity-mini-card {
		width: 100%;
		margin-bottom: 40rpx;
		height: 250rpx;
		border-radius: 20rpx 20rpx 20rpx 20rpx;
		/* background-color: #007AFF; */
		background-repeat: no-repeat;
		background-size: 100% 100%;
		backdrop-filter: blur(10rpx);
	}

	.collapse {
		width: 100%;
		padding: 40rpx 40rpx 40rpx 40rpx;
	}

	.imgcard {
		width: inherit;
		height: 250rpx;
		border-radius: 20rpx 20rpx 20rpx 20rpx;

	}

	.card-back {
		width: inherit;
		height: 250rpx;
		border-radius: 20rpx 20rpx 20rpx 20rpx;
		/* background-color: #007AFF; */

	}

	.line-round {
		margin-top: 5rpx;
		margin-bottom: 15rpx;
		width: 70rpx;
		height: 10rpx;
		border-radius: 5rpx 5rpx 5rpx 5rpx;
		background-color: gainsboro;
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
	
	.cuIcon-likefill.likefill {
		transition: all 0.3s ease-in 0s;
		color: #DD514C;
		opacity: 1;
	}

	.cuIcon-likefill {
		transition: all 0.3s ease-in 0s;
		color: #FFFFFF;
		position: relative;
		left: 60rpx;
		opacity: 0;
	}
	
	.cuIcon-like.likefill {
		transition: all 0.3s ease-in 0s;
		color: #DD514C;
		opacity: 0;
	}

	.cuIcon-like {
		transition: all 0.3s ease-in 0s;
		color: #FFFFFF;
		opacity: 1;
	}
</style>
