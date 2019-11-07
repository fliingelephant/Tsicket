<template>
  <el-container class="login-container">
    <el-header class="login-header">
      <el-row>
        <el-col :span="20"><div>活动列表</div></el-col>
        <el-col :span="4"><div class="add-event"><el-button @click="addEvent">添加活动</el-button></div></el-col>
      </el-row>
    </el-header>
    <el-main>
      <el-table
          :data="tableData"
          style="width: 100%">
        <el-table-column
          prop="event_name"
          label="活动名称">
        </el-table-column>
        <el-table-column
          prop="start_time"
          label="发票开始">
        </el-table-column>
        <el-table-column
          prop="end_time"
          label="发票结束">
        </el-table-column>
        <el-table-column
          prop="event_capacity"
          label="活动容量">
        </el-table-column>
        <el-table-column
          prop="left_ticket"
          label="剩余票数">
        </el-table-column>
      </el-table>

    </el-main>
    <el-footer>
      <div class="list-footer" @click="help">查看帮助</div>
    </el-footer>
  </el-container>
</template>

<script>
    export default {
        name: "Register",
        data() {
            return({
                tableData: [],
            })
        },
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