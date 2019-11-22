<template>
  <el-container class="container">
    <el-header class="header">
      <el-row>
        <el-col :span="20"><div>活动列表</div></el-col>
        <el-col :span="4"><div class="add-event"><el-button @click="addEvent">添加活动</el-button></div></el-col>
      </el-row>
    </el-header>
    <el-main>
      <el-table
              stripe
              :data="table_data.slice((currentPage-1)*pageSize,currentPage*pageSize)">
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
        <el-table-column
                label="操作">
          <el-button @click="eventInfo" type="text" size="small">查看</el-button>
          <el-button @click="changeEvent" type="text" size="small">编辑</el-button>
        </el-table-column>
      </el-table>
      <el-pagination
      @size-change="handleSizeChange"
      @current-change="handleCurrentChange"
      :current-page="currentPage"
      :page-sizes="[10, 20]"
      :page-size="pageSize"
      :total="table_data.length"
      layout="total, sizes, prev, pager, next, jumper">
      </el-pagination>
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
                table_data: [{
                    event_name:"测试_活动名称",
                    start_time:"2019.6.10 12:00:00",
                    end_time:"2019.6.10 12:00:00",
                    event_capacity:20,
                    left_ticket:10
                },{
                    event_name:"测试_活动名称",
                    start_time:"2019.6.10 12:00:00",
                    end_time:"2019.6.10 12:00:00",
                    event_capacity:20,
                    left_ticket:10
                },{
                    event_name:"测试_活动名称",
                    start_time:"2019.6.10 12:00:00",
                    end_time:"2019.6.10 12:00:00",
                    event_capacity:20,
                    left_ticket:10
                },{
                    event_name:"测试_活动名称",
                    start_time:"2019.6.10 12:00:00",
                    end_time:"2019.6.10 12:00:00",
                    event_capacity:20,
                    left_ticket:10
                },{
                    event_name:"测试_活动名称",
                    start_time:"2019.6.10 12:00:00",
                    end_time:"2019.6.10 12:00:00",
                    event_capacity:20,
                    left_ticket:10
                },{
                    event_name:"测试_活动名称",
                    start_time:"2019.6.10 12:00:00",
                    end_time:"2019.6.10 12:00:00",
                    event_capacity:20,
                    left_ticket:10
                },{
                    event_name:"测试_活动名称",
                    start_time:"2019.6.10 12:00:00",
                    end_time:"2019.6.10 12:00:00",
                    event_capacity:20,
                    left_ticket:10
                },{
                    event_name:"测试_活动名称",
                    start_time:"2019.6.10 12:00:00",
                    end_time:"2019.6.10 12:00:00",
                    event_capacity:20,
                    left_ticket:10
                },{
                    event_name:"测试_活动名称",
                    start_time:"2019.6.10 12:00:00",
                    end_time:"2019.6.10 12:00:00",
                    event_capacity:20,
                    left_ticket:10
                },{
                    event_name:"测试_活动名称",
                    start_time:"2019.6.10 12:00:00",
                    end_time:"2019.6.10 12:00:00",
                    event_capacity:20,
                    left_ticket:10
                },{
                    event_name:"测试_活动名称",
                    start_time:"2019.6.10 12:00:00",
                    end_time:"2019.6.10 12:00:00",
                    event_capacity:20,
                    left_ticket:10
                },{
                    event_name:"测试_活动名称",
                    start_time:"2019.6.10 12:00:00",
                    end_time:"2019.6.10 12:00:00",
                    event_capacity:20,
                    left_ticket:10
                },{
                    event_name:"测试_活动名称",
                    start_time:"2019.6.10 12:00:00",
                    end_time:"2019.6.10 12:00:00",
                    event_capacity:20,
                    left_ticket:10
                },{
                    event_name:"测试_活动名称",
                    start_time:"2019.6.10 12:00:00",
                    end_time:"2019.6.10 12:00:00",
                    event_capacity:20,
                    left_ticket:10
                },{
                    event_name:"测试_活动名称",
                    start_time:"2019.6.10 12:00:00",
                    end_time:"2019.6.10 12:00:00",
                    event_capacity:20,
                    left_ticket:10
                },{
                    event_name:"测试_活动名称",
                    start_time:"2019.6.10 12:00:00",
                    end_time:"2019.6.10 12:00:00",
                    event_capacity:20,
                    left_ticket:10
                },{
                    event_name:"测试_活动名称",
                    start_time:"2019.6.10 12:00:00",
                    end_time:"2019.6.10 12:00:00",
                    event_capacity:20,
                    left_ticket:10
                },{
                    event_name:"测试_活动名称",
                    start_time:"2019.6.10 12:00:00",
                    end_time:"2019.6.10 12:00:00",
                    event_capacity:20,
                    left_ticket:10
                },{
                    event_name:"测试_活动名称",
                    start_time:"2019.6.10 12:00:00",
                    end_time:"2019.6.10 12:00:00",
                    event_capacity:20,
                    left_ticket:10
                },{
                    event_name:"测试_活动名称",
                    start_time:"2019.6.10 12:00:00",
                    end_time:"2019.6.10 12:00:00",
                    event_capacity:20,
                    left_ticket:10
                },{
                    event_name:"测试_活动名称",
                    start_time:"2019.6.10 12:00:00",
                    end_time:"2019.6.10 12:00:00",
                    event_capacity:20,
                    left_ticket:10
                },{
                    event_name:"测试_活动名称",
                    start_time:"2019.6.10 12:00:00",
                    end_time:"2019.6.10 12:00:00",
                    event_capacity:20,
                    left_ticket:10
                },{
                    event_name:"测试_活动名称",
                    start_time:"2019.6.10 12:00:00",
                    end_time:"2019.6.10 12:00:00",
                    event_capacity:20,
                    left_ticket:10
                },{
                    event_name:"测试_活动名称",
                    start_time:"2019.6.10 12:00:00",
                    end_time:"2019.6.10 12:00:00",
                    event_capacity:20,
                    left_ticket:10
                },{
                    event_name:"测试_活动名称",
                    start_time:"2019.6.10 12:00:00",
                    end_time:"2019.6.10 12:00:00",
                    event_capacity:20,
                    left_ticket:10
                },{
                    event_name:"测试_活动名称",
                    start_time:"2019.6.10 12:00:00",
                    end_time:"2019.6.10 12:00:00",
                    event_capacity:20,
                    left_ticket:10
                },{
                    event_name:"测试_活动名称",
                    start_time:"2019.6.10 12:00:00",
                    end_time:"2019.6.10 12:00:00",
                    event_capacity:20,
                    left_ticket:10
                },{
                    event_name:"测试_活动名称",
                    start_time:"2019.6.10 12:00:00",
                    end_time:"2019.6.10 12:00:00",
                    event_capacity:20,
                    left_ticket:10
                },{
                    event_name:"测试_活动名称",
                    start_time:"2019.6.10 12:00:00",
                    end_time:"2019.6.10 12:00:00",
                    event_capacity:20,
                    left_ticket:10
                },{
                    event_name:"测试_活动名称",
                    start_time:"2019.6.10 12:00:00",
                    end_time:"2019.6.10 12:00:00",
                    event_capacity:20,
                    left_ticket:10
                },{
                    event_name:"测试_活动名称",
                    start_time:"2019.6.10 12:00:00",
                    end_time:"2019.6.10 12:00:00",
                    event_capacity:20,
                    left_ticket:10
                },{
                    event_name:"测试_活动名称",
                    start_time:"2019.6.10 12:00:00",
                    end_time:"2019.6.10 12:00:00",
                    event_capacity:20,
                    left_ticket:10
                },{
                    event_name:"测试_活动名称",
                    start_time:"2019.6.10 12:00:00",
                    end_time:"2019.6.10 12:00:00",
                    event_capacity:20,
                    left_ticket:10
                },{
                    event_name:"测试_活动名称",
                    start_time:"2019.6.10 12:00:00",
                    end_time:"2019.6.10 12:00:00",
                    event_capacity:20,
                    left_ticket:10
                },{
                    event_name:"测试_活动名称",
                    start_time:"2019.6.10 12:00:00",
                    end_time:"2019.6.10 12:00:00",
                    event_capacity:20,
                    left_ticket:10
                },{
                    event_name:"测试_活动名称",
                    start_time:"2019.6.10 12:00:00",
                    end_time:"2019.6.10 12:00:00",
                    event_capacity:20,
                    left_ticket:10
                }],
                currentPage:1,
                pageSize:10
            })
        },
        methods:{
            addEvent(){
                this.$router.push('/AddEvent')
            },
            eventInfo(){
                this.$router.push('/EventInfo')
            },
            changeEvent(){
                this.$router.push('/ChangeEvent')
            },
            help(){
                this.$router.push('/Help')
            },
            handleSizeChange: function(val) {
                this.pageSize = val;
            },
            handleCurrentChange: function(page) {
                this.currentPage = page;
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
  .add-event{
    text-align: right;
  }

</style>