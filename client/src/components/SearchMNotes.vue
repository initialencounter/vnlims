<template>
  <div class="container">
    <header>
      <form @submit.prevent="submitQuery" class="search-form">
        <div class="search-wrapper">
          <label for="mNotes">市场部备注:</label>
          <div class="input-group">
            <input 
              type="text" 
              v-model="query.mNotes" 
              id="mNotes" 
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
import { projectTracking } from "../utils";
import axios from "axios";
import { isDev } from "../utils";
import { ElLoading } from "element-plus";

const isDevMode = isDev();
let today = new Date().toISOString().split("T")[0];
let lastMonth = new Date(new Date().setMonth(new Date().getMonth() - 1))
  .toISOString()
  .split("T")[0];
const query = ref<Record<string, string | number>>({
  mNotes: "",
});

let dataList = ref<DataModel[]>([]);
const submitQuery = async () => {
  const loading = ElLoading.service({
    lock: true,
    text: '加载中...',
    background: 'rgba(0, 0, 0, 0.7)'
  });

  try {
    console.log(`查询字符串: ${query.value.mNotes}`);
    if (isDevMode) {
      const res = await axios.get(
        `http://localhost:4000/searchMNotes?mNotes=${query.value.mNotes}`
      );
      console.log("res:", res);
      dataList.value = res.data;
    } else {
      const res = await axios.get(`/searchMNotes?mNotes=${query.value.mNotes}`);
      dataList.value = res.data;
    }
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
