<template>
  <el-container class="container">

    <el-header class="header">
      <div>修改个人信息</div>
    </el-header>

    <el-main>
      <el-form ref="register">
        <el-form-item prop="username">
          <el-row :gutter="20">
            <el-col :span="5"><div class="info-text"><a class="compulsory">*</a>用户ID</div></el-col>
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
            <el-col :span="5"><div class="info-text"><a class="compulsory">*</a>机构/个人名称</div></el-col>
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
            <el-col :span="5"><div class="info-text"><a class="compulsory">*</a>电子邮箱</div></el-col>
            <el-col :span="11"><el-input
                    type="text"
                    v-model="email"
                    auto-complete="off"
            ></el-input></el-col>
          </el-row>
        </el-form-item>

        <el-form-item prop="phone">
          <el-row :gutter="20">
            <el-col :span="5"><div class="info-text"><a class="compulsory">*</a>联系电话</div></el-col>
            <el-col :span="11"><el-input
                    type="text"
                    v-model="phone"
                    auto-complete="off"
            ></el-input></el-col>
          </el-row>
        </el-form-item>
        <el-form-item>
          <el-row :gutter="20">
            <el-col :span="5"><div class="info-text">头像</div></el-col>
            <el-col :span="15">
              <img :src="head_portrait" height="200">
            </el-col>
            <el-col :span="4">
              <el-button v-if="!change" @click="change=true">修改</el-button>
            </el-col>
          </el-row>
        </el-form-item>

        <el-form-item v-if="change">
          <el-row :gutter="20">
            <el-col :span="4"><div class="info-text">修改头像</div></el-col>
            <el-col :span="20">
              <!--上传图片-->
              <el-upload
                      :action="upload_url"
                      :limit="1"
                      accept="image/png,image/jpeg"
                      list-type="picture-card"
                      :before-upload="beforeUploadPicture"
                      :on-remove="handleRemove"
                      :on-success="uploadSuccess"
                      :on-error="uploadError"
                      :show-file-list="true">
                <i class="el-icon-plus"></i>
              </el-upload>
            </el-col>
          </el-row>
        </el-form-item>
      </el-form>

      <el-row>
        <el-col :span="10">
          <el-button @click="changeInfo" type="primary">修改</el-button>
          <el-button @click="pageReturn">返回</el-button>
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
                head_portrait:'',
                change:false,
                upload_url:'apis/sponsors/pic/head_portrait',
                email_regular:/^([A-Za-z0-9_\-.])+@([A-Za-z0-9_\-.])+\.([A-Za-z]{2,4})$/,
                phone_regular:/^\d{11}$|^\d{7,8}$|^(\d{4}|\d{3})-(\d{7,8})$|^(\d{4}|\d{3})-(\d{7,8})-(\d{4}|\d{3}|\d{2}|\d{1})$|^(\d{7,8})-(\d{4}|\d{3}|\d{2}|\d{1})$/
            }
        },
        mounted(){
            this.testLogIn()
            this.getInfo()
        },
        methods:{
            getInfo() {
                this.$axios.get("/sponsors/view").then(response => {
                    let userInfo=response.data
                    this.head_portrait=userInfo.head_portrait
                    this.username=userInfo.id
                    this.email=userInfo.email
                    this.phone=userInfo.phone_number
                    this.name=userInfo.sponsor_name
                })
            },
            changeInfo(){
                let email_available=this.email_regular.test(this.email)
                let phone_available=this.phone_regular.test(this.phone)
                if(email_available&&phone_available){
                    let data={
                        "id":this.username,
                        "sponsor_name":this.name,
                        "head_portrait":this.head_portrait,
                        "email":this.email,
                        "phone_number":this.phone
                    }
                    this.$axios.put("/sponsors/view", data).then(response => {
                        if(response.status===200) {
                            this.$message({
                                message: '修改成功',
                                type: 'success'
                            })
                            this.$router.push('/EventList')
                        }
                        else{
                            this.$message({
                                message: '修改失败',
                                type: 'error'
                            })
                        }
                    })
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
            },
            testLogIn(){
                if(!this.$store.state.username){
                    this.$message({
                        message: '请登录',
                        type: 'error'
                    })
                    this.$router.push('/')
                }
            },
            pageReturn(){
                this.$router.push('/EventList')
            },
            beforeUploadPicture(file) {
                if(file.size > 10*1024*1024){
                    this.$message.error("上传图片不能大于10M");
                    return false;
                }
            },

            // 上传图片成功
            uploadSuccess(res, file, fileList) {
                file.url=file.response.file_url
                this.head_portrait=file.url
            },
            // 上传图片出错
            uploadError(err, file, fileList) {
                this.$message.error("上传出错");
            },
            // 移除图片
            handleRemove(file, fileList) {
                this.head_portrait=''
            },

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
    text-align: center;
  }
  .info-text {
    font-weight: 400;
    font-size: 18px;
  }
  .compulsory{
    color:#ff0000
  }
</style>
