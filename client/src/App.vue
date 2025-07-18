<script setup lang="ts">
import { useDark, useToggle } from '@vueuse/core';
import { Moon, Sunny } from '@element-plus/icons-vue';
import Sidebar from './components/Sidebar.vue';
import { computed, ref, onMounted } from 'vue';
import { useRoute } from 'vue-router';

const isDark = useDark();
const toggleDark = useToggle(isDark);
const route = useRoute();
// Define the type for Sidebar component instance
type SidebarInstance = InstanceType<typeof Sidebar> | null;
const sidebarRef = ref<SidebarInstance>(null);
interface MenuItem {
  icon: any;
  title: string;
  path: string;
  description: string;
}
const menuItems = ref<MenuItem[]>([]);

onMounted(() => {
  if (sidebarRef.value) {
    menuItems.value = sidebarRef.value.menuItems;
  }
});

const currentPageTitle = computed(() => {
  const currentPath = route.path;
  const currentMenuItem = menuItems.value.find(item => item.path === currentPath);
  return currentMenuItem ? currentMenuItem.title : 'DATABASE';
});
</script>

<template>
  <div class="app-container">
    <Sidebar ref="sidebarRef" />
    <div class="main-content">
      <div class="top-bar">
        <div class="top-bar-left">
          <h2 class="page-title">{{ currentPageTitle }}</h2>
        </div>
        <div class="top-bar-right">
          <div class="theme-toggle" @click="toggleDark()">
            <el-icon size="18">
              <Moon v-if="isDark" />
              <Sunny v-else />
            </el-icon>
          </div>
        </div>
      </div>
      <div class="content-wrapper">
        <router-view />
      </div>
    </div>
  </div>
</template>

<style scoped>
.app-container {
  display: flex;
  width: 100%;
  min-width: 100%;
  height: 100vh;
  min-height: 100vh;
  background: var(--el-bg-color-page);
  overflow: hidden;
}

.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-width: 0;
  height: 100vh;
  overflow: hidden;
}

.top-bar {
  height: 64px;
  min-height: 64px;
  max-height: 64px;
  background: var(--el-bg-color);
  border-bottom: 1px solid var(--el-border-color-light);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 24px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.08);
  z-index: 100;
  flex-shrink: 0;
}

.top-bar-left {
  display: flex;
  align-items: center;
}

.page-title {
  font-size: 20px;
  font-weight: 600;
  color: var(--el-text-color-primary);
  margin: 0;
}

.top-bar-right {
  display: flex;
  align-items: center;
  gap: 16px;
}

.theme-toggle {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  height: 40px;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease;
  color: var(--el-text-color-regular);
}

.theme-toggle:hover {
  background: var(--el-fill-color-light);
  color: var(--el-color-primary);
}

.content-wrapper {
  flex: 1;
  overflow: auto;
  background: var(--el-bg-color-page);
  min-height: 0;
}
</style>