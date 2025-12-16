<template>
  <div class="search-container">
    <div class="search-form-card">
      <el-form
        :model="query"
        @submit.prevent="submitQuery"
        label-position="top"
        class="search-form"
      >
        <el-row :gutter="16">
          <el-col :span="8">
            <el-form-item label="运输方式">
              <el-select
                v-model="query.systemId"
                placeholder="请选择运输方式"
                size="default"
              >
                <el-option label="全部" value="all" />
                <el-option label="空运" value="pek" />
                <el-option label="海运" value="sek" />
                <el-option label="陆运" value="aek" />
                <el-option label="铁路" value="sek" />
              </el-select>
            </el-form-item>
          </el-col>

          <el-col :span="8">
            <el-form-item label="物品种类">
              <el-select
                v-model="query.category"
                placeholder="请选择类别"
                size="default"
              >
                <el-option label="化学品" value="chemical" />
                <el-option label="锂电池类" value="battery" />
                <el-option label="钠离子电池类" value="sodium" />
                <el-option label="器械类" value="device" />
                <el-option label="磁性物质" value="magnetic" />
              </el-select>
            </el-form-item>
          </el-col>

          <el-col :span="8">
            <el-form-item label="委托类型">
              <el-select
                v-model="query.reportType"
                placeholder="请选择委托类型"
                size="default"
              >
                <el-option label="全部" value="" />
                <el-option label="初验" value="0" />
                <el-option label="换证" value="1" />
              </el-select>
            </el-form-item>
          </el-col>
        </el-row>

        <el-row :gutter="16">
          <el-col :span="8">
            <el-form-item label="主检员">
              <el-input
                clearable
                autocomplete="on"
                v-model="query.appraiserName"
                placeholder="请输入主检员姓名"
                size="default"
              />
            </el-form-item>
          </el-col>

          <el-col :span="8">
            <el-form-item label="物品名称">
              <el-input
                clearable
                autocomplete="on"
                v-model="query.itemName"
                placeholder="请输入物品名称"
                size="default"
              />
            </el-form-item>
          </el-col>

          <el-col :span="8">
            <el-form-item label="委托单位">
              <el-input
                clearable
                autocomplete="on"
                v-model="query.principal"
                placeholder="请输入委托单位"
                size="default"
              />
            </el-form-item>
          </el-col>
        </el-row>

        <el-row :gutter="16">
          <el-col :span="6">
            <el-form-item label="开始日期">
              <el-date-picker
                v-model="query.startDate"
                type="date"
                placeholder="选择开始日期"
                size="default"
                style="width: 100%"
                value-format="YYYY-MM-DD"
                @clear="query.startDate = ''"
              />
            </el-form-item>
          </el-col>

          <el-col :span="6">
            <el-form-item label="结束日期">
              <el-date-picker
                v-model="query.endDate"
                type="date"
                placeholder="选择结束日期"
                size="default"
                style="width: 100%"
                value-format="YYYY-MM-DD"
                @clear="query.endDate = ''"
              />
            </el-form-item>
          </el-col>

          <el-col :span="6">
            <el-form-item label="项目编号">
              <el-input
                clearable
                autocomplete="on"
                v-model="query.projectNo"
                placeholder="请输入项目编号"
                size="default"
              />
            </el-form-item>
          </el-col>

          <el-col :span="6">
            <el-form-item label="显示条数">
              <el-input-number
                v-model="query.rows"
                :min="1"
                :max="5000"
                size="default"
                style="width: 100%"
              />
            </el-form-item>
          </el-col>
        </el-row>

        <el-form-item class="submit-form-item">
          <el-button
            type="primary"
            size="default"
            @click="submitQuery"
            :loading="isLoading"
            class="submit-btn"
          >
            <el-icon class="mr-2"><Search /></el-icon>
            {{ isLoading ? "查询中..." : "开始查询" }}
          </el-button>
        </el-form-item>
      </el-form>
    </div>

    <div class="results-section" v-if="dataList.length > 0">
      <div class="results-header">
        <h4 class="results-title">查询结果</h4>
        <span class="results-count">共找到 {{ dataList.length }} 条记录</span>
      </div>
      <DataForm :tableData="dataList" />
    </div>
  </div>
</template>

<script setup lang="ts">
import DataForm from "./Form.vue";
import { ref } from "vue";
import type { DataModel } from "../types";
import { Search } from "@element-plus/icons-vue";
import axios from "axios";
import { isDev } from "../utils";
import { ElLoading, ElMessage } from "element-plus";
import { useSearchStore } from "../stores/search";

const isDevMode = isDev();
const searchStore = useSearchStore();
const query = ref<Record<string, string | number>>(searchStore.homeQuery);
const dataList = ref<DataModel[]>(searchStore.homeResults);
const isLoading = ref(false);

const submitQuery = async () => {
  isLoading.value = true;

  const loading = ElLoading.service({
    lock: true,
    text: "正在查询数据...",
    background: "rgba(0, 0, 0, 0.7)",
  });

  try {
    const trimmedQuery = Object.fromEntries(
      Object.entries(query.value).map(([key, value]) => [
        key,
        typeof value === "string" ? value.trim() : value,
      ]),
    );

    if (trimmedQuery["systemId"] === "all") {
      trimmedQuery["systemId"] = "";
    }

    if (trimmedQuery["projectNo"]) {
      trimmedQuery["startDate"] = "";
      trimmedQuery["endDate"] = "";
    }

    const queryString = new URLSearchParams(
      trimmedQuery as Record<string, string>,
    ).toString();

    const baseUrl = isDevMode ? "http://localhost:4000" : "";
    const res = await axios.get(`${baseUrl}/search?${queryString}`);

    let data: DataModel[] = res.data;
    searchStore.setHomeResults(data);
    dataList.value = data.slice(0, Number(query.value.rows));

    ElMessage.success(`查询完成，共找到 ${data.length} 条记录`);
  } catch (error) {
    console.error("查询出错:", error);
    ElMessage.error("查询失败，请稍后重试");
  } finally {
    loading.close();
    isLoading.value = false;
  }
};
</script>

<style scoped>
.search-container {
  padding: 20px;
  max-width: 1200px;
  margin: 0 auto;
  height: 100%;
  box-sizing: border-box;
  width: 100%;
}

.search-header {
  margin-bottom: 20px;
}

.search-title {
  font-size: 22px;
  font-weight: 600;
  color: var(--el-text-color-primary);
  margin: 0 0 4px 0;
}

.search-subtitle {
  font-size: 14px;
  color: var(--el-text-color-secondary);
  margin: 0;
}

.search-form-card {
  background: var(--el-bg-color);
  border-radius: 12px;
  padding: 20px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.06);
  border: 1px solid var(--el-border-color-lighter);
  margin-bottom: 24px;
}

.search-form .el-form-item {
  margin-bottom: 16px;
}

.search-form :deep(.el-form-item__label) {
  font-weight: 500;
  color: var(--el-text-color-primary);
  margin-bottom: 4px;
  font-size: 14px;
  padding-bottom: 0;
  line-height: 1.2;
}

.submit-form-item {
  margin-top: 4px;
  margin-bottom: 0;
  text-align: center;
}

.submit-btn {
  padding: 8px 32px;
  font-size: 14px;
  border-radius: 6px;
  box-shadow: 0 2px 8px rgba(64, 158, 255, 0.2);
}

.results-section {
  margin-top: 24px;
}

.results-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 16px;
  padding: 0 4px;
}

.results-title {
  font-size: 18px;
  font-weight: 600;
  color: var(--el-text-color-primary);
  margin: 0;
}

.results-count {
  font-size: 13px;
  color: var(--el-text-color-secondary);
  background: var(--el-fill-color-light);
  padding: 4px 10px;
  border-radius: 16px;
}

.mr-2 {
  margin-right: 6px;
}

/* 响应式处理 */
@media (max-width: 1400px) {
  .search-container {
    padding: 16px;
    max-width: none;
  }
}

@media (max-width: 1200px) {
  .search-container {
    padding: 12px;
  }

  .search-form :deep(.el-col) {
    margin-bottom: 8px;
  }
}

@media (max-width: 768px) {
  .search-container {
    padding: 8px;
  }

  .search-form :deep(.el-row) {
    margin: 0 -4px;
  }

  .search-form :deep(.el-col) {
    padding: 0 4px;
  }
}
</style>
