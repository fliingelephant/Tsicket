<template>
  <el-container class="container">

    <el-header class="header">
      <div>欢迎来到帮助界面</div>
    </el-header>

    <el-main class="help-text">
      <div>第一次使用本系统进行活动信息发布，请先注册。</div>
      <div>注册并通过审核后可以进入活动界面发布活动信息。</div>
      <div>发布活动信息需要填写必填项，并通过审核</div>
      <div>以往发布的活动信息可以从活动界面中查看。</div>

      <el-row>
        <el-col :span="3">
          <el-button @click="pageReturn">返回</el-button>
        </el-col>
      </el-row>

    </el-main>

  </el-container>
</template>

<script>
    export default {
        name: "Help",
        methods:{
            pageReturn(){
                this.$router.push('/EventList')
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
    text-align: center;
    border-bottom: 1px solid silver;
  }
  .help-text {
    font-weight: 400;
    font-size: 16px;
  }
</style>
