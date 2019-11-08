import Vue from 'vue'
import Vuex from 'vuex'
import createPersistedState from 'vuex-persistedstate'

Vue.use(Vuex);

export default new Vuex.Store({
    state: {
        username: '',
        admin: false
    },
    mutations: {
        login(state, val){
            state.username = val.username
            state.admin = val.admin
        },
        logout(state){
            state.username = ''
            state.admin = false
        }
    },
    plugins:[createPersistedState()]
})