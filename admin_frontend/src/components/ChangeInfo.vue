<template>
  <el-container class="login-container">
    <el-header class="login-header">
      <div>修改个人信息</div>
    </el-header>
    <el-main>
      <el-form ref="register">
        <el-form-item prop="username">
          <el-row :gutter="20">
            <el-col :span="5"><div align="left" class="login-main"><a class="compulsory">*</a>用户名</div></el-col>
            <el-col :span="11"><el-input
                    :disabled="true"
                    type="text"
                    v-model="username"
                    auto-complete="off"
            ></el-input></el-col>
          </el-row>
        </el-form-item>
        <el-form-item prop="name">
          <el-row :gutter="20">
            <el-col :span="5"><div align="left" class="login-main"><a class="compulsory">*</a>机构/个人名称</div></el-col>
            <el-col :span="11"><el-input
                    :disabled="true"
                    type="text"
                    v-model="name"
                    auto-complete="off"
            ></el-input></el-col>
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

      </el-form>
      <el-row>
        <el-col>
          <el-button type="primary" style="width: 10%">修改</el-button>
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
                email: '',
                phone: '',
                name: '',
                regular:/^[\S]{6,32}$/

            }
        },
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