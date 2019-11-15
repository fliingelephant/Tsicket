<template>
  <el-container class="login-container">
    <el-header class="login-header">
      <el-row>
        <el-col :span="20"><div>活动列表</div></el-col>
        <el-col :span="4"><div class="add-event"><el-button>管理人员</el-button></div></el-col>
      </el-row>
    </el-header>
    <el-main>
      <el-tabs v-model="activeName">
        <el-tab-pane label="活动详情" name="first">
          <el-form
                  ref="usernameRegister"
                  label-suffix="left"
                  label-width="0px"
          >
            <el-form-item prop="name">
              <el-row :gutter="20">
                <el-col :span="5"><div align="left" class="login-main"><a class="compulsory">*</a>活动名称</div></el-col>
                <el-col :span="12"><div class="login-main">{{event_name}}</div></el-col>
              </el-row>
            </el-form-item>
            <el-form-item prop="place">
              <el-row :gutter="20">
                <el-col :span="5"><div align="left" class="login-main"><a class="compulsory">*</a>活动地点</div></el-col>
                <el-col :span="12"><div class="login-main">{{event_place}}</div></el-col>
              </el-row>
            </el-form-item>
            <el-form-item prop="distributestart">
              <el-row :gutter="20">
                <el-col :span="5"><div align="left" class="login-main"><a class="compulsory">*</a>发票开始时间</div></el-col>
                <el-col :span="12"><div class="login-main">{{start_time}}</div></el-col>

              </el-row>
            </el-form-item>
            <el-form-item prop="distributeend">
              <el-row :gutter="20">
                <el-col :span="5"><div align="left" class="login-main"><a class="compulsory">*</a>发票停止时间</div></el-col>
                <el-col :span="12"><div class="login-main">{{end_time}}</div></el-col>

              </el-row>
            </el-form-item>

            <el-form-item prop="capacity">
              <el-row :gutter="20">
                <el-col :span="5"><div align="left" class="login-main"><a class="compulsory">*</a>活动容量</div></el-col>
                <el-col :span="12"><div class="login-main">{{event_capacity}}</div></el-col>
              </el-row>
            </el-form-item>

            <el-form-item prop="datetime">
              <el-row :gutter="20">
                <el-col :span="5"><div align="left" class="login-main"><a class="compulsory">*</a>活动开始时间</div></el-col>
                <el-col :span="12"><div class="login-main">{{event_start}}</div></el-col>

              </el-row>
            </el-form-item>
            <el-form-item prop="method">
              <el-row :gutter="20">
                <el-col :span="5"><div align="left" class="login-main"><a class="compulsory">*</a>活动发票方式</div></el-col>
                <el-col :span="12"><div class="login-main">{{distribute}}</div></el-col>
              </el-row>
            </el-form-item>
            <el-form-item prop="type">
              <el-row :gutter="20">
                <el-col :span="5"><div align="left" class="login-main"><a class="compulsory">*</a>活动类别</div></el-col>
                <el-col :span="12"><div class="login-main">{{event_type}}</div></el-col>
              </el-row>
            </el-form-item>

            <el-form-item prop="description">
              <el-row :gutter="20">
                <el-col :span="5"><div align="left" class="login-main"><a class="compulsory">*</a>活动简介</div></el-col>
                <el-col :span="12"><div class="login-main">{{event_intro}}</div></el-col>
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
                activeName: 'first',
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
                events:[],
                new_events:[]
            };
        },
        methods: {

        }
    }
</script>

<style scoped>
  .login-container{

    margin: 10px;
    padding: 5px;
    font-weight: 600;
    font-size: 25px;
    text-align: left;
  }

  .login-header {
    text-align: left;
    border-top: none;
    border-left: none;
    border-right: none;

  }

  .add-event{
    text-align: right;
  }
  .login-main {
    font-weight: 400;
    font-size: 18px;
  }
  .compulsory{
    color:#ff0000
  }
</style>