import Vue from 'vue'
import App from './App'
import message from './components/message.vue'
import activityMiniCard from './components/activity-mini-card.vue'
import activityPrepare from "@/components/activity-prepare.vue"
import activityCheck from "@/components/activity-check.vue"
import followList from "@/components/follow-list.vue"
Vue.component('message', message)
Vue.component('activity-mini-card', activityMiniCard)
Vue.component('activity-prepare', activityPrepare)
Vue.component('activity-check', activityCheck)
Vue.component('follow-list', followList)

Vue.config.productionTip = false

App.mpType = 'app'

const app = new Vue({
	...App
})
app.$mount()
