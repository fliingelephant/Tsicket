(global["webpackJsonp"]=global["webpackJsonp"]||[]).push([["pages/index/index"],{"0031":function(t,o,e){"use strict";e.r(o);var a=e("ede1"),i=e("1136");for(var n in i)"default"!==n&&function(t){e.d(o,t,function(){return i[t]})}(n);e("281a");var l=e("2877"),c=Object(l["a"])(i["default"],a["a"],a["b"],!1,null,null,null);o["default"]=c.exports},1136:function(t,o,e){"use strict";e.r(o);var a=e("57e9"),i=e.n(a);for(var n in a)"default"!==n&&function(t){e.d(o,t,function(){return a[t]})}(n);o["default"]=i.a},1921:function(t,o,e){"use strict";(function(t){e("d9cf"),e("921b");a(e("66fd"));var o=a(e("0031"));function a(t){return t&&t.__esModule?t:{default:t}}t(o.default)}).call(this,e("543d")["createPage"])},"281a":function(t,o,e){"use strict";var a=e("f4f7"),i=e.n(a);i.a},"57e9":function(t,o,e){"use strict";(function(t){Object.defineProperty(o,"__esModule",{value:!0}),o.default=void 0;var e=getApp(),a={data:function(){return{keyword:"",scrollTop:0,activityindex:0,reload:!1,cardCur:0,more:!0,swiperList:[],dotStyle:!1,url:"/static/cardback0.jpg",activitylist:[],current:0,tabs:["介绍","动态"]}},onLoad:function(){console.log(e.globalData.cookie),""!=e.globalData.cookie?(console.log("indexonload"),this.loadpage()):(console.log("nocookie"),e.globalData.cookieReadyCallback=this.loadpage),t.showShareMenu({})},onPageScroll:function(t){this.scrollTop=t.scrollTop},onPullDownRefresh:function(){this.more=!0,this.activityindex=0,this.loadpage()},onReachBottom:function(){this.loadactivity(!1)},onShareAppMessage:function(t){return{title:e.globalData.sharetitle,path:"/pages/index/index",imageUrl:e.globalData.shareimg}},methods:{loadpage:function(){var o=this;t.showLoading({title:"加载中"}),t.request({url:e.globalData.apiurl+"users/broadcast",header:{"content-type":"application/json",cookie:e.globalData.cookie},success:function(e){o.swiperList=[],o.reload=!1,console.log(e),console.log(e.data),o.swiperList=e.data.list,o.reload=!0,o.activitylist!=[]&&t.stopPullDownRefresh()}}),this.loadactivity(!0)},loadactivity:function(o){var a=this;this.more&&t.request({url:e.globalData.apiurl+"users/index",data:{index:this.activityindex},header:{"content-type":"application/json",cookie:e.globalData.cookie},success:function(i){console.log(i),console.log(i.data),i.data.events?(i.data.events.forEach(function(t,o){t.img_url&&""!=t.img_url||(t.img_url=e.globalData.backimg[parseInt("11"+t.event_id)%4]),t.delay=.1*(o+5)+"s",setTimeout(function(){t.delay=void 0},100*(o+11))}),o?(a.activitylist=[],t.showToast({title:"加载成功",icon:"none"})):setTimeout(function(){t.pageScrollTo({scrollTop:a.scrollTop+300,duration:500}),console.log("top"+a.scrollTop)},200),a.activitylist=a.activitylist.concat(i.data.events),a.activityindex+=i.data.events.length,a.more=i.data.more,console.log(a.activitylist)):t.showToast({title:"加载失败",icon:"none"}),a.swiperList!=[]&&t.stopPullDownRefresh()}})},cardSwiper:function(t){this.cardCur=t.detail.current},swiperActivity:function(t){this.activityPage(this.swiperList[this.cardCur].event_id)},activityPage:function(o){t.navigateTo({url:"../activity/activity?id="+o})},InputBlur:function(t){console.log(t)},searchPage:function(){console.log(this.keyword),t.navigateTo({url:"../search/search?keyword="+this.keyword})},like:function(o){var a=this;t.request({url:e.globalData.apiurl+"users/like/"+this.activitylist[o].event_id,method:"POST",header:{"content-type":"application/json",cookie:e.globalData.cookie},success:function(t){console.log(t.data),console.log(o),a.activitylist[o].like=t.data.like}})}}};o.default=a}).call(this,e("543d")["default"])},ede1:function(t,o,e){"use strict";var a=function(){var t=this,o=t.$createElement;t._self._c},i=[];e.d(o,"a",function(){return a}),e.d(o,"b",function(){return i})},f4f7:function(t,o,e){}},[["1921","common/runtime","common/vendor"]]]);