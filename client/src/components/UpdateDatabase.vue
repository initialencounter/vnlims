<template>
  <div class="update-container">
    <h2 class="page-title">更新数据库</h2>

    <h3>最后更新时间：{{ lastUpdatedTime }}</h3>

    <Login />

    <div class="calendar-wrapper">
      <el-calendar v-model="calendarDate">
        <template #date-cell="{ data }">
          <div
            class="calendar-day"
            :class="{
              'has-data': dateCounts[toDateStr(data.date)]?.total,
              'is-selected': toDateStr(data.date) === selectedDate,
            }"
            @click="selectDate(data.date)"
          >
            <span class="day-number">{{ data.day }}</span>
            <span class="day-count">
              <template v-if="dateCounts[toDateStr(data.date)]?.total">
                总{{ dateCounts[toDateStr(data.date)].total }}
                / 未打印<span :class="dateCounts[toDateStr(data.date)].null_report ? 'count-red' : 'count-green'">{{ dateCounts[toDateStr(data.date)].null_report }}</span>
              </template>
              <template v-else>0</template>
            </span>
          </div>
        </template>
      </el-calendar>
    </div>

    <div v-if="selectedDate" class="action-bar">
      <div class="selected-info">
        <span>已选日期：<strong>{{ selectedDate }}</strong></span>
        <span v-if="dateCounts[selectedDate]?.total" class="hint-text">
          （已有 {{ dateCounts[selectedDate].total }} 条数据，其中 <span :class="dateCounts[selectedDate].null_report ? 'count-red' : 'count-green'">{{ dateCounts[selectedDate].null_report }}</span> 条未报，重新导入将覆盖）
        </span>
        <span v-else class="hint-text empty">（暂无数据）</span>
      </div>
      <el-button
        type="primary"
        :loading="isLoading"
        @click="submitRequest"
      >
        {{ isLoading ? '更新中...' : '更新' }}
      </el-button>
    </div>

    <el-alert
      v-if="message"
      :title="message"
      :type="messageType === 'success' ? 'success' : 'error'"
      show-icon
      :closable="true"
      class="message-alert"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { isDev } from '../utils'
import axios from 'axios'
import Login from './Login.vue'

const currentTime = new Date(new Date().setHours(new Date().getHours() + 8))
const selectedDate = ref('')
const calendarDate = ref(new Date())
const isLoading = ref(false)
const message = ref('')
const messageType = ref('success')
const lastUpdatedTime = ref('')
const dateCounts = ref<Record<string, { total: number; null_report: number }>>({})

const toDateStr = (date: Date): string => {
  const year = date.getFullYear()
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  return `${year}-${month}-${day}`
}

const fetchDateCounts = async (year: number, month: number) => {
  const baseUrl = isDev() ? 'http://localhost:4000' : ''
  try {
    const res = await axios.get(`${baseUrl}/getDateCounts`, {
      params: { year, month },
    })
    dateCounts.value = res.data
  } catch {
    dateCounts.value = {}
  }
}

const ymKey = (d: Date) => `${d.getFullYear()}-${d.getMonth() + 1}`
let lastYm = ''

const loadMonth = (d: Date) => {
  const ym = ymKey(d)
  if (ym === lastYm) return
  lastYm = ym
  fetchDateCounts(d.getFullYear(), d.getMonth() + 1)
}

onMounted(async () => {
  const baseUrl = isDev() ? 'http://localhost:4000' : ''
  try {
    const lastUpdatedRes = await axios.get(`${baseUrl}/getLastUpdated`)
    lastUpdatedTime.value = lastUpdatedRes.data
  } catch {
    // 静默处理
  }
  loadMonth(calendarDate.value)
})

watch(calendarDate, loadMonth)

const selectDate = (date: Date) => {
  selectedDate.value = toDateStr(date)
}

const submitRequest = async () => {
  if (!selectedDate.value) {
    message.value = '请先在日历中选择一个日期'
    messageType.value = 'error'
    return
  }

  const dateObj = new Date(selectedDate.value)
  if (dateObj.getTime() > new Date(currentTime).getTime()) {
    message.value = '日期不能超过今天'
    messageType.value = 'error'
    return
  }

  isLoading.value = true
  message.value = ''

  try {
    const baseUrl = isDev() ? 'http://localhost:4000' : ''
    const res = await axios.get(`${baseUrl}/import`, {
      params: {
        date: selectedDate.value,
      },
    })
    message.value = res?.data
    if (message.value.startsWith('请先登录')) {
      messageType.value = 'error'
    } else {
      messageType.value = 'success'
    }
  } catch (error: any) {
    message.value = `错误: ${error.message}`
    messageType.value = 'error'
  } finally {
    isLoading.value = false
  }
}
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

.calendar-wrapper {
  margin-top: 20px;
  border-radius: 8px;
  box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.05);
  overflow: hidden;
}

/* 覆盖 Element Plus 日历格子高度 */
.calendar-wrapper :deep(.el-calendar-table td) {
  height: 68px;
  vertical-align: top;
}

.calendar-wrapper :deep(.el-calendar-table .el-calendar-day) {
  height: 100%;
  padding: 4px 6px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: flex-start;
  box-sizing: border-box;
}

.calendar-day {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  width: 100%;
  height: 100%;
  cursor: pointer;
  border-radius: 4px;
  transition: background-color 0.15s;
  padding: 2px 4px;
  box-sizing: border-box;
}

.calendar-day:hover {
  background-color: #ecf5ff;
}

.calendar-day.has-data {
  background-color: #e6f7e6;
}

.calendar-day.has-data:hover {
  background-color: #d4edd4;
}

.calendar-day.is-selected {
  background-color: #409eff !important;
}

.calendar-day.is-selected .day-number,
.calendar-day.is-selected .day-count {
  color: #fff;
}

.day-number {
  font-size: 18px;
  font-weight: 500;
  color: #303133;
  line-height: 1.5;
}

.day-count {
  font-size: 16px;
  color: #67c23a;
  line-height: 1.4;
  min-height: 15px;
}

.calendar-day.has-data .day-count {
  font-weight: 600;
}

.calendar-day.is-selected .day-count {
  color: #e6ffe6;
}

.action-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-top: 20px;
  padding: 16px 20px;
  background: #fff;
  border-radius: 8px;
  box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.05);
}

.selected-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.hint-text {
  font-size: 13px;
  color: #409eff;
}

.hint-text.empty {
  color: #909399;
}

.count-green {
  color: #67c23a;
}

.count-red {
  color: #f56c6c;
}

.message-alert {
  margin-top: 20px;
}
</style>
