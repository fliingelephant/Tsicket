import Vue from 'vue'
import Router from 'vue-router'
import EventInfo from "@/components/EventInfo";
import EventList from "@/components/EventList";
import Register from "@/components/Register";
import LogIn from "@/components/LogIn";
import Help from "@/components/Help";
import AdminLogIn from "@/components/AdminLogIn";
import AdminMenu from "@/components/AdminMenu";

Vue.use(Router)

export default new Router({
    routes: [
        {
            path:'/',
            name: 'LogIn',
            component: LogIn,
            meta:{
                hasInfo:false,
                title:'登录'
            }
        },{
            path:'/Register',
            name: 'Register',
            component: Register,
            meta:{
                hasInfo:false,
                title:'注册'
            }
        },{
            path:'/EventInfo',
            name: 'EventInfo',
            component: EventInfo,
            meta:{
                hasInfo:true,
                title:'编辑活动'
            }
        },{
            path:'/EventList',
            name: 'EventList',
            component: EventList,
            meta:{
                hasInfo:true,
                title:'过往活动'
            }
        },{
            path:'/Help',
            name: 'Help',
            component: Help,
            meta: {
                hasInfo:true,
                title: '帮助'
            }
        },
        {
            path:'/AdminLogIn',
            name: 'AdminLogIn',
            component: AdminLogIn,
            meta: {
                hasInfo:false,
                title: '管理员登陆'

            }
        },
        {
            path:'/AdminMenu',
            name: 'AdminMenu',
            component: AdminMenu,
            meta: {
                hasInfo:true,
                title: '管理员菜单'
            }
        }
    ]
})