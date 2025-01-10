<template>
  <div class="container">
    <header>
      <form @submit.prevent="submitQuery" class="form">
        <div class="form-group">
          <label for="systemId">运输方式:</label>
          <el-select
            v-model="query.systemId"
            id="systemId"
            placeholder="请选择运输方式"
          >
            <el-option label="空运" value="pek"></el-option>
            <el-option label="海运" value="sek"></el-option>
            <el-option label="陆运" value="aek"></el-option>
            <el-option label="铁路" value="sek"></el-option>
          </el-select>
        </div>
        <div class="form-group">
          <label for="category">物品种类:</label>
          <el-select
            v-model="query.category"
            id="category"
            placeholder="请选择类别"
          >
            <el-option label="化学品" value="chemical"></el-option>
            <el-option label="锂电池类" value="battery"></el-option>
            <el-option label="钠离子电池类" value="sodium"></el-option>
            <el-option label="器械类" value="device"></el-option>
            <el-option label="磁性物质" value="magnetic"></el-option>
          </el-select>
        </div>
        <div class="form-group">
          <label for="reportType">委托类型:</label>
          <el-select
            v-model="query.reportType"
            id="reportType"
            placeholder="请选择委托类型"
          >
            <el-option label="全部" value=""></el-option>
            <el-option label="初验" value="0"></el-option>
            <el-option label="换证" value="1"></el-option>
          </el-select>
        </div>
        <div class="form-group">
          <label for="appraiserName">主检员:</label>
          <input type="text" v-model="query.appraiserName" id="appraiserName" />
        </div>
        <div class="form-group">
          <label for="itemName">物品名称:</label>
          <input type="text" v-model="query.itemName" id="itemName" />
        </div>
        <div class="form-group">
          <label for="principal">委托单位:</label>
          <input type="text" v-model="query.principal" id="principal" />
        </div>
        <div class="form-group">
          <label for="startDate">开始日期:</label>
          <input type="date" v-model="query.startDate" id="startDate" />
        </div>
        <div class="form-group">
          <label for="endDate">结束日期:</label>
          <input type="date" v-model="query.endDate" id="endDate" />
        </div>
        <div class="form-group">
          <label for="projectNo">项目编号:</label>
          <input type="text" v-model="query.projectNo" id="projectNo" />
        </div>
        <div class="form-group">
          <label for="page">页码:</label>
          <input type="number" v-model="query.page" id="page" value="1" />
        </div>
        <div class="form-group">
          <label for="rows">行数:</label>
          <input type="number" v-model="query.rows" id="rows" value="3000" />
        </div>
        <button type="submit">查询</button>
      </form>
    </header>
    <main>
      <!-- <div id="dataDisplay"> -->
      <DataForm :tableData="dataList" />
      <!-- </div> -->
    </main>
  </div>
</template>

<script setup lang="ts">
import DataForm from './Form.vue';
import { ref } from 'vue';
import type { DataModel } from '../types';
import { projectTracking } from '../utils';
import axios from 'axios';
import { isDev } from '../utils';
import { ElLoading } from 'element-plus';
import { useSearchStore } from '../stores/search';

const isDevMode = isDev();
const searchStore = useSearchStore();
const query = ref<Record<string, string | number>>(searchStore.homeQuery);

let dataList = ref<DataModel[]>(searchStore.homeResults);
const submitQuery = async () => {
  const loading = ElLoading.service({
    lock: true,
    text: '加载中...',
    background: 'rgba(0, 0, 0, 0.7)'
  });
  
  try {
    const trimmedQuery = Object.fromEntries(
      Object.entries(query.value).map(([key, value]) => [key, typeof value === 'string' ? value.trim() : value])
    );
    const queryString = new URLSearchParams(trimmedQuery as Record<string, string>).toString();
    console.log(`查询字符串: ${queryString}`);
    
    if (isDevMode) {
      const res = await axios.get(`http://localhost:4000/search?${queryString}`);
      let data: DataModel[] = res.data;
      console.log('res:', res);
      searchStore.setHomeResults(data);
      dataList.value = data.slice(0, Number(query.value.rows));
    } else {
      const res = await axios.get(`/search?${queryString}`);
      let data: DataModel[] = res.data;
      searchStore.setHomeResults(data);
      dataList.value = data.slice(0, Number(query.value.rows));
    }
  } catch (error) {
    console.error('查询出错:', error);
    // 可以在这里添加错误提示
  } finally {
    loading.close();
  }
};
</script>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: flex-start;
  height: 100vh;
}

.header {
  width: 100%;
}

.form {
  display: grid;
  grid-template-columns: repeat(6, 1fr);
  gap: 15px;
  max-width: 1200px;
  padding: 20px;
  border-radius: 8px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
}

.form-group {
  display: flex;
  flex-direction: column;
}

.form-group label {
  margin-bottom: 5px;
  font-weight: bold;
}

.form-group input {
  padding: 10px;
  border: 1px solid #ccc;
  border-radius: 4px;
  transition: border-color 0.3s;
}

.form-group input:focus {
  border-color: #007bff;
  outline: none;
}

button {
  padding: 10px 20px;
  background-color: #007bff;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.3s;
}

button:hover {
  background-color: #0056b3;
}

#dataDisplay {
  margin-top: 20px;
  width: 100%;
  height: calc(100% - 300px);
  border-radius: 4px;
  padding: 10px;
}
</style>
