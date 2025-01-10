<template>
  <div class="container">
    <header>
      <form @submit.prevent="submitQuery" class="search-form">
        <div class="search-wrapper">
          <label :for="type">{{ label }}:</label>
          <div class="input-group">
            <input 
              type="text" 
              v-model="queryText" 
              :id="type" 
              placeholder="请输入搜索内容"
            />
            <button type="submit">查询</button>
          </div>
        </div>
      </form>
    </header>
    <main>
      <DataForm :tableData="dataList" />
    </main>
  </div>
</template>

<script setup lang="ts">
import DataForm from "./Form.vue";
import { ref } from "vue";
import type { DataModel } from "../types";
import axios from "axios";
import { isDev } from "../utils";
import { ElLoading } from "element-plus";

const props = defineProps<{
  type: string
  endpoint: string
  label: string
}>();

const isDevMode = isDev();
const queryText = ref('');
const dataList = ref<DataModel[]>([]);

const submitQuery = async () => {
  const loading = ElLoading.service({
    lock: true,
    text: '加载中...',
    background: 'rgba(0, 0, 0, 0.7)'
  });

  try {
    console.log(`查询字符串: ${queryText.value}`);
    const endpoint = props.endpoint;
    const baseUrl = isDevMode ? 'http://localhost:4000' : '';
    
    const res = await axios.get(
      `${baseUrl}/${endpoint}?${props.type}=${queryText.value.trim()}`
    );
    console.log("res:", res);
    dataList.value = res.data;
  } catch (error) {
    console.error('查询出错:', error);
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

.search-wrapper {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.search-wrapper label {
  font-weight: 500;
  color: #333;
}

.input-group {
  display: flex;
  gap: 10px;
}

.input-group input {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid #dcdfe6;
  border-radius: 4px;
  font-size: 14px;
  transition: all 0.3s;
}

.input-group input:focus {
  border-color: #409eff;
  outline: none;
  box-shadow: 0 0 0 2px rgba(64, 158, 255, 0.2);
}

button {
  padding: 8px 20px;
  background-color: #409eff;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  transition: background-color 0.3s;
}

button:hover {
  background-color: #66b1ff;
}

button:active {
  background-color: #3a8ee6;
}
</style> 