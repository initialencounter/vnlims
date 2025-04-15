<script setup lang="ts">
import { useDark, useToggle } from '@vueuse/core';
import { Moon, Sunny } from '@element-plus/icons-vue';
import Sidebar from './components/Sidebar.vue';

const isDark = useDark();
const toggleDark = useToggle(isDark);
</script>

<template>
  <div class="app-container">
    <Sidebar />
    <div class="top-bar">
      <div class="icon-btn" @click="toggleDark()">
        <div v-if="isDark">
          <el-icon><Moon /></el-icon>
        </div>
        <div v-else>
          <el-icon><Sunny /></el-icon>
        </div>
      </div>
    </div>
    <router-view></router-view>
  </div>
</template>

<style scoped>
.app-container {
  display: flex;
  width: 100%;
  height: 100vh;
}

.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-width: 0;
  width: calc(100% - 250px);
}

.top-bar {
  height: 50px;
  border-bottom: 1px solid var(--el-border-color);
  display: flex;
  align-items: center;
  padding: 0 20px;
}

main {
  flex: 1;
  padding: 20px;
  overflow-y: auto;
}

.icon-btn {
  cursor: pointer;
  padding: 8px;
  border-radius: 4px;
}

.icon-btn:hover {
  background-color: var(--el-fill-color-light);
}
</style>
