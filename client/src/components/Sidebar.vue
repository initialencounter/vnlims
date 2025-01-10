<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { House, Search, Fold, Expand, Refresh } from '@element-plus/icons-vue'

const router = useRouter()
const isCollapse = ref(false)

const menuItems = [
  {
    icon: House,
    title: '首页',
    path: '/'
  },
  {
    icon: Search,
    title: '搜索技术部备注',
    path: '/searchTNotes'
  },
  {
    icon: Search,
    title: '搜索市场部备注',
    path: '/searchMNotes'
  },
  {
    icon: Search,
    title: '搜索项目名称',
    path: '/searchItemCName'
  },
  {
    icon: Refresh,
    title: '更新数据库',
    path: '/updateDatabase'
  }
]

const handleSelect = (path: string) => {
  router.push(path)
}
</script>

<template>
  <el-menu
    class="sidebar-menu"
    :collapse="isCollapse"
    :default-active="$route.path"
    @select="handleSelect"
  >
    <div class="toggle-button" @click="isCollapse = !isCollapse">
      <el-icon><Fold v-if="!isCollapse"/><Expand v-else/></el-icon>
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