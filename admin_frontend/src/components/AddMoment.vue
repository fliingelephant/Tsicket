<template>
  <el-container class="container">

    <el-header class="header">
      <div>发布动态</div>
    </el-header>
    <el-main>
      <el-form>
        <el-form-item>
          <el-row :gutter="20">
            <el-col :span="4"><div class="event-text"><a class="compulsory">*</a>动态文字</div></el-col>
            <el-col :span="12"><el-input
                    type="textarea"
                    v-model="script"
                    :autosize="{minRows:5,maxRows:20}"
                    auto-complete="off"
            ></el-input></el-col>
          </el-row>
        </el-form-item>

        <el-form-item>
          <el-row :gutter="20">
            <el-col :span="4"><div class="event-text"><a class="compulsory">*</a>动态图片</div></el-col>
            <el-col :span="20">
              <!--上传图片-->
              <el-upload
                :action="url+new Date().toISOString()"
                :limit="9"
                multiple
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

      <el-row>
        <el-col :span="3">
          <el-button type="primary" @click="post">发布</el-button>
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
        name: "AddMoment",
        data() {
            return({
                lastfilename:'',
                filelist:[],
                url:'apis/sponsors/pic/',
                script:'这是一个动态',
                picture:''
            })
        },
        methods:{
            pageReturn() {
                this.$router.push('/EventInfo/'+this.$route.params.id)
            },
            beforeUploadPicture(file) {
                if(file.size > 10*1024*1024){
                    this.$message.error("上传图片不能大于10M");
                    return false;
                }
                else{
                    return new Promise((resolve, reject) => {
                        this.$nextTick(() => {
                            this.lastfilename = file.name
                            resolve()
                        })
                    })
                }

            },
            // 上传图片时调用
            uploadProgress(event,file, fileList){
            },
            // 上传图片成功
            uploadSuccess(res, file, fileList) {
                file.url=file.response.file_url
                this.filelist=fileList
            },
            // 上传图片出错
            uploadError(err, file, fileList) {
                this.$message.error("上传出错");
            },
            // 移除图片
            handleRemove(file, fileList) {
                this.filelist=fileList
            },
            handlePictureCardPreview(file) {
                this.dialogImageUrl = file.url;
                this.imgVisible = true;
            },
            post(){
                let pics=[]
                for(let i=0;i<this.filelist.length;i++){
                    pics.push(this.filelist[i].url)
                }
                let data={
                    "text":this.script,
                    "pictures":pics
                }
                this.$axios.post("/events/moments/"+this.$route.params.id,data).then(response => {
                    if(response.status===200) {
                        this.$router.push('/EventInfo/'+this.$route.params.id)
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

            }
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
