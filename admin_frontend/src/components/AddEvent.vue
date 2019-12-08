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
                      value-format="yyyy-MM-dd HH:mm:ss"
                      v-model="distributestart"
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
                      value-format="yyyy-MM-dd HH:mm:ss"
                      v-model="distributeend"
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
                      value-format="yyyy-MM-dd HH:mm:ss"
                      v-model="date"
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
              <el-radio v-model="method" label='1'>扫码领票</el-radio>
              <el-radio v-model="method" label='2'>申请审核</el-radio>
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
            <el-col :span="4"><div class="event-text">活动图片</div></el-col>
            <el-col :span="20">
              <el-upload
                      :action="upload_url+new Date().toISOString()"
                      :limit="1"
                      accept="image/png,image/jpeg"
                      list-type="picture-card"
                      :before-upload="beforeUploadPicture"
                      :on-preview="handlePictureCardPreview"
                      :on-progress="uploadProgress"
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
        <el-col :span="3">
          <el-button @click="post" type="primary">发布</el-button>
        </el-col>
        <el-col :span="3">
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
                date: '',
                method: '',
                description:'',
                upload_url:'apis/sponsors/pic/',
                event_picture:'',
                distributestart:'',
                distributeend:'',
                capacity:'',
            }
        },
        methods:{
            post(){
                let data={
                    "event_name":this.name,
                    "start_time":this.distributestart,
                    "end_time":this.distributeend,
                    "event_time":this.date,
                    "event_type":parseInt(this.method),
                    "event_introduction":this.description,
                    "event_picture":this.event_picture,
                    "event_capacity":this.capacity,
                    "left_tickets": this.capacity,
                    "event_location":this.place,
                }
                this.$axios.post("/sponsors",data).then(response => {
                    if(response.status===200) {
                        this.$router.push('/EventList')
                    }
                    else{
                        this.$message({
                            message: '提交失败',
                            type: 'error'
                        })
                    }
                },err=>{
                    this.$message({
                        message: '提交失败',
                        type: 'error'
                    })
                })
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
            // 上传图片时调用
            uploadProgress(event,file, fileList){
            },
            // 上传图片成功
            uploadSuccess(res, file, fileList) {
                file.url=file.response.file_url
                this.event_picture=file.url
            },
            // 上传图片出错
            uploadError(err, file, fileList) {
                this.$message.error("上传出错");
            },
            // 移除图片
            handleRemove(file, fileList) {
                this.event_picture=''
            },
            handlePictureCardPreview(file) {
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
