<template>
  <el-container class="container">

    <el-header class="header">
      <div>活动发布</div>
    </el-header>

    <el-main>
      <el-form>

        <el-form-item>
          <el-row :gutter="20">
            <el-col :span="5"><div class="event-text"><a class="compulsory">*</a>活动名称</div></el-col>
            <el-col :span="12"><el-input
                    type="text"
                    v-model="name"
                    auto-complete="off"
            ></el-input></el-col>
          </el-row>
        </el-form-item>

        <el-form-item>
          <el-row :gutter="20">
            <el-col :span="5"><div class="event-text"><a class="compulsory">*</a>活动地点</div></el-col>
            <el-col :span="12"><el-input
                    type="text"
                    v-model="place"
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
                      v-model="distribute_start"
                      type="datetime"
              ></el-date-picker>
            </el-col>
          </el-row>
        </el-form-item>

        <el-form-item>
          <el-row :gutter="20">
            <el-col :span="5"><div class="event-text"><a class="compulsory">*</a>发票结束时间</div></el-col>
            <el-col :span="12">
              <el-date-picker
                      value-format="yyyy/MM/dd HH:mm:ss"
                      v-model="distribute_end"
                      type="datetime"
              ></el-date-picker>
            </el-col>
          </el-row>
        </el-form-item>

        <el-form-item>
          <el-row :gutter="20">
            <el-col :span="5"><div class="event-text"><a class="compulsory">*</a>活动容量</div></el-col>
            <el-col :span="12"><el-input-number
                    v-model="capacity"
            ></el-input-number></el-col>
          </el-row>
        </el-form-item>

        <el-form-item>
          <el-row :gutter="20">
            <el-col :span="5"><div class="event-text"><a class="compulsory">*</a>活动开始时间</div></el-col>
            <el-col :span="12">
              <el-date-picker
                      value-format="yyyy/MM/dd HH:mm:ss"
                      v-model="event_time"
                      type="datetime"
              ></el-date-picker>
            </el-col>
          </el-row>
        </el-form-item>

        <el-form-item>
          <el-row :gutter="20">
            <el-col :span="5"><div class="event-text"><a class="compulsory">*</a>活动发票方式</div></el-col>
            <el-col :span="12">
              <el-radio v-model="method" label='0'>抢票</el-radio>
            </el-col>
          </el-row>
        </el-form-item>

        <el-form-item>
          <el-row :gutter="20">
            <el-col :span="5"><div class="event-text"><a class="compulsory">*</a>活动简介</div></el-col>
            <el-col :span="12"><el-input
                    type="textarea"
                    v-model="description"
                    :autosize="{minRows:5,maxRows:20}"
                    auto-complete="off"
            ></el-input></el-col>
          </el-row>
        </el-form-item>

        <el-form-item>
          <el-row :gutter="20">
            <el-col :span="5"><div class="event-text">活动图片</div></el-col>
            <el-col :span="12">
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

      <el-row :gutter="20">
        <el-col :span="10">
          <el-button @click="post" type="primary">发布</el-button>
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
                name: '',
                place: '',
                event_time: '',
                method: '',
                description:'',
                upload_url:'apis/sponsors/pic/',
                event_picture:'',
                distribute_start:'',
                distribute_end:'',
                capacity:'',
            }
        },
        methods:{
            post(){

                if(this.name === '' || this.place === ''|| this.event_time === ''||this.method === ''||this.description === ''||this.distribute_start === ''||this.distribute_end === ''||this.capacity === ''){
                    this.$message.error('请输入必填项')
                }
                else {
                    let time_start=Date.parse(this.distribute_start)
                    let time_end=Date.parse(this.distribute_end)
                    let time_event=Date.parse(this.event_time)
                    if(time_start<time_end&&time_end<time_event) {
                        let data = {
                            "event_name": this.name,
                            "start_time": this.distribute_start,
                            "end_time": this.distribute_end,
                            "event_time": this.event_time,
                            "event_type": parseInt(this.method),
                            "event_introduction": this.description,
                            "event_picture": this.event_picture,
                            "event_capacity": this.capacity,
                            "left_tickets": this.capacity,
                            "event_location": this.place,
                        }
                        this.$axios.post("/sponsors", data).then(response => {
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
                    else
                    {
                        this.$message.error('开始时间应早于结束时间，结束时间应早于活动时间。')
                    }
                }
            },
            pageReturn(){
                this.$router.push('/EventList')
            },
            beforeUploadPicture(file) {
                if(file.size > 10*1024*1024){
                    this.$message.error("上传图片不能大于10M")
                    return false;
                }
            },

            uploadSuccess(res, file, fileList) {
                file.url=file.response.file_url
                this.event_picture=file.url
            },

            uploadError(err, file, fileList) {
                this.$message.error("上传出错")
            },

            handleRemove(file, fileList) {
                this.event_picture=''
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
