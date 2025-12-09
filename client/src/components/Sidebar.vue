<script setup lang="ts">
import { ref } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { House, Search, Fold, Expand, Refresh, Document, Box, Edit } from '@element-plus/icons-vue';

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
  {
    index: '3',
    path: '/stack',
    title: '堆码计算',
    icon: Box,
    requiresUnlock: false,
  },
  {
    path: '/textProcessor',
    title: '文本处理器',
    icon: Edit,
    description: '批量处理文本数据'
  },
];
const handleSelect = (path: string) => {
  router.push(path);
};
</script>

<template>
  <el-menu
    class="sidebar-menu"
    :collapse="isCollapse"
    :default-active="$route.path"
    @select="handleSelect"
  >
    <div class="toggle-button" @click="isCollapse = !isCollapse">
      <el-icon><Fold v-if="!isCollapse" /><Expand v-else /></el-icon>
    </div>
    <el-menu-item v-for="item in menuItems" :key="item.path" :index="item.path">
      <el-icon>
        <component :is="item.icon" />
      </el-icon>
      <span>{{ item.title }}</span>
    </el-menu-item>
  </el-menu>
</template>

<style scoped>
.sidebar-menu {
  height: 100vh;
  border-right: none;
}

.sidebar-menu:not(.el-menu--collapse) {
  width: 200px;
}

.toggle-button {
  height: 50px;
  line-height: 50px;
  text-align: center;
  cursor: pointer;
  border-bottom: 1px solid var(--el-border-color);
}

.toggle-button:hover {
  background-color: var(--el-menu-hover-bg-color);
}

.sidebar {
  width: 250px;
  flex-shrink: 0;
}
</style>
