<template>
  <el-container class="container">

    <el-header class="header">
      <el-row>
        <el-col :span="16"><div>活动详情</div></el-col>
        <el-col :span="4"><div class="add-event"><el-button @click="addNotice" disabled>发布通知</el-button></div></el-col>
        <el-col :span="4"><div class="add-event"><el-button @click="addMoment">添加动态</el-button></div></el-col>
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
                <el-col :span="12"><div class="event-text">{{info.event_status}}</div></el-col>
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

        <el-tab-pane label="动态列表" name="second">

          <el-table
                  stripe
                  :data="moment_data">

            <el-table-column
                    prop="text"
                    label="动态内容"
                    min-width="40%">
            </el-table-column>

            <el-table-column
                    prop="pictures"
                    label="图片"
                    min-width="40%">
              <template   slot-scope="scope">
                <img v-for="picture in scope.row.pictures" :src="picture" class="image" min-width="70" height="70" />
              </template>
            </el-table-column>

            <el-table-column
                    prop="time"
                    label="发布时间"
                    min-width="20%">
            </el-table-column>

          </el-table>


        </el-tab-pane>

        <el-tab-pane label="通知列表" name="third">

          <el-table
                  stripe
                  :data="notice_data">

            <el-table-column
                    prop="moment_content"
                    label="通知内容">
            </el-table-column>

            <el-table-column
                    prop="moment_time"
                    label="发布时间">
            </el-table-column>

          </el-table>


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
        name: "EventInfo",
        data() {
            return {
                info:'',
                notice_data:[],
                moment_data:[],
                active_name:'first'
            };
        },
        mounted(){
            this.getInfo()
            //this.getNotice()
            this.getMoment()
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
            getNotice(){
                this.$axios.get("/events/posts/"+this.$route.params.id).then(response => {
                    if(response.status===200) {
                        this.notice_data=response.data
                        console.log(this.notice_data)
                    }
                    else{
                        this.$message({
                            message: '查询通知失败',
                            type: 'error'
                        })
                    }
                },err=>{
                    this.$message({
                        message: '查询通知失败',
                        type: 'error'
                    })
                })

            },
            getMoment(){
                this.$axios.get("/events/moments/"+this.$route.params.id).then(response => {
                    if(response.status===200) {
                        this.moment_data=response.data.moments
                    }
                    else{
                        this.$message({
                            message: '查询动态失败',
                            type: 'error'
                        })
                    }
                },err=>{
                    this.$message({
                        message: '查询动态失败',
                        type: 'error'
                    })
                })

            },
            addNotice(){
                this.$router.push('/AddNotice/'+this.$route.params.id)
            },
            addMoment(){
                this.$router.push('/AddMoment/'+this.$route.params.id)
            },
            pageReturn(){
                this.$router.push('/EventList')
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
