<template>
  <el-container class="container">

    <el-header class="header">
      <div>欢迎来到帮助界面</div>
    </el-header>

    <el-main class="help-text">
      <h3>活动列表</h3>
      <p>登录后，进入活动列表页面，该列表中可以查询到您发布过的活动的基本信息,包括活动名，活动状态，剩余票数，时间等。</p>
      <p>点击查看可以进入活动详情界面，查看更多活动信息。点击编辑可以修改一些活动信息。</p>
      <p>根据当前的活动状态，在活动对应的操作栏可以进行相应的操作。</p>
      <h3>活动详情</h3>
      <p>在活动详情界面中除了可以查看活动的详细信息，还可以发布该活动的动态，供活动参与者查看。</p>
      <h3>活动动态</h3>
      <p>活动动态支持文字和图片，图片最多发布九张。</p>



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
