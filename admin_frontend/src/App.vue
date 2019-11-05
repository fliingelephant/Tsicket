<template>
  <div id="app">
    <el-row class = "topbar" type="flex" justify="space-between">
      <el-col :span="10" class="topbar-tittle">
        <div>清易票活动平台</div>
      </el-col>
    </el-row>
    <router-view/>
  </div>

</template>

<script>
export default {
  name: 'app',
  components: {
  }
}
</script>

<style>
  #app {
    font-family: 'Microsoft YaHei', Helvetica, Arial, sans-serif;
    text-align: center;
    margin: auto;
  }
  .topbar {
    border-bottom: solid 1px;
    width: 100%;
    border-color:#3f51b5;
    margin: auto;
    align-items: flex-end;
    margin-bottom: 20px;
  }
  .topbar-tittle {
    color:#038bff;
    font-weight: 600;
    font-size: 30px;
    text-align: left;
    margin-left: 20px;
    margin-bottom: 10px;
  }
</style>
