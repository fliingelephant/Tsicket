<template>
  <el-container class="container">

    <el-header class="header">
      <div>管理员登录</div>
    </el-header>

    <el-main>
      <el-form>
        <el-form-item>
          <div align="left" class="login-text">用户名：</div>
          <el-input
                  v-model="username"
                  type="text"
          ></el-input>
        </el-form-item>

        <el-form-item>
          <div align="left" class="login-text">密码：</div>
          <el-input
                  v-model="password"
                  type="password"
          ></el-input>
        </el-form-item>


        <el-form-item>
          <div align="left" class="login-text">验证码：</div>
          <el-row>
            <el-col :span="12">
              <el-input
                      v-model="input_identify_code"
                      type="text"
              ></el-input>

            </el-col>
            <el-col :span="12">
              <canvas id="id_canavas" :width="width" :height="height" @click="referesh" ></canvas>
            </el-col>
          </el-row>
        </el-form-item>

      </el-form>

      <el-row>
        <el-col>
          <el-button @click="login" type="primary" style="width: 30%">登录</el-button>
        </el-col>
      </el-row>

    </el-main>
  </el-container>
</template>

<script>
    export default {
        name: "LogIn",
        data() {
            return {
                username: '',
                password: '',
                input_identify_code:"",
                identify_code:"",
                min_size:25,
                max_size:40,
                min_bg_color:200,
                max_bg_color:250,
                min_font_color:60,
                max_font_color:180,
                min_line_color:100,
                max_line_color:140,
                min_dot_color:125,
                max_dot_color:255,
                width:140,
                height:40
            }
        },
        methods:{
            login(){
                if(this.input_identify_code===this.identify_code) {


                    let data = {
                        "admin_id": this.username,
                        "password": this.password
                    }
                    this.$axios.post("/admins/login", data).then(response => {
                        if (response.status === 200) {
                            let resdata = {
                                username: this.username,
                                admin: true
                            }
                            this.$store.commit('login', resdata)
                            this.$message({
                                message: '登录成功',
                                type: 'success'
                            })
                            this.$router.push('/AdminMenu')
                        } else {
                            this.$message({
                                message: '登录失败',
                                type: 'error'
                            })
                            this.drawPic()
                        }
                    }, err => {
                        if (err.response.status === 422)
                            this.$message({
                                message: '登录失败：用户名或密码错误',
                                type: 'error'
                            })
                        this.drawPic()
                    })
                }
                else{
                    this.$message.error("验证码错误")
                    this.drawPic()
                }
            },
            randomNum(min, max) {
                return Math.floor(Math.random() * (max - min) + min)
            },
            // 生成一个随机的颜色
            randomColor(min, max) {
                var r = this.randomNum(min, max)
                var g = this.randomNum(min, max)
                var b = this.randomNum(min, max)
                return 'rgb(' + r + ',' + g + ',' + b + ')'
            },
            drawPic() {
                this.identify_code=Math.floor(Math.random() * (9999 - 1000) + 1000).toString()
                var canvas = document.getElementById('id_canavas')
                var ctx = canvas.getContext('2d')
                ctx.textBaseline = 'bottom'
                // 绘制背景
                ctx.fillStyle = this.randomColor(
                    this.min_bg_color,
                    this.max_bg_color
                )
                ctx.fillRect(0, 0, this.width, this.height)
                // 绘制文字
                for (let i = 0; i < this.identify_code.length; i++) {
                    this.drawText(ctx, this.identify_code[i], i)
                }
                this.drawLine(ctx)
                this.drawDot(ctx)
            },
            drawText(ctx, txt, i) {
                ctx.fillStyle = this.randomColor(this.min_font_color, this.max_font_color)
                ctx.font =
                    this.randomNum(this.min_size, this.max_size) + 'px SimHei'
                var x = (i + 1) * (this.width / (this.identify_code.length + 1))
                var y = this.randomNum(this.max_size, this.height - 5)
                var deg = this.randomNum(-45, 45)
                // 修改坐标原点和旋转角度
                ctx.translate(x, y)
                ctx.rotate(deg * Math.PI / 180)
                ctx.fillText(txt, 0, 0)
                // 恢复坐标原点和旋转角度
                ctx.rotate(-deg * Math.PI / 180)
                ctx.translate(-x, -y)
            },
            drawLine(ctx) {
                // 绘制干扰线
                for (let i = 0; i < 8; i++) {
                    ctx.strokeStyle = this.randomColor(
                        this.min_line_color,
                        this.max_line_color
                    )
                    ctx.beginPath()
                    ctx.moveTo(
                        this.randomNum(0, this.width),
                        this.randomNum(0, this.height)
                    )
                    ctx.lineTo(
                        this.randomNum(0, this.width),
                        this.randomNum(0, this.height)
                    )
                    ctx.stroke()
                }
            },
            drawDot(ctx) {
                // 绘制干扰点
                for (let i = 0; i < 100; i++) {
                    ctx.fillStyle = this.randomColor(0, 255)
                    ctx.beginPath()
                    ctx.arc(
                        this.randomNum(0, this.width),
                        this.randomNum(0, this.height),
                        1,
                        0,
                        2 * Math.PI
                    )
                    ctx.fill()
                }
            },
            referesh(){
                this.drawPic()
            },
        },
        mounted(){
            this.drawPic()
        }
    }
</script>

<style scoped>
  .container{
    border-radius: 20px;
    background-clip: padding-box;
    margin: 100px auto;
    padding: 20px;
    width: 400px;
    border: 1px solid silver;
    font-weight: 600;
    font-size: 25px;
    text-align: center;
  }
  .header {
    text-align: center;
    border-bottom: 1px solid silver;
  }
  .login-text {
    font-weight: 400;
    font-size: 16px;
  }
</style>
