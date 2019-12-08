import Vue from 'vue';
import router from "./router";
import ElementUI from 'element-ui';
import 'element-ui/lib/theme-chalk/index.css';
import App from './App.vue';
import axios from 'axios';
import store from './store';

Vue.use(ElementUI);

Vue.prototype.$axios = axios
axios.defaults.baseURL = '/apis'

new Vue({
  el: '#app',
  router,
  store,
  render: h => h(App)
});

