<template>
  <el-container class="container">

    <el-header class="header">
      <div>管理员登录</div>
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

        <el-form-item>
          <div align="left" class="login-text">密码：</div>
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
                let data={
                    "admin_id":this.username,
                    "password":this.password
                }
                this.$axios.post("/admins/login", data).then(response => {
                    if (response.status === 200) {
                        let resdata = {
                            username: this.username,
                            admin: true
                        }
                        this.$store.commit('login', resdata)
                        this.$message({
                            message: '登录成功',
                            type: 'success'
                        })
                        this.$router.push('/AdminMenu')
                    } else {
                        this.$message({
                            message: '登录失败',
                            type: 'error'
                        })
                    }
                },err=>{
                    if(err.response.status===422)
                        this.$message({
                            message: '登录失败：用户名或密码错误',
                            type: 'error'
                        })
                })
            },
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
  .login-text {
    font-weight: 400;
    font-size: 16px;
  }
</style>
