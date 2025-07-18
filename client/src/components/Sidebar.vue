<script setup lang="ts">
import { ref } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { House, Search, Fold, Expand, Refresh, Document } from '@element-plus/icons-vue';

const router = useRouter();
const route = useRoute();
const isCollapse = ref(false);

const menuItems = [
  {
    icon: House,
    title: '首页',
    path: '/search',
    description: '数据查询与概览'
  },
  {
    icon: Document,
    title: '技术部备注',
    path: '/',
    description: '搜索技术部备注信息'
  },
  {
    icon: Document,
    title: '市场部备注',
    path: '/searchMNotes',
    description: '搜索市场部备注信息'
  },
  {
    icon: Search,
    title: '项目名称',
    path: '/searchItemCName',
    description: '按项目名称搜索'
  },
  {
    icon: Search,
    title: '委托方搜索', 
    path: '/searchPrincipal',
    description: '按委托方搜索'
  },
  {
    icon: Refresh,
    title: '更新数据库',
    path: '/updateDatabase',
    description: '同步最新数据'
  },
];

// 暴露菜单项给父组件
defineExpose({
  menuItems
});

const handleSelect = (path: string) => {
  router.push(path);
};

const isActive = (path: string) => {
  return route.path === path;
};
</script>

<template>
  <div class="sidebar" :class="{ collapsed: isCollapse }">
    <div class="sidebar-header">
      <div class="logo" v-if="!isCollapse">
        <div class="logo-icon">L</div>
        <span class="logo-text">ITHIUM</span>
      </div>
      <div class="toggle-btn" @click="isCollapse = !isCollapse">
        <el-icon size="16">
          <Fold v-if="!isCollapse" />
          <Expand v-else />
        </el-icon>
      </div>
    </div>
    
    <nav class="sidebar-nav">
      <div 
        v-for="item in menuItems" 
        :key="item.path"
        class="nav-item"
        :class="{ active: isActive(item.path) }"
        @click="handleSelect(item.path)"
      >
        <div class="nav-icon">
          <el-icon size="18">
            <component :is="item.icon" />
          </el-icon>
        </div>
        <div class="nav-content" v-if="!isCollapse">
          <span class="nav-title">{{ item.title }}</span>
          <span class="nav-description">{{ item.description }}</span>
        </div>
        <div class="nav-indicator" v-if="isActive(item.path)"></div>
      </div>
    </nav>
  </div>
</template>

<style scoped>
.sidebar {
  width: 280px;
  min-width: 280px;
  max-width: 280px;
  background: var(--el-bg-color);
  border-right: 1px solid var(--el-border-color-light);
  display: flex;
  flex-direction: column;
  transition: width 0.3s ease, min-width 0.3s ease, max-width 0.3s ease;
  position: relative;
  box-shadow: 2px 0 8px rgba(0, 0, 0, 0.05);
  flex-shrink: 0;
  height: 100vh;
  overflow-y: auto;
  overflow-x: hidden;
}

.sidebar.collapsed {
  width: 64px;
  min-width: 64px;
  max-width: 64px;
}

.sidebar-header {
  height: 64px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 16px;
  border-bottom: 1px solid var(--el-border-color-lighter);
}

.logo {
  display: flex;
  align-items: center;
  gap: 12px;
}

.logo-icon {
  width: 32px;
  height: 32px;
  background: linear-gradient(135deg, var(--el-color-primary), var(--el-color-primary-light-3));
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-weight: 600;
  font-size: 16px;
}

.logo-text {
  font-size: 22px;
  font-weight: 600;
  color: var(--el-text-color-primary);
}

.toggle-btn {
  width: 32px;
  height: 32px;
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.2s ease;
  color: var(--el-text-color-regular);
}

.toggle-btn:hover {
  background: var(--el-fill-color-light);
  color: var(--el-color-primary);
}

.sidebar-nav {
  flex: 1;
  padding: 16px 0;
}

.nav-item {
  position: relative;
  display: flex;
  align-items: center;
  padding: 12px 16px;
  margin: 0 12px 4px 12px;
  border-radius: 10px;
  cursor: pointer;
  transition: all 0.2s ease;
  user-select: none;
}

.nav-item:hover {
  background: var(--el-fill-color-light);
}

.nav-item.active {
  background: var(--el-color-primary-light-9);
  color: var(--el-color-primary);
}

.nav-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  height: 40px;
  border-radius: 8px;
  transition: all 0.2s ease;
}

.nav-item.active .nav-icon {
  background: var(--el-color-primary-light-8);
  color: var(--el-color-primary);
}

.nav-content {
  flex: 1;
  margin-left: 12px;
  overflow: hidden;
}

.nav-title {
  display: block;
  font-size: 14px;
  font-weight: 500;
  color: var(--el-text-color-primary);
  line-height: 1.2;
}

.nav-description {
  display: block;
  font-size: 12px;
  color: var(--el-text-color-secondary);
  margin-top: 2px;
  line-height: 1.2;
}

.nav-item.active .nav-title {
  color: var(--el-color-primary);
}

.nav-indicator {
  position: absolute;
  right: 0;
  top: 50%;
  transform: translateY(-50%);
  width: 3px;
  height: 20px;
  background: var(--el-color-primary);
  border-radius: 2px 0 0 2px;
}

.collapsed .nav-item {
  justify-content: center;
  padding: 12px;
}

.collapsed .nav-content {
  display: none;
}

/* 响应式处理 */
@media (max-width: 1024px) {
  .sidebar {
    width: 240px;
    min-width: 240px;
    max-width: 240px;
  }
}

@media (max-width: 768px) {
  .sidebar {
    width: 200px;
    min-width: 200px;
    max-width: 200px;
  }
}
</style>
