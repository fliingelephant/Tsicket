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
        <el-table
                :data="events"
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
      </el-tab-pane>
      <el-tab-pane label="审核新活动" name="second">
        <el-table
                :data="new_events"
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
                activeName: 'second',
                events:[],
                new_events:[]
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