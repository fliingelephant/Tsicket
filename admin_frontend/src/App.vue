<template>
  <div id="app">
    <el-row class="topbar" type="flex" justify="space-between">
      <el-col :span="20" class="topbar-tittle">
        <div>清易票活动平台</div>
      </el-col>
      <el-col :span="4" class="topbar-info">
        <el-dropdown :hide-on-click="false" @command="handleCommand" v-model="username">
          <span class="el-dropdown-link" style="cursor: pointer;">
            <p v-if="!this.$store.state.admin" style="display:inline">{{$store.state.username}}</p>
            <p v-if="this.$store.state.admin" style="display:inline">管理员：{{$store.state.username}}</p>
          </span>
          <el-dropdown-menu  slot="dropdown">
            <el-dropdown-item command="logout">退出登录</el-dropdown-item>
            <el-dropdown-item v-if="!this.$store.state.admin" command="change_info">修改信息</el-dropdown-item>
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
        data(){
            return{
                username:''
            }
        },
        methods: {
            handleCommand(command) {
                if(command==="logout") {
                    if (!this.$store.state.admin) {
                        this.$axios.post("/sponsors/logout").then(res => {
                            if (res.status === 200) {
                                this.$store.commit('logout')
                                this.$router.push('/')
                            } else {
                                this.$message({
                                    message: '登出失败',
                                    type: 'error'
                                })
                            }
                        }, err => {
                            if (err.response.status === 422)
                                this.$message({
                                    message: '登出失败',
                                    type: 'error'
                                })
                        })
                    }
                    else{
                        this.$axios.post("/admins/logout").then(res => {
                            if (res.status === 200) {
                                this.$store.commit('logout')
                                this.$router.push('/AdminLogIn')
                            } else {
                                this.$message({
                                    message: '登出失败',
                                    type: 'error'
                                })
                            }
                        }, err => {
                            if (err.response.status === 422)
                                this.$message({
                                    message: '登出失败',
                                    type: 'error'
                                })
                        })
                    }
                }

                else if(command==="change_info") {
                    this.$router.push('/ChangeInfo')
                }
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
  }
  .topbar-tittle {
    color:#038bff;
    font-weight: 600;
    font-size: 30px;
    text-align: left;
    margin-left: 20px;
    margin-bottom: 10px;
  }
  .topbar-info {
    margin-top: 10px;
    margin-right: 40px;
    text-align: right;
  }
</style>
