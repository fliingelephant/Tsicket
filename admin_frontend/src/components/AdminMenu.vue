<template>
  <el-container class="login-container">
    <el-header class="login-header">
      <div>
        管理员菜单
      </div>
    </el-header>
    <el-main>
    <el-tabs v-model="activeName">
      <el-tab-pane label="活动管理" name="first">

      </el-tab-pane>
      <el-tab-pane label="审核新活动" name="second">

      </el-tab-pane>
    </el-tabs>
    </el-main>
  </el-container>
</template>

<script>
    export default {
        name: "AdminMenu",
        data() {
            return {
                activeName: 'second'
            };
        },
        methods: {

        }
    };
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
    border-top: none;
    border-left: none;
    border-right: none;

  }
  .login-main {
    font-weight: 400;
    font-size: 18px;
  }
  .compulsory{
    color:#ff0000
  }
</style>