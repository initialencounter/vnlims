<template>
  <div class="update-container">
    <h2 class="page-title">更新数据库</h2>

    <el-form
      @submit.prevent="submitRequest"
      label-position="top"
      class="update-form"
    >
      <el-row :gutter="20">
        <el-col :span="8">
          <el-form-item label="用户名">
            <el-input
              v-model="userName"
              placeholder="请输入用户名"
              :disabled="isLoading"
            />
          </el-form-item>
        </el-col>

        <el-col :span="8">
          <el-form-item label="Session ID">
            <el-input
              v-model="sessionId"
              placeholder="请输入 Session ID"
              :disabled="isLoading"
            >
              <template #prefix>
                <el-icon><Key /></el-icon>
              </template>
            </el-input>
          </el-form-item>
        </el-col>

        <el-col :span="8">
          <el-form-item label="日期">
            <el-date-picker
              v-model="date"
              type="date"
              placeholder="选择日期"
              format="YYYY-MM-DD"
              value-format="YYYY-MM-DD"
              :disabled="isLoading"
              style="width: 100%"
            />
          </el-form-item>
        </el-col>
      </el-row>

      <el-form-item>
        <el-button
          type="primary"
          :loading="isLoading"
          @click="submitRequest"
          class="submit-button"
        >
          {{ isLoading ? "更新中..." : "更新" }}
        </el-button>
      </el-form-item>

      <el-alert
        v-if="message"
        :title="message"
        :type="messageType === 'success' ? 'success' : 'error'"
        show-icon
        :closable="true"
      />
    </el-form>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { Key } from "@element-plus/icons-vue";
import { isDev } from "../utils";
import axios from "axios";

const sessionId = ref("d87825c3-c7ce-467c-af63-9b6c2bb86dd5");
const date = ref("2025-01-10");
const userName = ref("zengwang");
const isLoading = ref(false);
const message = ref("");
const messageType = ref("success");

const submitRequest = async () => {
  isLoading.value = true;
  message.value = "";

  try {
    let res;
    if (sessionId.value.length < 30) {
      message.value = "Session ID 错误";
      messageType.value = "error";
      return;
    }
    if (isDev()) {
      res = await axios.get("http://localhost:4000/import", {
        params: {
          sessionId: sessionId.value,
          date: date.value,
          userName: userName.value,
        },
      });
    } else {
      res = await axios.get("/import", {
        params: {
          sessionId: sessionId.value,
          date: date.value,
          userName: userName.value,
        },
      });
    }
    message.value = res?.data;
    messageType.value = "success";
  } catch (error: any) {
    message.value = `错误: ${error.message}`;
    messageType.value = "error";
  } finally {
    isLoading.value = false;
  }
};
</script>

<style scoped>
.update-container {
  padding: 20px;
}

.page-title {
  margin-bottom: 30px;
  font-size: 24px;
  color: #303133;
}

.update-form {
  padding: 30px;
  border-radius: 8px;
  box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.05);
}

.submit-button {
  min-width: 120px;
}

.el-alert {
  margin-top: 20px;
}
</style>
