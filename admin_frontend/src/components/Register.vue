<template>
  <el-container class="container">
    <el-header class="header">
      <div>活动发布者注册</div>
    </el-header>
    <el-main>
      <el-form>
        <el-form-item>
          <el-row :gutter="20">
            <el-col :span="4"><div class="register-text"><a class="compulsory">*</a>用户名</div></el-col>
            <el-col :span="12"><el-input
                    type="text"
                    v-model="username"
                    auto-complete="off"
            ></el-input></el-col>
            <el-col :span="8"><div class="hint-text">字母开头，6至32字符(字母、数字、下划线)</div></el-col>
          </el-row>
        </el-form-item>
        <el-form-item>
          <el-row :gutter="20">
            <el-col :span="4"><div class="register-text"><a class="compulsory">*</a>密码</div></el-col>
            <el-col :span="12"><el-input
                    type="password"
                    v-model="password"
                    auto-complete="off"
            ></el-input></el-col>
            <el-col :span="8"><div class="hint-text">6至32字符(字母、数字、特殊符号)</div></el-col>
          </el-row>
        </el-form-item>
        <el-form-item >
          <el-row :gutter="20">
            <el-col :span="4"><div class="register-text"><a class="compulsory">*</a>确认密码</div></el-col>
            <el-col :span="12"><el-input
                    type="password"
                    v-model="confirm"
                    auto-complete="off"
            ></el-input></el-col>
            <el-col :span="8"><div class="hint-text">6至32字符(字母、数字、特殊符号)</div></el-col>
          </el-row>
        </el-form-item>
        <el-form-item>
          <el-row :gutter="20">
            <el-col :span="4"><div class="register-text"><a class="compulsory">*</a>机构/个人名称</div></el-col>
            <el-col :span="12"><el-input
                    type="text"
                    v-model="name"
                    auto-complete="off"
            ></el-input></el-col>
          </el-row>
        </el-form-item>
        <el-form-item>
          <el-row :gutter="20">
            <el-col :span="4"><div class="register-text"><a class="compulsory">*</a>电子邮箱</div></el-col>
            <el-col :span="12"><el-input
                    type="text"
                    v-model="email"
                    auto-complete="off"
            ></el-input></el-col>
          </el-row>
        </el-form-item>
        <el-form-item>
          <el-row :gutter="20">
            <el-col :span="4"><div class="register-text"><a class="compulsory">*</a>联系电话</div></el-col>
            <el-col :span="12"><el-input
                    type="text"
                    v-model="phone"
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
                username_regular:/^[a-zA-z][0-9a-zA-Z_]{5,31}$/,
                password_regular:/^[\S]{6,32}$/,
            }
        },
        methods:{
            reg(){
                if(this.password===this.confirm && this.password_regular.test(this.password)&& this.username_regular.test(this.username)){
                    let data={
                        "sponsorname":this.name,
                        "id":this.username,
                        "password": this.password,
                    }
                    this.$axios.post("/sponsors/register", data).then(response => {
                        if(response.status===200) {
                            this.$message({
                                message: '注册成功',
                                type: 'success'
                            })
                            this.$router.push('/')
                        }
                        else{
                            this.$message({
                                message: '注册失败',
                                type: 'error'
                            })
                        }
                    },err=>{
                        if(err.response.status===422)
                            this.$message({
                                message: '登录失败：用户不存在',
                                type: 'error'
                            })
                    })
                }
                else if(!this.username_regular.test(this.username)){
                    this.$message({
                        message:'用户名不符合要求',
                        type:'warning'
                    });
                }
                else if(!this.password_regular.test(this.password)){
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
  .container{
    margin: 10px;
    padding: 5px;
    font-weight: 600;
    font-size: 25px;
    text-align: left;
  }

  .header {
    display: flex;
  }
  .register-text {
    font-weight: 400;
    font-size: 17px;
  }
  .hint-text {
    font-weight: 400;
    font-size: 16px;
  }
  .compulsory{
    color:#ff0000
  }

</style>