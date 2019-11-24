<template>
  <el-container class="container">
    <el-header class="header">
      <el-row>
        <el-col :span="20"><div>活动列表</div></el-col>
        <el-col :span="4"><div class="add-event"><el-button>管理人员</el-button></div></el-col>
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
                <el-col :span="12"><div class="event-text">{{event_start}}</div></el-col>

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
                <el-col :span="5"><div class="event-text"><a class="compulsory">*</a>活动类别</div></el-col>
                <el-col :span="12"><div class="event-text">{{event_type}}</div></el-col>
              </el-row>
            </el-form-item>

            <el-form-item>
              <el-row :gutter="20">
                <el-col :span="5"><div class="event-text"><a class="compulsory">*</a>活动简介</div></el-col>
                <el-col :span="12"><div class="event-text">{{info.event_introduction}}</div></el-col>
              </el-row>
            </el-form-item>
          </el-form>
        </el-tab-pane>
      </el-tabs>
    </el-main>
  </el-container>
</template>

<script>
    export default {
        name: "EventInfo",
        data() {
            return {
                info:'',
                active_name: 'first',
                event_name: '测试——活动名称',
                event_place: '测试——活动地点',
                event_start: '2019.6.18 12:00:00',
                distribute:'测试——抢票',
                event_type:'讲座',
                event_intro:'测试——本活动是一场讲座，会邀请xxxyyy来到zzzzzzzzz，为大家带来xxxxx。',
                start_time:"2019.6.10 12:00:00",
                end_time:"2019.6.10 12:00:00",
                event_capacity:20,
                left_ticket:10,
            };
        },
        mounted(){
            this.getInfo()
        },
        methods: {
            getInfo(){
                let data={
                    "id":this.$route.params.id
                }

                this.$axios.post("/events/view",data).then(response => {
                    if(response.status===200) {
                        this.info=response.data
                        console.log(this.info)
                    }
                    else{
                        this.$message({
                            message: '查询失败',
                            type: 'error'
                        })
                    }
                },err=>{

                    this.$message({
                        message: '查询失败',
                        type: 'error'
                    })
                })



            },



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
</style>
