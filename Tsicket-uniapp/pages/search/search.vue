<template>
	<view>
		<view class="cu-bar search animation-fade">
			<!-- <text class="cuIcon-roundadd margin-left" style="font-size: 48rpx"></text> -->
			<view class="search-form round">
				<text class="cuIcon-search"></text>
				<input v-model="keyword" :adjust-position="false" type="text" placeholder="根据活动名,组织搜索活动" confirm-type="search"
				 @confirm="newsearch"></input>
			</view>
			<view class="action">
				<button class="cu-btn shadow-blur round" @click="newsearch">搜索</button>
			</view>
		</view>
		<view class="padding">
			<view class="flex-column">
				<activity-mini-card v-for="(item,index) in activitylist" :class="[item.delay?'animation-slide-bottom':'']" :style="[{animationDelay: item.delay}]"
				 :key="index" :activity="item" @like="like(index)" @clickCard="activityPage"></activity-mini-card>
				<view v-if="noresult" class="text-center text-gray">这里空空如也</view>
			</view>
		</view>
	</view>
</template>

<script>
	const app = getApp()

	export default {
		data() {
			return {
				scrollTop: 0,
				keyword: '',
				searchindex: 0,
				activitylist: [],
				more: true,
				noresult: false
			}
		},
		onLoad(option) {
			console.log(option)
			this.keyword = option.keyword
			this.search()
		},
		onPageScroll(res) {
			this.scrollTop = res.scrollTop
		},
		onReachBottom() {
			this.search()
		},
		methods: {
			newsearch() {
				this.searchindex = 0
				this.activitylist = []
				this.more = true
				this.noresult = false
				this.search()
			},
			search() {
				if (this.more) {
					uni.request({
						url: app.globalData.apiurl + 'users/search',
						method: 'POST',
						data: {
							index: this.searchindex,
							keyword: this.keyword
						},
						header: {
							'content-type': 'application/json', //自定义请求头信息
							'cookie': app.globalData.cookie
						},
						success: (res) => {
							console.log(res.data);
							if (res.data.events && res.data.events[0]) {
								res.data.events.forEach((res, index) => {
									res.delay = '' + (index + 5) * 0.1 + 's'
									if(!res.img_url || (res.img_url == '') ) {
										res.img_url =  app.globalData.backimg[parseInt('11' + res.event_id) % 4]
									}
									setTimeout(() => {
										res.delay = undefined
									}, (index + 11) * 100)
								})
								if (this.searchindex != 0) {
									setTimeout(() => {
										uni.pageScrollTo({
											scrollTop: this.scrollTop + 300,
											duration: 500,
										})
										console.log("top" + this.scrollTop)
									}, 200)
								}
								this.activitylist = this.activitylist.concat(res.data.events)
								this.searchindex += res.data.events.length
								this.more = res.data.more
								console.log(this.activitylist)
							} else {
								this.noresult = true
								uni.showToast({
									title: "没有找到相关活动",
									icon: 'none'
								})
							}
						}
					})
				}
			},
			activityPage(id) {
				uni.navigateTo({
					url: "../activity/activity?id=" + id,
				})
			},
			like(index) {
				uni.request({
					url: app.globalData.apiurl + 'users/like/' + this.activitylist[index].event_id,
					method: 'POST',
					header: {
						'content-type': 'application/json', //自定义请求头信息
						'cookie': app.globalData.cookie
					},
					success: (res) => {
						console.log(res.data)
						console.log(index)
						this.activitylist[index].like = res.data.like
					}
				});
			}
		}
	}
</script>

<style>

</style>
