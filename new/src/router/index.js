import Vue from 'vue'
import Router from 'vue-router'
import EventInfo from "@/components/EventInfo";
import EventList from "@/components/EventList";
import Register from "@/components/Register";
import LogIn from "@/components/LogIn";
import Help from "@/components/Help";

Vue.use(Router)

export default new Router({
    routes: [
        {
            path:'/',
            name: 'LogIn',
            component: LogIn,
            meta:{
                title:'登录'
            }
        },{
            path:'/Register',
            name: 'Register',
            component: Register,
            meta:{
                title:'注册'
            }
        },{
            path:'/EventInfo',
            name: 'EventInfo',
            component: EventInfo,
            meta:{
                title:'编辑活动'
            }
        },{
            path:'/EventList',
            name: 'EventList',
            component: EventList,
            meta:{
                title:'过往活动'
            }
        },{
            path:'/Help',
            name: 'Help',
            component: Help,
            meta: {
                title: '帮助'
            }
        }
    ]
})