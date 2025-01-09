<template>
  <div class="table-container">
    <el-table 
      :data="tableData" 
      style="width: 100%"
      stripe 
      border
      :max-height="800"
    >
      <el-table-column prop="projectNo" label="项目编号" width="170" fixed="left" />
      <el-table-column prop="reportNo" label="报告编号" width="150" />
      <el-table-column prop="itemCName" label="项目名称" min-width="150" />
      <el-table-column prop="assigneeName" label="提交人" width="70" />
      <el-table-column prop="principalName" label="委托方" width="100" />
      <el-table-column prop="appraiserName" label="主检员" width="70" />
      <el-table-column prop="auditorName" label="审核员" width="70" />
      <el-table-column prop="displayStatus" label="状态" width="90" />
      <el-table-column prop="submitDate" label="提交日期" width="120" />
      <el-table-column prop="tnotes" label="技术部备注" min-width="150" />
      <el-table-column prop="mnotes" label="市场部备注" min-width="200">
        <template #default="scope">
          {{ formatDate(scope.row.submitDate) }}
        </template>
      </el-table-column>
      <el-table-column label="操作" fixed="right" width="120">
        <template #default="scope">
          <el-button
            size="small"
            type="primary"
            @click="handleView(scope.row)"
          >
            复制编号
          </el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>

<script setup lang="ts">
import type { DataModel } from '../types'
import { ElMessage } from 'element-plus'

defineProps<{
  tableData: DataModel[]
}>()

// 格式化日期
const formatDate = (dateStr: string) => {
  if (!dateStr) return ''
  const date = new Date(dateStr)
  return date.toLocaleDateString('zh-CN')
}

// 查看详情
const handleView = (row: DataModel) => {
  navigator.clipboard.writeText(row.projectNo)
  ElMessage.success('复制成功')
}

</script>

<style scoped>
.table-container {
  padding: 16px 24px;
  overflow-x: auto;
}

:deep(.el-table) {
  --el-table-border-color: var(--el-border-color-lighter);
  --el-table-header-bg-color: var(--el-fill-color-light);
  margin: 0 auto;
}

/* 优化表格在小屏幕上的显示 */
@media screen and (max-width: 1400px) {
  .table-container {
    padding: 12px;
  }
}
</style>
