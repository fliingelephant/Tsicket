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
              :data="events.slice((currentPage-1)*pageSize,currentPage*pageSize)">
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
          prop="left_tickets"
          label="剩余票数">
        </el-table-column>

        <el-table-column
                label="操作">
          <template slot-scope="scope">
            <el-button @click="eventInfo(scope.row)" type="text" size="small">查看</el-button>
            <el-button @click="changeEvent(scope.row)" type="text" size="small">编辑</el-button>
          </template>
        </el-table-column>

      </el-table>
      <el-pagination
      @size-change="handleSizeChange"
      @current-change="handleCurrentChange"
      :current-page="currentPage"
      :page-sizes="[10, 20]"
      :page-size="pageSize"
      :total="events.length"
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
                events: [],
                currentPage:1,
                pageSize:10
            })
        },
        mounted(){
            this.testLogIn()
            this.getEvents()
        },
        methods:{
            addEvent(){
                this.$router.push('/AddEvent')
            },
            eventInfo(row){
                this.$router.push('/EventInfo/'+row.event_id)
            },
            changeEvent(row){
                this.$router.push('/ChangeEvent/'+row.event_id)
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
            testLogIn(){
                if(!this.$store.state.username){
                    this.$message({
                        message: '请登录',
                        type: 'error'
                    })
                    this.$router.push('/')
                }
            },
            getEvents(){
                if(!!this.$store.state.username){
                    this.$axios.get("/sponsors").then(response => {
                        if(response.status===200) {
                            this.events=response.data.events
                        }
                        else{
                            this.$message({
                                message: '获取失败',
                                type: 'error'
                            })
                        }
                    },err=>{

                            this.$message({
                                message: '获取失败',
                                type: 'error'
                            })
                    })
                }

            }
        },

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
