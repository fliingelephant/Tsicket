<template>
  <el-container class="container">

    <el-header class="header">
      <div>管理员菜单</div>
    </el-header>

    <el-main>
    <el-tabs v-model="activeName">
      <el-tab-pane label="活动管理" name="first">
        <el-table
                :data="events.slice((currentPage-1)*pageSize,currentPage*pageSize)"
                style="width: 100%">

          <el-table-column
                  prop="event_name"
                  label="活动名称">
          </el-table-column>

          <el-table-column
                  prop="event_time"
                  label="活动开始">
          </el-table-column>

          <el-table-column
                  prop="event_capacity"
                  label="活动容量">
          </el-table-column>

          <el-table-column
                  prop="event_string"
                  label="活动状态">
          </el-table-column>

          <el-table-column
                  label="操作"
                  min-width="120%"
          >
            <template slot-scope="scope">
              <el-button @click="eventInfo(scope.row)" type="text" size="small">查看</el-button>
              <el-button v-if="pass_state.indexOf(scope.row.event_status)!==-1" @click="passExamine(scope.row)" type="text" size="small">通过审核</el-button>
              <el-button v-if="post_state.indexOf(scope.row.event_status)!==-1" type="text" size="small" @click="passAdvertise(scope.row)">通过推广申请</el-button>
              <el-button v-if="drop_state.indexOf(scope.row.event_status)!==-1" type="text" size="small" @click="dropAdvertise(scope.row)">中止推广</el-button>
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
                events:[],
                states: {
                    0 : "待审核",
                    1 : "抢票未开始",
                    2 : "抢票开始",
                    3 : "抢票结束",
                    4 : "活动取消",
                    10 : "非法状态",
                    11 : "抢票未开始（推广申请中）",
                    12 : "抢票开始（推广申请中）",
                    13 : "抢票结束",
                    14 : "活动取消",
                    20 : "非法状态",
                    21 : "抢票未开始（推广中）",
                    22 : "抢票开始（推广中）",
                    23 : "抢票结束",
                    24 : "活动取消"
                },
                pass_state:[0,10,20],
                post_state:[11,12],
                drop_state:[21,22],
                activeName: 'first',
                currentPage:1,
                pageSize:10
            };
        },
        mounted(){
            this.getEvents()
        },
        methods: {
            getEvents(){
                this.$axios.get("/admins").then(response => {
                    if (response.status === 200) {
                        this.events=response.data.events
                        for(let i=0 ; i< this.events.length; i++){
                            this.events[i].event_string = this.states[this.events[i].event_status]
                        }
                    } else {
                        this.$message({
                            message: '获取活动失败',
                            type: 'error'
                        })
                    }
                },err=>{
                    this.$message({
                        message: '获取活动失败',
                        type: 'error'
                    })
                })
            },
            handleSizeChange: function(val) {
                this.pageSize = val;
            },
            handleCurrentChange: function(page) {
                this.currentPage = page;
            },
            eventInfo(row){
                this.$router.push('/AdminEvent/'+row.event_id)
            },
            passExamine(row){
                this.$axios.post("/admins/review/"+row.event_id).then(response => {
                    if (response.status === 200) {
                        this.$router.go(0)

                    } else {
                        this.$message({
                            message: '处理失败',
                            type: 'error'
                        })
                    }
                },err=>{
                    this.$message({
                        message: '处理失败',
                        type: 'error'
                    })
                })
            },
            passAdvertise(row){
                this.$axios.post("/admins/advertise/"+row.event_id).then(response => {
                    if (response.status === 200) {
                        this.$router.go(0)

                    } else {
                        this.$message({
                            message: '处理失败',
                            type: 'error'
                        })
                    }
                },err=>{
                    this.$message({
                        message: '处理失败',
                        type: 'error'
                    })
                })
            },
            dropAdvertise(row){
                this.$axios.delete("/admins/advertise/"+row.event_id).then(response => {
                    if (response.status === 200) {
                        this.$router.go(0)

                    } else {
                        this.$message({
                            message: '处理失败',
                            type: 'error'
                        })
                    }
                },err=>{
                    this.$message({
                        message: '处理失败',
                        type: 'error'
                    })
                })
            },
        }
    };
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
    display: flex;
    text-align: center;
    border-top: none;
    border-left: none;
    border-right: none;
  }
</style>
