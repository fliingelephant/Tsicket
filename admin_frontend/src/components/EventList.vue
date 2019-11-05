<template>
  <el-container class="login-container">
    <el-header class="login-header">
      <el-row>
        <el-col :span="20"><div>活动列表</div></el-col>
        <el-col :span="4"><div class="add-event"><el-button @click="addEvent">添加活动</el-button></div></el-col>
      </el-row>
    </el-header>
    <el-main>

    </el-main>
    <el-footer>
      <div class="list-footer" @click="help">查看帮助</div>
    </el-footer>
  </el-container>
</template>

<script>
    export default {
        name: "Register",
        methods:{
            addEvent(){
                this.$router.push('/EventInfo')
            },
            help(){
                this.$router.push('/Help')
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

    text-align: left;
    border-top: none;
    border-left: none;
    border-right: none;

  }
  .list-footer{
    text-align: center;
    font-size: 14px;
    color:#038bff;
    cursor: pointer;
  }
  .login-main {
    font-weight: 400;
    font-size: 18px;
  }
  .compulsory{
    color:#ff0000
  }
  .add-event{
    text-align: right;
  }

</style>