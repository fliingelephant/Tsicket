<template>
  <el-container class="container">

    <el-header class="header">
      <el-row>
        <el-col :span="16"><div>活动详情</div></el-col>
      </el-row>
    </el-header>

    <el-main>
      <el-tabs v-model="active_name">
        <el-tab-pane label="活动详情" name="first">
          <el-form>

            <el-form-item>
              <el-row :gutter="20">
                <el-col :span="5"><div class="event-text"><a class="compulsory">*</a>活动名称</div></el-col>
                <el-col :span="12"><div class="event-text">{{info.event_name}}</div></el-col>
              </el-row>
            </el-form-item>

            <el-form-item>
              <el-row :gutter="20">
                <el-col :span="5"><div class="event-text"><a class="compulsory">*</a>活动状态</div></el-col>
                <el-col :span="12"><div class="event-text">{{states[info.event_status]}}</div></el-col>
              </el-row>
            </el-form-item>

            <el-form-item>
              <el-row :gutter="20">
                <el-col :span="5"><div class="event-text"><a class="compulsory">*</a>活动地点</div></el-col>
                <el-col :span="12"><div class="event-text">{{info.event_location}}</div></el-col>
              </el-row>
            </el-form-item>

            <el-form-item>
              <el-row :gutter="20">
                <el-col :span="5"><div class="event-text"><a class="compulsory">*</a>发票开始时间</div></el-col>
                <el-col :span="12"><div class="event-text">{{info.start_time}}</div></el-col>
              </el-row>
            </el-form-item>

            <el-form-item>
              <el-row :gutter="20">
                <el-col :span="5"><div class="event-text"><a class="compulsory">*</a>发票停止时间</div></el-col>
                <el-col :span="12"><div class="event-text">{{info.end_time}}</div></el-col>
              </el-row>
            </el-form-item>

            <el-form-item>
              <el-row :gutter="20">
                <el-col :span="5"><div class="event-text"><a class="compulsory">*</a>活动容量</div></el-col>
                <el-col :span="12"><div class="event-text">{{info.event_capacity}}</div></el-col>
              </el-row>
            </el-form-item>

            <el-form-item>
              <el-row :gutter="20">
                <el-col :span="5"><div class="event-text"><a class="compulsory">*</a>活动开始时间</div></el-col>
                <el-col :span="12"><div class="event-text">{{info.event_time}}</div></el-col>
              </el-row>
            </el-form-item>

            <el-form-item>
              <el-row :gutter="20">
                <el-col :span="5"><div class="event-text"><a class="compulsory">*</a>活动发票方式</div></el-col>
                <el-col :span="12"><div class="event-text">{{info.event_type}}</div></el-col>
              </el-row>
            </el-form-item>

            <el-form-item>
              <el-row :gutter="20">
                <el-col :span="5"><div class="event-text"><a class="compulsory">*</a>活动简介</div></el-col>
                <el-col :span="12"><div class="event-text">{{info.event_introduction}}</div></el-col>
              </el-row>
            </el-form-item>

            <el-form-item v-if="info.event_picture">
              <el-row :gutter="20">
                <el-col :span="4"><div class="event-text">活动图片</div></el-col>
                <el-col :span="16">
                  <img :src="info.event_picture" height="400">
                </el-col>

              </el-row>
            </el-form-item>

          </el-form>
        </el-tab-pane>

      </el-tabs>
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
        name: "AdminEvent",
        data() {
            return {
                info:'',
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
                notice_data:[],
                moment_data:[],
                active_name: 'first',

            };
        },
        mounted(){
            this.getInfo()
        },
        methods: {
            getInfo(){
                let data={
                    "event_id":this.$route.params.id
                }
                this.$axios.post("/events/view",data).then(response => {
                    if(response.status===200) {
                        this.info=response.data
                    }
                    else{
                        this.$message({
                            message: '查询活动详情失败',
                            type: 'error'
                        })
                    }
                },err=>{
                    this.$message({
                        message: '查询活动详情失败',
                        type: 'error'
                    })
                })
            },
            pageReturn(){
                this.$router.push('/AdminMenu')
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
  .add-event{
    text-align: right;
  }
  .event-text {
    font-weight: 400;
    font-size: 18px;
  }
  .compulsory{
    color:#ff0000
  }
  .image{
    padding-left: 5px;
  }
</style>
