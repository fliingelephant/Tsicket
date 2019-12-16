<template>
  <el-container class="container">

    <el-header class="header">
      <div>活动信息修改</div>
    </el-header>

    <el-main>
      <el-form>
        <el-form-item>
          <el-row :gutter="20">
            <el-col :span="5"><div class="event-text"><a class="compulsory">*</a>活动名称</div></el-col>
            <el-col :span="12"><el-input
                    type="text"
                    v-model="info.event_name"
                    auto-complete="off"
            ></el-input></el-col>
          </el-row>
        </el-form-item>

        <el-form-item>
          <el-row :gutter="20">
            <el-col :span="5"><div class="event-text"><a class="compulsory">*</a>活动地点</div></el-col>
            <el-col :span="12"><el-input
                    type="text"
                    v-model="info.event_location"
                    auto-complete="off"
            ></el-input></el-col>
          </el-row>
        </el-form-item>

        <el-form-item>
          <el-row :gutter="20">
            <el-col :span="5"><div class="event-text"><a class="compulsory">*</a>发票开始时间</div></el-col>
            <el-col :span="12">
              <el-date-picker
                      value-format="yyyy/MM/dd HH:mm:ss"
                      v-model="info.start_time"
                      type="datetime"
              ></el-date-picker>
            </el-col>
          </el-row>
        </el-form-item>

        <el-form-item>
          <el-row :gutter="20">
            <el-col :span="5"><div class="event-text"><a class="compulsory">*</a>发票停止时间</div></el-col>
            <el-col :span="12">
              <el-date-picker
                      value-format="yyyy/MM/dd HH:mm:ss"
                      v-model="info.end_time"
                      type="datetime"
              ></el-date-picker>
            </el-col>
          </el-row>
        </el-form-item>

        <el-form-item>
          <el-row :gutter="20">
            <el-col :span="5"><div class="event-text"><a class="compulsory">*</a>活动容量</div></el-col>
            <el-col :span="12"><el-input-number
                    v-model="info.event_capacity"
            ></el-input-number></el-col>
          </el-row>
        </el-form-item>

        <el-form-item>
          <el-row :gutter="20">
            <el-col :span="5"><div class="event-text"><a class="compulsory">*</a>活动开始时间</div></el-col>
            <el-col :span="12">
              <el-date-picker
                      value-format="yyyy/MM/dd HH:mm:ss"
                      v-model="info.event_time"
                      type="datetime"
              ></el-date-picker>
            </el-col>
          </el-row>
        </el-form-item>

        <el-form-item>
          <el-row :gutter="20">
            <el-col :span="5"><div class="event-text"><a class="compulsory">*</a>活动发票方式</div></el-col>
            <el-col :span="12">
              <el-radio v-model="info.event_type" label="0">抢票</el-radio>
<!--              <el-radio v-model="info.event_type" label="1">扫码领票</el-radio>-->
<!--              <el-radio v-model="info.event_type" label="2">申请审核</el-radio>-->
            </el-col>
          </el-row>
        </el-form-item>


        <el-form-item>
          <el-row :gutter="20">
            <el-col :span="5"><div class="event-text"><a class="compulsory">*</a>活动简介</div></el-col>
            <el-col :span="12"><el-input
                    type="textarea"
                    v-model="info.event_introduction"
                    :autosize="{minRows:5,maxRows:20}"
                    auto-complete="off"
            ></el-input></el-col>
          </el-row>
        </el-form-item>

        <el-form-item>
          <el-row :gutter="20">
            <el-col :span="5"><div class="event-text">活动图片</div></el-col>
            <el-col :span="15">
              <img :src="info.event_picture" height="200">
            </el-col>
            <el-col :span="4">
              <el-button v-if="!change" @click="change=true">修改</el-button>
            </el-col>
          </el-row>
        </el-form-item>

        <el-form-item v-if="change">
          <el-row :gutter="20">
            <el-col :span="4"><div class="event-text">修改图片</div></el-col>
            <el-col :span="20">
              <!--上传图片-->
              <el-upload
                      :action="upload_url+new Date().toISOString()"
                      :limit="1"
                      accept="image/png,image/jpeg"
                      list-type="picture-card"
                      :before-upload="beforeUploadPicture"
                      :on-remove="handleRemove"
                      :on-success="uploadSuccess"
                      :on-error="uploadError"
                      :show-file-list="true">
                <i class="el-icon-plus"></i>
              </el-upload>
            </el-col>
          </el-row>
        </el-form-item>



      </el-form>

      <el-row>
        <el-col :span="10">
          <el-button @click="post" type="primary">修改</el-button>
          <el-button @click="pageReturn">返回</el-button>
        </el-col>
      </el-row>
    </el-main>
  </el-container>
</template>

<script>
    export default {
        name: "Register",
        data() {
            return {
                info:{},
                change:false,
                upload_url:'apis/sponsors/pic/',
                capacity:0,
            }
        },
        mounted(){
            this.getInfo()
        },
        methods:{
            getInfo(){
                let data={
                    "event_id":this.$route.params.id
                }
                this.$axios.post("/events/view",data).then(response => {
                    if(response.status===200) {
                        this.info=response.data
                        this.info.event_type=this.info.event_type.toString()
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
            post(){
                if(this.info.event_name===''||this.info.event_introduction===''||this.info.event_location==='')
                {
                    this.$message.error('必填项不能为空')
                }
                else {
                    let time_start=Date.parse(this.info.start_time)
                    let time_end=Date.parse(this.info.end_time)
                    let time_event=Date.parse(this.info.event_time)
                    if(time_start<time_end&&time_end<time_event) {
                        let data = {
                            "event_id": this.info.event_id,
                            "event_name": this.info.event_name,
                            "start_time": this.info.start_time,
                            "end_time": this.info.end_time,
                            "event_time": this.info.event_time,
                            "event_type": parseInt(this.info.event_type),
                            "event_introduction": this.info.event_introduction,
                            "event_picture": this.info.event_picture,
                            "event_capacity": parseInt(this.info.event_capacity),
                            "left_tickets": parseInt(this.info.event_capacity),
                            "event_location": this.info.event_location,
                        }
                        this.$axios.put("/events/view", data).then(response => {
                            if (response.status === 200) {
                                this.$router.push('/EventList')
                            } else {
                                this.$message({
                                    message: '提交失败',
                                    type: 'error'
                                })
                            }
                        }, err => {

                            this.$message({
                                message: '提交失败',
                                type: 'error'
                            })
                        })
                    }
                    else{
                        this.$message.error('开始时间应早于结束时间，结束时间应早于活动时间。')
                    }
                }
            },
            pageReturn(){
                this.$router.push('/EventList')
            },
            beforeUploadPicture(file) {
                if(file.size > 10*1024*1024){
                    this.$message.error("上传图片不能大于10M");
                    return false;
                }
            },

            uploadSuccess(res, file, fileList) {
                file.url=file.response.file_url
                this.info.event_picture=file.url
            },

            uploadError(err, file, fileList) {
                this.$message.error("上传出错");
            },

            handleRemove(file, fileList) {
                this.info.event_picture=''
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
    display: flex;
    text-align: center;
    border-top: none;
    border-left: none;
    border-right: none;
  }
  .event-text {
    font-weight: 400;
    font-size: 18px;
  }
  .compulsory{
    color:#ff0000
  }
</style>
