<template>
	<view class="bg-message">
		<view class="padding-top padding-lr">
		<view class="flex justify-between">
			<view class="flex text-left" @click="sponsorPage">
				<view class="cu-avatar round" :style="{backgroundImage: 'url(' + sponsor.avatarUrl + ')'}"></view>
				<view class="flex-column padding-left-sm">
					<view class="text-sm text-bold">{{sponsor.name}}</view>
					<view class="text-xs">{{activity.start}}</view>
				</view>
			</view>
			<view class="text-right">{{activity.name}}</view>
		</view>
		<view class="message-content padding-tb-sm solid-bottom">123</view>
		</view>
		<view class="toolbar flex align-stretch text-center text-xxl">
			<view class="flex-sub flex align-center justify-center" @click="$emit('reserve', message.id)">
				<text :class="appreciate ? 'cuIcon-appreciate' : 'cuIcon-appreciatefill'" @click.stop="appreciate=!appreciate"></text>
			</view>
			<view class="flex-sub flex align-center justify-center" @click="$emit('share', message.id)">
				<text class="cuIcon-share" @click.stop=""></text>
			</view>
			<view class="flex-sub flex align-center justify-center" @click="like">
				<text class="cuIcon-moreandroid" @click.stop=""></text>
			</view>
		</view>
	</view>
</template>

<script>
	export default {
		name: 'message',
		data() {
			return {
				appreciate: true,
			}
		},
		props: {
			activity: {
				type: Object
			}, // 标题
			sponsor: {
				type: Object
			},
			message: {
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

<style>
	.toolbar {
		height: 80rpx;
		width: 100%;
	}
</style>
