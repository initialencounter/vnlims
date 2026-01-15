<template>
  <div class="home">
    <!-- 登录界面 -->
    <div v-if="!loginStatus" class="login-container">
      <el-card class="login-card">
        <template #header>
          <div class="card-header">
            <span>用户登录</span>
          </div>
        </template>
        <el-form :model="loginForm" label-width="80px">
          <el-form-item label="用户名">
            <el-input v-model="loginForm.username" placeholder="请输入用户名" />
          </el-form-item>
          <el-form-item label="密码">
            <el-input
              v-model="loginForm.password"
              type="password"
              placeholder="请输入密码"
              show-password
            />
          </el-form-item>
          <el-form-item label="验证码">
            <div class="captcha-container">
              <el-image
                v-if="captchaImage"
                :src="captchaImage"
                fit="contain"
                class="captcha-image"
                @click="getCaptcha"
              >
                <template #error>
                  <div class="image-error">
                    <el-icon><Picture /></el-icon>
                    <span>加载失败</span>
                  </div>
                </template>
              </el-image>
              <el-button
                type="primary"
                @click="getCaptcha"
                :loading="loadingCaptcha"
              >
                {{ captchaImage ? '刷新验证码' : '获取验证码' }}
              </el-button>
            </div>
          </el-form-item>
          <el-form-item label="验证码">
            <el-input
              v-model="loginForm.code"
              placeholder="请输入验证码"
              @keyup.enter="handleLogin"
            />
          </el-form-item>
          <el-form-item>
            <el-button
              type="primary"
              @click="handleLogin"
              :loading="loggingIn"
              :disabled="!loginForm.code"
              style="width: 100%"
            >
              登录
            </el-button>
          </el-form-item>
        </el-form>
      </el-card>
    </div>
    <div v-else class="welcome-container">
      <el-result icon="success" title="登录成功"> </el-result>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref, onBeforeUnmount } from 'vue';
import { ElMessage } from 'element-plus';
import { Picture } from '@element-plus/icons-vue';
import { apiManager } from '../utils/api';

const loginStatus = ref(false);

const loginForm = ref({
  username: '',
  password: '',
  code: '',
});

const captchaImage = ref('');
const loadingCaptcha = ref(false);
const loggingIn = ref(false);

const getCaptcha = async () => {
  loadingCaptcha.value = true;
  try {
    const data = await apiManager.post('/get-captcha', {});
    if (data.img) {
      captchaImage.value = data.img;
      loginForm.value.code = ''; // 清空验证码输入
    } else if (data.message) {
      ElMessage.error(data.message);
    }
  } catch (error) {
    ElMessage.error('获取验证码失败: ' + error);
  } finally {
    loadingCaptcha.value = false;
  }
};

const handleLogin = async () => {
  if (
    loginForm.value.code.length === 0 ||
    !loginForm.value.username ||
    !loginForm.value.password
  ) {
    ElMessage.warning('请填充完整的登录信息');
    return;
  }

  loggingIn.value = true;
  try {
    // 使用用户在表单中填写或修改的账号密码
    const response = await fetch('/login', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        code: loginForm.value.code,
        username: loginForm.value.username,
        password: loginForm.value.password,
      }),
    });

    const data = await response.json();

    if (data.success) {
      ElMessage.success('登录成功');
    } else if (data.message) {
      ElMessage.error(data.message);
      // 登录失败后重新获取验证码
      getCaptcha();
    }
  } catch (error) {
    ElMessage.error('登录失败: ' + error);
    getCaptcha();
  } finally {
    loggingIn.value = false;
  }
};

let interval: any = null;
onMounted(() => {
  console.log('Starting login status interval');
  interval = setInterval(async () => {
    loginStatus.value = (await apiManager.get('get-login-status')).logged_in;
  }, 1000);
});

onBeforeUnmount(() => {
  if (interval) {
    console.log('Clearing login status interval');
    clearInterval(interval);
  }
});
</script>

<style scoped>
.home {
  padding: 20px;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.login-container {
  margin-top: 20px;
  width: 100%;
  max-width: 400px;
}

.login-card {
  box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.1);
}

.card-header {
  font-size: 18px;
  font-weight: bold;
  text-align: center;
}

.captcha-container {
  display: flex;
  gap: 10px;
  align-items: center;
  width: 100%;
}

.captcha-image {
  width: 150px;
  height: 50px;
  cursor: pointer;
  border-radius: 4px;
}

.image-error {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
}

.welcome-container {
  margin-top: 50px;
}

:deep(.el-card) {
  background: #363636;
}
</style>
