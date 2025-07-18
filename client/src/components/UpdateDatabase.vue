<template>
  <div class="update-container">
    <h3>最后更新时间：{{ lastUpdatedTime }}</h3>
    <el-form
      @submit.prevent="submitRequest"
      label-position="top"
      class="update-form"
      autocomplete="on"
    >
      <el-row :gutter="20">
        <el-col :span="8">
          <el-form-item label="用户名">
            <el-input
              v-model="username"
              placeholder="请输入用户名"
              autocomplete="username"
              :disabled="isLoading"
            />
          </el-form-item>
        </el-col>

        <el-col :span="8">
          <el-form-item label="密码">
            <el-input
              v-model="password"
              placeholder="请输入密码"
              :disabled="isLoading"
              autocomplete="current-password"
              show-password
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
          {{ isLoading ? '更新中...' : '更新' }}
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
import { ref, onMounted } from 'vue';
import { Key } from '@element-plus/icons-vue';
import { isDev } from '../utils';
import axios from 'axios';

const password = ref('');
const currentTime = new Date(new Date().setHours(new Date().getHours() + 8));
const date = ref(currentTime.toISOString().split('T')[0]);
const username = ref('');
const isLoading = ref(false);
const message = ref('');
const messageType = ref('success');
const lastUpdatedTime = ref('');
onMounted(() => {
  if (isDev()) {
    axios.get('http://localhost:4000/getLastUpdated').then((res) => {
      lastUpdatedTime.value = res.data;
    });
  } else {
    axios.get('/getLastUpdated').then((res) => {
      lastUpdatedTime.value = res.data;
    });
  }
});

const submitRequest = async () => {
  isLoading.value = true;
  message.value = '';

  try {
    let res;
    if (username.value.length < 3) {
      message.value = '用户名错误';
      messageType.value = 'error';
      return;
    }
    let dateObj = new Date(date.value);
    if (dateObj.getTime() > new Date(currentTime).getTime()) {
      message.value = '日期错误';
      messageType.value = 'error';
      return;
    }
    if (isDev()) {
      res = await axios.get('http://localhost:4000/import', {
        params: {
          password: password.value,
          date: date.value,
          username: username.value,
        },
      });
    } else {
      res = await axios.get('/import', {
        params: {
          password: password.value,
          date: date.value,
          username: username.value,
        },
      });
    }
    message.value = res?.data;
    messageType.value = 'success';
  } catch (error: any) {
    message.value = `错误: ${error.message}`;
    messageType.value = 'error';
  } finally {
    isLoading.value = false;
  }
};
</script>

<style scoped>
.update-container {
  padding: 20px;
  height: 100%;
  box-sizing: border-box;
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
