<template>
  <el-container class="login-container">
    <el-header class="login-header">
      <div>登录</div>
    </el-header>
    <el-main>
      <el-form>
        <el-form-item prop="username">
          <div align="left" class="login-main">用户名：</div>
          <el-input
                  v-model="username"
                  type="text"
          ></el-input>
        </el-form-item>
        <el-form-item prop="password">
          <div align="left" class="login-main">密码：</div>
          <el-input
                  v-model="password"
                  type="password"
          ></el-input>
        </el-form-item>
      </el-form>
      <el-row>
        <el-col>
          <el-button @click="login" type="primary" style="width: 30%">登录</el-button>
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
                this.$router.push('/AdminMenu')
            },
        }
    }
</script>

<style scoped>
  .login-container{
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

  .login-header {
    text-align: center;
    border-bottom: 1px solid silver;
  }
  .retrieve-text{
    font-size: 12px;
    margin:5px
  }

  .login-main {
    font-weight: 400;
    font-size: 16px;
  }
</style>