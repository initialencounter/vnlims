<template>
  <div class="text-processor">
    <div class="processor-container">
      <h2 class="title">文本处理器</h2>

      <!-- 脚本选择区域 -->
      <div class="script-section">
        <div class="script-header">
          <el-select
            v-model="selectedScriptId"
            placeholder="选择脚本"
            style="width: 300px"
            @change="loadScript"
          >
            <el-option
              v-for="script in scripts"
              :key="script.id"
              :label="script.name"
              :value="script.id"
            />
          </el-select>
          <div class="script-actions">
            <el-button type="primary" @click="executeScript">执行脚本</el-button>
            <el-button @click="showScriptEditor = !showScriptEditor">
              {{ showScriptEditor ? '隐藏' : '显示' }}脚本编辑器
            </el-button>
            <el-button type="success" @click="saveCustomScript">保存为新脚本</el-button>
            <el-button
              v-if="currentScript?.isCustom"
              type="danger"
              @click="deleteScript"
            >
              删除脚本
            </el-button>
          </div>
        </div>

        <!-- CodeMirror 编辑器 -->
        <div v-show="showScriptEditor" class="script-editor">
          <div class="editor-header">
            <span class="editor-label">JavaScript 脚本</span>
            <span class="editor-hint">提示: 使用 input.value 获取输入，使用 output.value 设置输出</span>
          </div>
          <div ref="editorContainer" class="codemirror-container"></div>
        </div>
      </div>

      <!-- 输入输出区域 -->
      <div class="io-section">
        <div class="io-panel">
          <div class="panel-header">
            <span>输入</span>
            <el-button size="small" @click="inputText = ''">清空</el-button>
          </div>
          <textarea
            v-model="inputText"
            class="io-textarea"
            placeholder="在此输入要处理的文本..."
          />
        </div>

        <div class="io-panel">
          <div class="panel-header">
            <span>输出</span>
            <el-button size="small" @click="copyOutput">复制</el-button>
          </div>
          <textarea
            v-model="outputText"
            class="io-textarea"
            placeholder="处理结果将显示在这里..."
            readonly
          />
        </div>
      </div>

      <!-- Console 输出区域 -->
      <div class="console-section">
        <div class="console-header">
          <span>Console 输出</span>
          <el-button size="small" @click="consoleLogs = []">清空</el-button>
        </div>
        <div class="console-output">
          <div
            v-for="(log, index) in consoleLogs"
            :key="index"
            :class="['console-log', `console-${log.type}`]"
          >
            <span class="console-time">{{ log.time }}</span>
            <span class="console-type">[{{ log.type }}]</span>
            <span class="console-message">{{ log.message }}</span>
          </div>
          <div v-if="consoleLogs.length === 0" class="console-empty">
            暂无输出
          </div>
        </div>
      </div>
    </div>

    <!-- 保存脚本对话框 -->
    <el-dialog
      v-model="showSaveDialog"
      title="保存脚本"
      width="400px"
    >
      <el-form>
        <el-form-item label="脚本名称">
          <el-input
            v-model="newScriptName"
            placeholder="请输入脚本名称"
            @keyup.enter="confirmSaveScript"
          />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showSaveDialog = false">取消</el-button>
        <el-button type="primary" @click="confirmSaveScript">确定</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script lang="ts" setup>
import { ref, onMounted, onBeforeUnmount, watch, nextTick } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { EditorView, basicSetup } from 'codemirror'
import { javascript } from '@codemirror/lang-javascript'
import { autocompletion, CompletionContext } from '@codemirror/autocomplete'
import { EditorState, Compartment } from '@codemirror/state'
import { oneDark } from '@codemirror/theme-one-dark'

interface Script {
  id: string
  name: string
  code: string
  isCustom?: boolean
}

interface ConsoleLog {
  type: 'log' | 'warn' | 'error'
  message: string
  time: string
}

// 默认脚本
const defaultScripts: Script[] = [
  {
    id: 'default-1',
    name: '编号处理脚本',
    code: `let text = input.value;
let projectNo = [];
text.split("\\n").forEach((line) => {
  if (line.startsWith('"')) {
    projectNo.push('"' + line.slice(1, 18));
  } else if (line.endsWith('"')) {
    projectNo.push(line.slice(0, 17) + '"');
  } else {
    projectNo.push(line.slice(0, 17));
  }
});
output.value = projectNo.join("\\n");
console.log('处理了 ' + projectNo.length + ' 行数据');`,
    isCustom: false
  },
  {
    id: 'default-2',
    name: '去除空行',
    code: `let text = input.value;
let lines = text.split("\\n").filter(line => line.trim() !== "");
output.value = lines.join("\\n");
console.log('移除空行后剩余 ' + lines.length + ' 行');`,
    isCustom: false
  },
  {
    id: 'default-3',
    name: '行号添加',
    code: `let text = input.value;
let lines = text.split("\\n");
let numbered = lines.map((line, index) => (index + 1) + ". " + line);
output.value = numbered.join("\\n");
console.log('为 ' + lines.length + ' 行添加了行号');`,
    isCustom: false
  },
  {
    id: 'default-4',
    name: '大小写转换',
    code: `let text = input.value;
output.value = text.toUpperCase();
console.log('已转换为大写');`,
    isCustom: false
  },
  {
    id: 'default-5',
    name: '去除重复行',
    code: `let text = input.value;
let lines = text.split("\\n");
let unique = [...new Set(lines)];
output.value = unique.join("\\n");
console.log('原始: ' + lines.length + ' 行, 去重后: ' + unique.length + ' 行');`,
    isCustom: false
  }
]

const scripts = ref<Script[]>([...defaultScripts])
const selectedScriptId = ref<string>('default-1')
const currentScript = ref<Script | null>(null)
const showScriptEditor = ref<boolean>(false)

const inputText = ref<string>('')
const outputText = ref<string>('')
const consoleLogs = ref<ConsoleLog[]>([])

const showSaveDialog = ref<boolean>(false)
const newScriptName = ref<string>('')

// CodeMirror 相关
const editorContainer = ref<HTMLElement | null>(null)
let editorView: EditorView | null = null
const themeCompartment = new Compartment()

// 自定义自动完成
const customCompletions = (context: CompletionContext) => {
  const word = context.matchBefore(/\w*/)
  if (!word || (word.from == word.to && !context.explicit)) return null

  const suggestions = [
    { label: 'input.value', type: 'variable', info: '获取输入框的文本内容' },
    { label: 'output.value', type: 'variable', info: '设置输出框的文本内容' },
    { label: 'console.log', type: 'function', info: '输出日志信息' },
    { label: 'console.warn', type: 'function', info: '输出警告信息' },
    { label: 'console.error', type: 'function', info: '输出错误信息' },
    { label: 'split', type: 'method', info: '分割字符串' },
    { label: 'join', type: 'method', info: '连接数组元素' },
    { label: 'map', type: 'method', info: '映射数组元素' },
    { label: 'filter', type: 'method', info: '过滤数组元素' },
    { label: 'forEach', type: 'method', info: '遍历数组' },
    { label: 'trim', type: 'method', info: '去除首尾空格' },
    { label: 'slice', type: 'method', info: '提取字符串片段' },
    { label: 'substring', type: 'method', info: '提取子字符串' },
    { label: 'replace', type: 'method', info: '替换字符串' },
    { label: 'toUpperCase', type: 'method', info: '转换为大写' },
    { label: 'toLowerCase', type: 'method', info: '转换为小写' },
    { label: 'startsWith', type: 'method', info: '检查字符串开头' },
    { label: 'endsWith', type: 'method', info: '检查字符串结尾' },
    { label: 'includes', type: 'method', info: '检查是否包含' },
    { label: 'match', type: 'method', info: '正则匹配' },
    { label: 'JSON.parse', type: 'function', info: '解析JSON字符串' },
    { label: 'JSON.stringify', type: 'function', info: '转换为JSON字符串' },
  ]

  return {
    from: word.from,
    options: suggestions
  }
}

// 初始化 CodeMirror
const initCodeMirror = () => {
  if (!editorContainer.value || editorView) return

  // 检测当前主题
  const isDark = document.documentElement.classList.contains('dark')

  const startState = EditorState.create({
    doc: currentScript.value?.code || '',
    extensions: [
      basicSetup,
      javascript(),
      autocompletion({
        override: [customCompletions]
      }),
      themeCompartment.of(isDark ? oneDark : []),
      EditorView.updateListener.of((update) => {
        if (update.docChanged && currentScript.value) {
          currentScript.value.code = update.state.doc.toString()
        }
      }),
      EditorView.theme({
        '&': {
          fontSize: '14px',
          height: '300px'
        },
        '.cm-scroller': {
          fontFamily: 'Consolas, Monaco, "Courier New", monospace',
          overflow: 'auto'
        }
      })
    ]
  })

  editorView = new EditorView({
    state: startState,
    parent: editorContainer.value
  })
}

// 更新编辑器内容
const updateEditorContent = (code: string) => {
  if (!editorView) return
  
  editorView.dispatch({
    changes: {
      from: 0,
      to: editorView.state.doc.length,
      insert: code
    }
  })
}

// 更新编辑器主题
const updateEditorTheme = (isDark: boolean) => {
  if (!editorView) return
  
  editorView.dispatch({
    effects: themeCompartment.reconfigure(isDark ? oneDark : [])
  })
}

// 监听主题变化
const observeThemeChange = () => {
  const observer = new MutationObserver((mutations) => {
    mutations.forEach((mutation) => {
      if (mutation.attributeName === 'class') {
        const isDark = document.documentElement.classList.contains('dark')
        updateEditorTheme(isDark)
      }
    })
  })

  observer.observe(document.documentElement, {
    attributes: true,
    attributeFilter: ['class']
  })

  return observer
}

let themeObserver: MutationObserver | null = null

// 加载保存的自定义脚本
onMounted(() => {
  loadCustomScripts()
  loadScript()
  
  nextTick(() => {
    initCodeMirror()
    themeObserver = observeThemeChange()
  })
})

onBeforeUnmount(() => {
  if (editorView) {
    editorView.destroy()
    editorView = null
  }
  if (themeObserver) {
    themeObserver.disconnect()
  }
})

// 监听编辑器显示状态
watch(showScriptEditor, async (newVal) => {
  if (newVal) {
    await nextTick()
    if (!editorView) {
      initCodeMirror()
    }
  }
})

const loadCustomScripts = () => {
  const saved = localStorage.getItem('textProcessor_customScripts')
  if (saved) {
    try {
      const customScripts = JSON.parse(saved) as Script[]
      scripts.value = [...defaultScripts, ...customScripts]
    } catch (e) {
      console.error('Failed to load custom scripts', e)
    }
  }
}

const saveCustomScriptsToStorage = () => {
  const customScripts = scripts.value.filter(s => s.isCustom)
  localStorage.setItem('textProcessor_customScripts', JSON.stringify(customScripts))
}

const loadScript = () => {
  const script = scripts.value.find(s => s.id === selectedScriptId.value)
  if (script) {
    currentScript.value = script
    if (editorView) {
      updateEditorContent(script.code)
    }
  }
}

const addConsoleLog = (type: 'log' | 'warn' | 'error', ...args: any[]) => {
  const time = new Date().toLocaleTimeString()
  const message = args.map(arg => {
    if (typeof arg === 'object') {
      try {
        return JSON.stringify(arg, null, 2)
      } catch {
        return String(arg)
      }
    }
    return String(arg)
  }).join(' ')
  
  consoleLogs.value.push({ type, message, time })
}

const executeScript = () => {
  consoleLogs.value = []
  outputText.value = ''
  
  try {
    // 获取当前代码
    const code = editorView ? editorView.state.doc.toString() : currentScript.value?.code || ''
    
    // 创建沙箱环境
    const input = { value: inputText.value }
    const output = { value: '' }
    
    // 拦截 console 方法
    const console = {
      log: (...args: any[]) => addConsoleLog('log', ...args),
      warn: (...args: any[]) => addConsoleLog('warn', ...args),
      error: (...args: any[]) => addConsoleLog('error', ...args)
    }
    
    // 执行脚本
    const func = new Function('input', 'output', 'console', code)
    func(input, output, console)
    
    outputText.value = output.value
    
    if (consoleLogs.value.length === 0) {
      addConsoleLog('log', '脚本执行成功')
    }
    
    ElMessage.success('脚本执行成功')
  } catch (error: any) {
    addConsoleLog('error', '脚本执行错误: ' + error.message)
    ElMessage.error('脚本执行失败: ' + error.message)
  }
}

const saveCustomScript = () => {
  newScriptName.value = ''
  showSaveDialog.value = true
}

const confirmSaveScript = () => {
  if (!newScriptName.value.trim()) {
    ElMessage.warning('请输入脚本名称')
    return
  }
  
  const code = editorView ? editorView.state.doc.toString() : currentScript.value?.code || ''
  
  const newScript: Script = {
    id: 'custom-' + Date.now(),
    name: newScriptName.value,
    code: code,
    isCustom: true
  }
  
  scripts.value.push(newScript)
  saveCustomScriptsToStorage()
  selectedScriptId.value = newScript.id
  loadScript()
  
  showSaveDialog.value = false
  ElMessage.success('脚本保存成功')
}

const deleteScript = () => {
  if (!currentScript.value?.isCustom) {
    ElMessage.warning('默认脚本不能删除')
    return
  }
  
  ElMessageBox.confirm(
    '确定要删除脚本 "' + currentScript.value.name + '" 吗？',
    '删除确认',
    {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    }
  ).then(() => {
    scripts.value = scripts.value.filter(s => s.id !== selectedScriptId.value)
    saveCustomScriptsToStorage()
    selectedScriptId.value = 'default-1'
    loadScript()
    ElMessage.success('脚本已删除')
  }).catch(() => {
    // 取消删除
  })
}

const copyOutput = () => {
  if (!outputText.value) {
    ElMessage.warning('输出内容为空')
    return
  }
  
  navigator.clipboard.writeText(outputText.value).then(() => {
    ElMessage.success('已复制到剪贴板')
  }).catch(() => {
    ElMessage.error('复制失败')
  })
}
</script>

<style scoped>
.text-processor {
  width: 100%;
  height: 100%;
  display: flex;
  justify-content: center;
  padding: 20px;
  box-sizing: border-box;
  overflow-y: auto;
}

.processor-container {
  background: var(--el-bg-color);
  border-radius: 12px;
  padding: 25px;
  width: 100%;
  max-width: 1400px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

.title {
  color: var(--el-text-color-primary);
  text-align: center;
  margin-bottom: 30px;
  font-size: 24px;
}

.script-section {
  margin-bottom: 20px;
}

.script-header {
  display: flex;
  gap: 12px;
  margin-bottom: 15px;
  flex-wrap: wrap;
  align-items: center;
}

.script-actions {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.script-editor {
  border: 2px solid var(--el-border-color);
  border-radius: 8px;
  overflow: hidden;
  background: var(--el-fill-color-light);
}

.editor-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 15px;
  background: var(--el-fill-color);
  border-bottom: 1px solid var(--el-border-color);
}

.editor-label {
  font-weight: 600;
  color: var(--el-text-color-primary);
}

.editor-hint {
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

.codemirror-container {
  height: 300px;
  overflow: hidden;
}

.codemirror-container :deep(.cm-editor) {
  height: 100%;
}

.codemirror-container :deep(.cm-scroller) {
  overflow: auto;
}

.io-section {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 20px;
  margin-bottom: 20px;
}

.io-panel {
  border: 2px solid var(--el-border-color);
  border-radius: 8px;
  overflow: hidden;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 15px;
  background: var(--el-fill-color);
  border-bottom: 1px solid var(--el-border-color);
  font-weight: 600;
  color: var(--el-text-color-primary);
}

.io-textarea {
  width: 100%;
  min-height: 300px;
  padding: 15px;
  border: none;
  background: var(--el-bg-color);
  color: var(--el-text-color-primary);
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-size: 14px;
  line-height: 1.6;
  resize: vertical;
  box-sizing: border-box;
}

.io-textarea:focus {
  outline: none;
}

.console-section {
  border: 2px solid var(--el-border-color);
  border-radius: 8px;
  overflow: hidden;
}

.console-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 15px;
  background: var(--el-fill-color);
  border-bottom: 1px solid var(--el-border-color);
  font-weight: 600;
  color: var(--el-text-color-primary);
}

.console-output {
  background: var(--el-bg-color);
  padding: 10px;
  max-height: 200px;
  overflow-y: auto;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-size: 13px;
}

.console-log {
  padding: 4px 8px;
  margin-bottom: 2px;
  border-left: 3px solid transparent;
}

.console-log {
  border-left-color: var(--el-color-info);
}

.console-warn {
  border-left-color: var(--el-color-warning);
  background: var(--el-color-warning-light-9);
}

.console-error {
  border-left-color: var(--el-color-danger);
  background: var(--el-color-danger-light-9);
}

.console-time {
  color: var(--el-text-color-secondary);
  margin-right: 8px;
  font-size: 11px;
}

.console-type {
  color: var(--el-text-color-regular);
  margin-right: 8px;
  font-weight: 600;
}

.console-message {
  color: var(--el-text-color-primary);
  white-space: pre-wrap;
  word-break: break-all;
}

.console-empty {
  color: var(--el-text-color-secondary);
  text-align: center;
  padding: 20px;
}

@media (max-width: 1024px) {
  .io-section {
    grid-template-columns: 1fr;
  }
  
  .script-header {
    flex-direction: column;
    align-items: stretch;
  }
  
  .script-actions {
    width: 100%;
  }
  
  .script-actions .el-button {
    flex: 1;
  }
}

@media (max-width: 768px) {
  .text-processor {
    padding: 10px;
  }

  .processor-container {
    padding: 15px;
  }

  .title {
    font-size: 20px;
  }

  .editor-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 8px;
  }
}
</style>