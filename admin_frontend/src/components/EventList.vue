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
              :data="events.slice((current_page-1)*page_size,current_page*page_size)">
        <el-table-column
          prop="event_name"
          label="活动名称">
        </el-table-column>

        <el-table-column
          prop="start_time"
          label="抢票开始">
        </el-table-column>

        <el-table-column
          prop="end_time"
          label="抢票结束">
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
                prop="event_string"
                label="活动状态">
        </el-table-column>

        <el-table-column
                label="操作"
                min-width="120%"
        >
          <template slot-scope="scope">
            <el-button @click="eventInfo(scope.row)" type="text" size="small">查看</el-button>
            <el-button @click="changeEvent(scope.row)" type="text" size="small">编辑</el-button>
            <el-button v-if="stop_state.indexOf(scope.row.event_status)!==-1" type="text" size="small"  @click="stopEvent(scope.row)">结束抢票</el-button>
            <el-button v-if="cancel_state.indexOf(scope.row.event_status)!==-1" type="text" size="small" @click="cnacelEvent(scope.row)">取消活动</el-button>
            <el-button v-if="apply_state.indexOf(scope.row.event_status)!==-1" type="text" size="small" @click="postApply(scope.row)">申请推广</el-button>
            <el-button v-if="drop_state.indexOf(scope.row.event_status)!==-1" type="text" size="small" @click="dropApply(scope.row)">撤销推广申请</el-button>
          </template>
        </el-table-column>
      </el-table>

      <el-pagination
      @size-change="handleSizeChange"
      @current-change="handleCurrentChange"
      :current-page="current_page"
      :page-sizes="[10, 20]"
      :page-size="page_size"
      :total="events.length"
      layout="total, sizes, prev, pager, next, jumper">
      </el-pagination>

    </el-main>

    <el-footer>
      <div class="list-footer"><a @click="help" style="cursor: pointer">查看帮助</a></div>
    </el-footer>

  </el-container>
</template>

<script>
    export default {
        name: "Register",
        data() {
            return({
                events: [],
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
                cancel_state: [0,1,2,10,11,12,20,21,22],
                stop_state: [1,2,11,12,21,22],
                apply_state: [1,2],
                drop_state : [11,12],
                current_page:1,
                page_size:10
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
                this.page_size = val;
            },
            handleCurrentChange: function(page) {
                this.current_page = page;
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
                    this.$axios.get("/sponsors/events").then(response => {
                        if(response.status===200) {
                            this.events=response.data.events
                            for(let i=0 ; i< this.events.length; i++){
                                this.events[i].event_string = this.states[this.events[i].event_status]
                            }
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
            },
            postApply(row){
                this.$axios.post("/sponsors/advertise/"+row.event_id).then(response => {
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
            dropApply(row){
                let data={
                    "event_id": row.event_id
                }
                this.$axios.delete("/sponsors/advertise/"+row.event_id).then(response => {
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
            stopEvent(row){
                this.$confirm('提前终止抢票不可撤回，是否确认提前终止抢票？','请确认',{
                    confirmButtonText: '确定',
                    cancelButtonText: '取消',
                    type: 'warning'

                }).then(()=>{
                    this.$axios.delete("/sponsors/book/"+row.event_id).then(response => {
                        if (response.status === 200) {
                            this.$router.go(0)

                        }
                        else {
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

                }).catch(()=>{})
            },
            cnacelEvent(row) {
                this.$confirm('取消活动不可撤回，是否确认取消活动？','请确认',{
                    confirmButtonText: '确定',
                    cancelButtonText: '取消',
                    type: 'warning'

                }).then(()=>{
                    this.$axios.put("/admins/cancel/"+row.event_id).then(response => {
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

                }).catch(()=>{})

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
  }
  .add-event{
    text-align: right;
  }
</style>
