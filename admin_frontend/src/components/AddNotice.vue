<template>
  <el-container class="container">

    <el-header class="header">
      <div>发布通知</div>
    </el-header>
    <el-main>
      <el-form>
        <el-form-item>
          <el-row :gutter="20">
            <el-col :span="4"><div class="event-text"><a class="compulsory">*</a>通知内容</div></el-col>
            <el-col :span="12"><el-input
                    type="textarea"
                    v-model="script"
                    :autosize="{minRows:5,maxRows:20}"
                    auto-complete="off"
            ></el-input>
            </el-col>
          </el-row>
        </el-form-item>
      </el-form>

      <el-row gutter="20">
        <el-col :span="3">
          <el-button type="primary">发布</el-button>
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
        name: "AddNotice",
        data() {
            return({
                script:'这是一个通知',
            })
        },
        methods:{
            pageReturn() {
                this.$router.push('/EventInfo/'+this.$route.params.id)
            },
            publishNotice(){
                let data = {
                    "content" : "活动时间更改为：21839090"
                }
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
