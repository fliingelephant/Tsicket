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

router.beforeEach((to, from, next) => {
  /* 路由发生变化修改页面title */
  if (to.meta.title) {
    document.title = to.meta.title
  }
  next()
})

new Vue({
  el: '#app',
  router,
  store,
  render: h => h(App)
});


