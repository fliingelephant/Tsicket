<template>
  <el-container class="login-container">
    <el-header class="login-header">
      <div>活动发布者注册</div>
    </el-header>
    <el-main>
      <el-form>
        <el-form-item prop="username">
          <el-row :gutter="20">
            <el-col :span="5"><div align="left" class="login-main"><a class="compulsory">*</a>用户名</div></el-col>
            <el-col :span="11"><el-input
                    type="text"
                    v-model="username"
                    auto-complete="off"
            ></el-input></el-col>
            <el-col :span="8"><div align="left" class="login-main">6至32字符(字母、数字、特殊符号)</div></el-col>
          </el-row>
        </el-form-item>
        <el-form-item prop="password">
          <el-row :gutter="20">
            <el-col :span="5"><div align="left" class="login-main"><a class="compulsory">*</a>密码</div></el-col>
            <el-col :span="11"><el-input
                    type="password"
                    v-model="password"
                    auto-complete="off"
            ></el-input></el-col>
            <el-col :span="8"><div align="left" class="login-main">6至32字符(字母、数字、特殊符号)</div></el-col>
          </el-row>
        </el-form-item>
        <el-form-item prop="password">
          <el-row :gutter="20">
            <el-col :span="5"><div align="left" class="login-main"><a class="compulsory">*</a>确认密码</div></el-col>
            <el-col :span="11"><el-input
                    type="password"
                    v-model="confirm"
                    auto-complete="off"
            ></el-input></el-col>
            <el-col :span="8"><div align="left" class="login-main">6至32字符(字母、数字、特殊符号)</div></el-col>
          </el-row>
        </el-form-item>
        <el-form-item prop="email">
          <el-row :gutter="20">
            <el-col :span="5"><div align="left" class="login-main"><a class="compulsory">*</a>电子邮箱</div></el-col>
            <el-col :span="11"><el-input
                    type="text"
                    v-model="email"
                    auto-complete="off"
            ></el-input></el-col>
          </el-row>
        </el-form-item>
        <el-form-item prop="phone">
          <el-row :gutter="20">
            <el-col :span="5"><div align="left" class="login-main"><a class="compulsory">*</a>联系电话</div></el-col>
            <el-col :span="11"><el-input
                    type="text"
                    v-model="phone"
                    auto-complete="off"
            ></el-input></el-col>
          </el-row>
        </el-form-item>
        <el-form-item prop="name">
          <el-row :gutter="20">
            <el-col :span="5"><div align="left" class="login-main"><a class="compulsory">*</a>机构/个人名称</div></el-col>
            <el-col :span="11"><el-input
                    type="text"
                    v-model="name"
                    auto-complete="off"
            ></el-input></el-col>
          </el-row>
        </el-form-item>
      </el-form>
      <el-row>
        <el-col>
          <el-button type="primary" @click="reg" style="width: 10%">注册</el-button>
        </el-col>
      </el-row>
    </el-main>
  </el-container>
</template>

<script>
    export default {
        name: "Register",
        data() {
            return {
                username: '',
                password: '',
                confirm: '',
                email: '',
                phone: '',
                name: '',
                regular:/^[\S]{6,32}$/

            }
        },
        methods:{
            reg:function(){
                if(this.password===this.confirm && this.regular.test(this.password)&& this.regular.test(this.username)){
                    this.$message({
                        message:'注册成功',
                        type:'success'
                    })
                    this.$router.push('/')
                }
                else if(!this.regular.test(this.username)){
                    this.$message({
                        message:'用户名不符合要求',
                        type:'warning'
                    });
                }
                else if(!this.regular.test(this.password)){
                    this.$message({
                        message:'密码不符合要求',
                        type:'warning'
                    });
                }
                else if(this.password!==this.confirm){
                    this.$message({
                        message:'两次输入密码不对应',
                        type:'warning'
                    });
                }
            }
        }
    }
</script>

<style scoped>
  .login-container{

    margin: 10px;
    padding: 5px;
    font-weight: 600;
    font-size: 25px;
    text-align: left;
  }

  .login-header {
    display: flex;
    text-align: center;
  }
  .login-main {
    font-weight: 400;
    font-size: 18px;
  }
  .compulsory{
    color:#ff0000
  }

</style>