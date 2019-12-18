<template>
	<view class="bg-message">
		<view class="padding-top padding-lr">
		<view class="flex justify-between">
			<view class="flex text-left" @click="$emit('sponsorPage', message.sponsor_name)">
				<view class="cu-avatar round" :style="{backgroundImage: 'url(' + message.sponsor_avatar + ')'}"></view>
				<view class="flex-column padding-left-sm">
					<view class="text-sm text-bold">{{message.sponsor_name}}</view>
					<view class="text-xs">{{message.time}}</view>
				</view>
			</view>
			<view class="text-right" @click.stop="$emit('activityPage', message.event_id)">{{message.event_name}}</view>
		</view>
		<view class="message-content padding-tb-sm solid-bottom">{{message.text}}</view>
		</view>
		<view class="toolbar flex align-stretch text-center text-xxl">
			<view class="flex-sub flex align-center justify-center" @click.stop="appreciate=!appreciate">
			<!-- $emit('appreciate', message.moment_id) -->
				<text :class="appreciate ? 'cuIcon-appreciatefill' : 'cuIcon-appreciate'"></text>
			</view>
			<view class="flex-sub flex align-center justify-center" @click.stop="$emit('share', message.moment_id)">
				<button open-type="share">
					<view class="flex align-center justify-center">
						<text class="cuIcon-share"></text>
					</view>
				</button>
			</view>
			<view class="flex-sub flex align-center justify-center" @click.stop="$emit('more', message.moment_id)">
				<text class="cuIcon-moreandroid"></text>
			</view>
		</view>
	</view>
</template>

<script>
	export default {
		name: 'message',
		data() {
			return {
				appreciate: false
			}
		},
		props: {
			message: {
				type: Object
			}
		}
	}
</script>

<style>
	.toolbar {
		height: 80rpx;
		width: 100%;
	}
	
	button {
		font-size: 44rpx;
		width: 100%;
		height: 100%;
		padding: 0;
		border: none;
		background: none;
	}
	
	button:after {
		border: none;
	}
	
	button>view {
		width: 100%;
		height: 100%;
	}
</style>
