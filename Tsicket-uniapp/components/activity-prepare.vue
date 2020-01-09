<template>
	<view class="activity-prepare" :class="{'unfold': activity.isUnfold}">
		<view class="imgcard" :style="{backgroundImage: activity.img_url ?  'url(' + activity.img_url + ')':'' , backgroundRepeat : 'no-repeat', backgroundSize : '100% 100%'}" >
		<!-- <image :src="activity.bgUrl" class="imgcard"></image>
		<view class="card-back" :class="{'unfold': activity.isUnfold}"></view> -->
		<view class="info padding flex align-start justify-between text-white" @click="$emit('unfold')">
			<view class="text-left flex-column info-flex">
				<view class="text-xxl">{{activity.sponsor_name}}
					<view class="line-round"></view>
					<view class="text-df">{{activity.event_time}}</view>
					<view class="text-df">{{activity.event_location}}</view>
					<view class="text-df"><!-- 其他信息 --></view>
				</view>
				<view class="text-df"></view>
			</view>
			<view class="text-right flex-column info-flex">
				<view>
					<view class="text-sm"><!-- 票号码 --></view>
				</view>
				<view class="flex align-end justify-end" @click.stop="$emit('clickSponsor', activity.sponsor_name)">
					<view class="padding-right-xs text-sm">{{activity.sponsor_name}}</view>
					<view class="cu-avatar round solids" :style="{backgroundImage: 'url(' + activity.sponsor_avatar + ')'}"></view>
				</view>
			</view>
		</view>
		<view class="toolbar flex align-stretch text-center" :class="{'unfold':activity.isUnfold}">
			<view class="flex-sub flex align-center justify-center" @click="$emit('detail', activity.event_id)">
				<text class="cuIcon-news"></text>
				<!-- <@click.stop="$emit('detail', activity.id)"> -->
			</view>
			<!-- <view class="flex-sub flex align-center justify-center" @click="$emit('announcement', activity.event_id)">
				<text class="cuIcon-notice"></text>
			</view>
			<view class="flex-sub flex align-center justify-center" @click="$emit('QRcode', activity.event_id)">
				<text class="cuIcon-qr_code"></text>
			</view> -->
			<view class="flex-sub flex align-center justify-center" @click="$emit('cancel', activity.event_id)">
				<text class="cuIcon-delete"></text>
			</view>
			<view class="flex-sub flex align-center justify-center" @click="$emit('like', activity.event_id)">
				<text :class="activity.like ? 'cuIcon-likefill' : 'cuIcon-like' "></text>
			</view>
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
			sponsorPage() {
				return 0
			}
		}
	}
</script>
<!-- .imgcard {
		position: absolute;
		width: 100%;
		height: 300rpx;
		border-radius: 20rpx 20rpx 20rpx 20rpx;
		background-color: #007AFF;
		z-index: -1;
	} -->
<style>
	.activity-prepare {
		margin-bottom: 40rpx;
		height: 300rpx;
		transition: all 0.5s ease 0s;
		border-radius: 20rpx 20rpx 20rpx 20rpx;
		background-color: #F2F2F2;
	}
	
	.activity-prepare.unfold {
		height: 400rpx;
		transition: all 0.5s ease 0s;
	}

	.collapse {
		width: 100%;
		padding: 40rpx 40rpx 40rpx 40rpx;
	}

	.imgcard {
		width: inherit;
		height: 300rpx;
		border-radius: 20rpx 20rpx 20rpx 20rpx;
		/* background-color: #007AFF; */
	}

	.card-back {
		position: absolute;
		width: 100%;
		height: 300rpx;
		transition: all 0.5s ease 0s;
		border-radius: 20rpx 20rpx 20rpx 20rpx;
		background-color: #F2F2F2;
		z-index: -2;
	}

	.card-back.unfold {
		height: 400rpx;
		transition: all 0.5s ease 0s;
	}

	.line-round {
		margin-top: 10rpx;
		margin-bottom: 10rpx;
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
		opacity: 0;
		width: 100%;
		height: 0;
		transition: all 0.5s ease 0s;
		font-size: 48rpx;
	}

	.toolbar.unfold {
		opacity: 1;
		height: 100rpx;
		transition: all 0.5s ease 0s;
	}


	.tab-swiper-view {
		height: calc(100vh - 480rpx - 80rpx)
	}

	.cuIcon-likefill {
		transition: opacity 1s ease-in 0s;
	}
</style>
