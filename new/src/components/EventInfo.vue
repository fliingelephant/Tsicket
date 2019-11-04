<template>
    <el-container>
        <el-title>发布活动</el-title>
        <el-main>
            <el-form>
                <el-form-item>
                    <div>活动名称</div>
                    <el-input
                            type="text"
                            auto-complete="off"
                    ></el-input>
                </el-form-item>
                <el-form-item>
                    <el-date-picker
                            v-model="dateTime"
                    ></el-date-picker>
                </el-form-item>
                <el-form-item>
                    <div>抢票开始时间</div>
                    <el-date-picker
                            v-model="dateTime"
                    ></el-date-picker>
                </el-form-item>
                <el-form-item>
                    <div>抢票结束时间</div>
                    <el-date-picker
                            v-model="dateTime"
                    ></el-date-picker>
                </el-form-item>
                <el-form-item>
                    <div>活动容量</div>
                    <el-input
                            type="number"
                            auto-complete="off"
                    ></el-input>
                </el-form-item>
                <el-form-item>
                    <div>活动简介</div>
                    <el-input
                            type="textarea"
                            auto-complete="off"
                    ></el-input>
                </el-form-item>
            </el-form>
        </el-main>
    </el-container>
</template>

<script>
    export default {
        name: "EventInfo"
    }
</script>

<style scoped>

</style>