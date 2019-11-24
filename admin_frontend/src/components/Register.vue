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
            <el-col :span="8"><div class="hint-text">后期不可修改</div></el-col>
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
                email_regular:/^([A-Za-z0-9_\-.])+@([A-Za-z0-9_\-.])+\.([A-Za-z]{2,4})$/,
                phone_regular:/^\d{11}$|^\d{7,8}$|^(\d{4}|\d{3})-(\d{7,8})$|^(\d{4}|\d{3})-(\d{7,8})-(\d{4}|\d{3}|\d{2}|\d{1})$|^(\d{7,8})-(\d{4}|\d{3}|\d{2}|\d{1})$/
            }
        },
        methods:{
            reg(){
                let password_same=this.password===this.confirm
                let password_available=this.password_regular.test(this.password)
                let username_available=this.username_regular.test(this.username)
                let email_available=this.email_regular.test(this.email)
                let phone_available=this.phone_regular.test(this.phone)
                if(password_same && password_available && username_available && email_available && phone_available && this.name!==""){
                    let data={
                        "sponsorname":this.name,
                        "id":this.username,
                        "password": this.password,
                        "email": this.email,
                        "phone_number": this.phone
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
                                message: '注册失败：用户名被占用',
                                type: 'error'
                            })
                    })
                }
                else if(!username_available){
                    this.$message({
                        message:'用户名不符合要求',
                        type:'warning'
                    });
                }
                else if(!password_available){
                    this.$message({
                        message:'密码不符合要求',
                        type:'warning'
                    });
                }
                else if(!password_same){
                    this.$message({
                        message:'两次输入密码不对应',
                        type:'warning'
                    });
                }
                else if(!email_available){
                    this.$message({
                        message:'电子邮件不合规范',
                        type:'warning'
                    });
                }
                else if(!phone_available){
                    this.$message({
                        message:'电话号码不合规范',
                        type:'warning'
                    });
                }
                else if(this.name===""){
                    this.$message({
                        message:'个人/机构名称不合规范',
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
