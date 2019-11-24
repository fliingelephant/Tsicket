<template>
  <el-container class="container">
    <el-header class="header">
      <div>登录</div>
    </el-header>
    <el-main>
      <el-form>
        <el-form-item>
          <div align="left" class="login-text">用户名：</div>
          <el-input
                  v-model="username"
                  type="text"
          ></el-input>
        </el-form-item>
        <el-form-item prop="password">
          <div align="left" class="login-text">密码：</div>
          <el-input
                  v-model="password"
                  type="password"
          ></el-input>
        </el-form-item>
      </el-form>
      <el-row class="retrieve-text">
        <div align="right"> 找回密码 </div>
      </el-row>
      <el-row>
        <el-col :span="12">
          <el-button @click="login" type="primary" style="width: 60%">登录</el-button>
        </el-col>
        <el-col :span="12">
          <el-button @click="toRegister" style="width: 60%">注册</el-button>
        </el-col>
      </el-row>
    </el-main>
  </el-container>
</template>

<script>
    export default {
        name: "LogIn",
        data() {
            return {
                username: '',
                password: ''
            }
        },
        methods:{
            login(){
                let data={
                    "id":this.username,
                    "password":this.password
                }
                this.$axios.post("/sponsors/login", data).then(response => {
                    if (response.status === 200) {
                        let resdata = {
                            username: response.data,
                            admin: false
                        }
                        this.$store.commit('login', resdata)
                        this.$message({
                            message: '登录成功',
                            type: 'success'
                        })
                        this.$router.push('/EventList')
                    } else {
                        this.$message({
                            message: '登录失败',
                            type: 'error'
                        })
                    }
                },err=>{
                    if(err.response.status===422)
                        this.$message({
                            message: '登录失败：用户不存在',
                            type: 'error'
                        })})

            },
            toRegister(){
                this.$router.push('/Register')
            }
        }
    }
</script>

<style scoped>
  .container{
    border-radius: 20px;
    background-clip: padding-box;
    margin: 100px auto;
    padding: 20px;
    width: 400px;
    border: 1px solid silver;
    font-weight: 600;
    font-size: 25px;
    text-align: center;
  }

  .header {
    text-align: center;
    border-bottom: 1px solid silver;
  }
  .retrieve-text{
    font-size: 12px;
    margin:5px
  }

  .login-text {
    font-weight: 400;
    font-size: 16px;
  }
</style>
