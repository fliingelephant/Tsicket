<template>
  <el-container class="container">

    <el-header class="header">
      <div>管理员菜单</div>
    </el-header>

    <el-main>
    <el-tabs v-model="activeName">
      <el-tab-pane label="活动管理" name="first">
        <el-table
                :data="events.slice((currentPage-1)*pageSize,currentPage*pageSize)"
                style="width: 100%">

          <el-table-column
                  prop="event_name"
                  label="活动名称">
          </el-table-column>

          <el-table-column
                  prop="event_time"
                  label="活动开始">
          </el-table-column>

          <el-table-column
                  prop="event_capacity"
                  label="活动容量">
          </el-table-column>

          <el-table-column
                  prop="event_status"
                  label="活动状态">
          </el-table-column>

          <el-table-column
                  label="操作"
                  min-width="120%"
          >
            <template slot-scope="scope">
              <el-button @click="eventInfo(scope.row)" type="text" size="small">查看</el-button>
              <el-button disabled type="text" size="small">通过审核</el-button>
              <el-button disabled type="text" size="small">取消活动</el-button>
              <el-button disabled type="text" size="small">通过推广申请</el-button>
            </template>
          </el-table-column>

        </el-table>

        <el-pagination
                @size-change="handleSizeChange"
                @current-change="handleCurrentChange"
                :current-page="currentPage"
                :page-sizes="[10, 20]"
                :page-size="pageSize"
                :total="events.length"
                layout="total, sizes, prev, pager, next, jumper">
        </el-pagination>



      </el-tab-pane>

    </el-tabs>
    </el-main>

  </el-container>
</template>


<script>
    export default {
        name: "AdminMenu",
        data() {
            return {
                events:[],
                activeName: 'first',
                currentPage:1,
                pageSize:10
            };
        },
        mounted(){
            this.getEvents()
        },
        methods: {
            getEvents(){
                this.$axios.get("/admins").then(response => {
                    if (response.status === 200) {
                        this.events=response.data.events
                    } else {
                        this.$message({
                            message: '获取活动失败',
                            type: 'error'
                        })
                    }
                })
            },
            handleSizeChange: function(val) {
                this.pageSize = val;
            },
            handleCurrentChange: function(page) {
                this.currentPage = page;
            },
            eventInfo(row){
                this.$router.push('/AdminEvent/'+row.event_id)
            },
        }
    };
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
</style>
