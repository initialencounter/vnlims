<template>
  <div class="table-container">
    <el-table
      :data="tableData"
      style="width: 100%"
      stripe
      border
      :max-height="800"
      :row-class-name="setRowClass"
    >
      <el-table-column
        prop="projectNo"
        label="项目编号"
        width="170"
        fixed="left"
      />
      <el-table-column prop="reportNo" label="报告编号" width="150" />
      <el-table-column prop="itemCName" label="项目名称" min-width="150" />
      <el-table-column prop="assigneeName" label="提交人" width="70" />
      <el-table-column prop="principalName" label="委托方" width="100" />
      <el-table-column prop="appraiserName" label="主检员" width="70" />
      <el-table-column prop="auditorName" label="审核员" width="70" />
      <el-table-column prop="displayStatus" label="状态" width="90" />
      <el-table-column prop="submitDate" label="提交日期" width="120" />
      <el-table-column prop="tnotes" label="技术部备注" min-width="150" />
      <el-table-column prop="mnotes" label="市场部备注" min-width="200" />
      <el-table-column label="操作" fixed="right" width="120">
        <template #default="scope">
          <el-button size="small" type="primary" @click="handleView(scope.row)">
            复制编号
          </el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>

<script setup lang="ts">
import type { DataModel } from '../types';
import { ElMessage } from 'element-plus';

defineProps<{
  tableData: DataModel[];
}>();

// 设置行的类名
const setRowClass = (row: {row:DataModel}) => {
  let systemId = row.row?.projectNo?.slice(0, 3); // 获取系统ID
  switch (systemId) {
    case 'PEK':
      return 'row-green';
    case 'SEK':
      return 'row-blue';
    case 'AEK':
      return 'row-purple';
    case 'REK':
      return 'row-red';
    default:
      return 'row-withe';
  }
};

// 查看详情
const handleView = (row: DataModel) => {
  // 使用隐藏的输入框进行复制
  const input = document.createElement('input');
  input.value = row.projectNo;
  document.body.appendChild(input);
  input.select();
  document.execCommand('copy');
  document.body.removeChild(input);
  ElMessage.success('复制成功');
};
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

:deep(.row-green) {
  color: #51a020 !important;
}

:deep(.row-blue) {
  color: #3e8ed0 !important;
}

:deep(.row-purple) {
  color: #8C1AF6 !important;
}

:deep(.row-red) {
  color: #EA3323 !important;
}

/* 优化表格在小屏幕上的显示 */
@media screen and (max-width: 1400px) {
  .table-container {
    padding: 12px;
  }
}
</style>
