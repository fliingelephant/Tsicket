<template>
  <div id="app">
    <el-row class = "topbar" type="flex" justify="space-between">
      <el-col :span="20" class="topbar-tittle">
        <div>清易票活动平台</div>
      </el-col>
      <el-col :span="4" class="topbar-info">
        <el-dropdown :hide-on-click="false" @command="handleCommand" v-model="username">
        <span class="el-dropdown-link" style="cursor: pointer;">
        <p style="display:inline">{{$store.state.username}}</p>
        </span>
        <el-dropdown-menu  slot="dropdown">
          <el-dropdown-item command="logout">退出登录</el-dropdown-item>
        </el-dropdown-menu>
        </el-dropdown>
      </el-col>
    </el-row>
    <router-view/>
  </div>

</template>

<script>
export default {
  name: 'app',
  components: {

  },
    data(){
      return{
          username:''
      }
    },
  methods: {
      handleCommand(command) {
          this.$axios
              .post("/logout")
              .then(res=>{
                  this.$store.commit('logout')
                  this.$router.push('/')
              })
      },
  }
}
</script>

<style>
  #app {
    font-family: 'Microsoft YaHei', Helvetica, Arial, sans-serif;
    text-align: center;
    margin: auto;
  }
  .topbar {
    border-bottom: solid 1px;
    width: 100%;
    border-color:#3f51b5;
    margin: auto;
    align-items: flex-end;
    margin-bottom: 20px;
  }
  .topbar-tittle {
    color:#038bff;
    font-weight: 600;
    font-size: 30px;
    text-align: left;
    margin-left: 20px;
    margin-bottom: 10px;
  }
</style>
