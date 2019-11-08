<template>
  <el-container class="login-container">
    <el-header class="login-header">
      <div>
        欢迎来到帮助界面
      </div>
    </el-header>
    <el-main class="login-main">
      <div>第一次使用本系统进行活动信息发布，请先注册。</div>
      <div>注册并通过审核后可以进入活动界面发布活动信息。</div>
      <div>发布活动信息需要填写必填项，并通过审核</div>
      <div>以往发布的活动信息可以从活动界面中查看。</div>
    </el-main>
  </el-container>
</template>

<script>
    export default {
        name: "Help"
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
    text-align: center;
    border-bottom: 1px solid silver;
  }

  .login-main {
    font-weight: 400;
    font-size: 16px;
  }
</style>