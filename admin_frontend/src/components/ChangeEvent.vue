<template>
  <el-container class="login-container">
    <el-header class="login-header">
      <div>
        活动信息修改
      </div>
    </el-header>
    <el-main>
      <el-form
              ref="usernameRegister"
              label-suffix="left"
              label-width="0px"
      >
        <el-form-item prop="name">
          <el-row :gutter="20">
            <el-col :span="5"><div align="left" class="login-main"><a class="compulsory">*</a>活动名称</div></el-col>
            <el-col :span="12"><el-input
                    type="text"
                    v-model="name"
                    auto-complete="off"
            ></el-input></el-col>
          </el-row>
        </el-form-item>
        <el-form-item prop="place">
          <el-row :gutter="20">
            <el-col :span="5"><div align="left" class="login-main"><a class="compulsory">*</a>活动地点</div></el-col>
            <el-col :span="12"><el-input
                    type="text"
                    v-model="place"
                    auto-complete="off"
            ></el-input></el-col>
          </el-row>
        </el-form-item>
        <el-form-item prop="distributestart">
          <el-row :gutter="20">
            <el-col :span="5"><div align="left" class="login-main"><a class="compulsory">*</a>发票开始时间</div></el-col>
            <el-col :span="12">
              <el-date-picker
                      v-model="distributestart"
                      type="datetime"
              ></el-date-picker>
            </el-col>

          </el-row>
        </el-form-item>
        <el-form-item prop="distributeend">
          <el-row :gutter="20">
            <el-col :span="5"><div align="left" class="login-main"><a class="compulsory">*</a>发票停止时间</div></el-col>
            <el-col :span="12">
              <el-date-picker
                      v-model="distributeend"
                      type="datetime"
              ></el-date-picker>
            </el-col>

          </el-row>
        </el-form-item>

        <el-form-item prop="capacity">
          <el-row :gutter="20">
            <el-col :span="5"><div align="left" class="login-main"><a class="compulsory">*</a>活动容量</div></el-col>
            <el-col :span="12"><el-input-number
                    v-model="capacity"
            ></el-input-number></el-col>

          </el-row>
        </el-form-item>

        <el-form-item prop="datetime">
          <el-row :gutter="20">
            <el-col :span="5"><div align="left" class="login-main"><a class="compulsory">*</a>活动开始时间</div></el-col>
            <el-col :span="12">
              <el-date-picker
                      v-model="date"
                      type="datetime"
              ></el-date-picker>
            </el-col>

          </el-row>
        </el-form-item>
        <el-form-item prop="method">
          <el-row :gutter="20">
            <el-col :span="5"><div align="left" class="login-main"><a class="compulsory">*</a>活动发票方式</div></el-col>
            <el-col :span="12">
              <el-radio v-model="method" label="0">抢票</el-radio>
              <el-radio v-model="method" label="1">扫码领票</el-radio>
              <el-radio v-model="method" label="2">申请审核</el-radio>
            </el-col>
          </el-row>
        </el-form-item>
        <el-form-item prop="type">
          <el-row :gutter="20">
            <el-col :span="5"><div align="left" class="login-main"><a class="compulsory">*</a>活动类别</div></el-col>
            <el-col :span="12">
              <el-select v-model="type">
                <el-option
                        v-for="type in types"
                        :key="type.value"
                        :label="type.label"
                        :value="type.value"
                ></el-option>
              </el-select>
            </el-col>
          </el-row>
        </el-form-item>

        <el-form-item prop="description">
          <el-row :gutter="20">
            <el-col :span="5"><div align="left" class="login-main"><a class="compulsory">*</a>活动简介</div></el-col>
            <el-col :span="12"><el-input
                    type="textarea"
                    v-model="description"
                    :autosize="{minRows:5,maxRows:20}"
                    auto-complete="off"
            ></el-input></el-col>
          </el-row>
        </el-form-item>
      </el-form>
      <el-row>
        <el-col>
          <el-button @click="post" type="primary" style="width: 10%">发布</el-button>
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
                time: '',
                method: '',
                type:'',
                description:'',
                distributestart:'',
                distributeend:'',
                capacity:'',

                types:[{
                    value:'0',
                    label:'讲座'
                },{
                    value:'1',
                    label:'文艺活动'
                },{
                    value:'2',
                    label:'其他'
                }
                ]
            }
        },
        methods:{
            post(){
                this.$router.push('/EventList')
            }
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
    display: flex;
    text-align: center;
    border-top: none;
    border-left: none;
    border-right: none;

  }
  .login-main {
    font-weight: 400;
    font-size: 18px;
  }
  .compulsory{
    color:#ff0000
  }

</style>