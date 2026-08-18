<script setup lang="ts">
import { ref, watch } from 'vue'
import type { Muya } from '@muyajs/core'
import { useEditorStore, type EditorTab } from '../stores/editor'
import { createMuya, handleImageSource, resolveLocale } from '../editor/muya-factory'
import { api, LOSSY_CODE, LARGE_FILE_EDITED_CODE, FULL_LOAD_TOO_LARGE_CODE } from '../api'
import { formatError, t, translateIoError } from '../i18n'
import { message } from '@tauri-apps/plugin-dialog'
import InsertImageDialog from './InsertImageDialog.vue'

const store = useEditorStore()

const instances = new Map<string, Muya>()
const insertImageOpen = ref(false)
const insertImageTab = ref<EditorTab | null>(null)

// 从磁盘重载期间，暂停 onChange 的 dirty 标记，避免 setContent 触发误改
const suppressingChange = new Set<string>()

async function onInsertImage(src: string) {
  const tab = insertImageTab.value
  insertImageOpen.value = false
  insertImageTab.value = null
  if (!tab) return
  const muya = instances.get(tab.id)
  if (!muya) return
  try {
    const docDir = tab.path ? tab.path.replace(/[^\\/]+$/, '') : ''
    const relative = await handleImageSource(src, docDir)
    // 本地图片文件读取 / 转存失败：给出提示，避免无声无息
    if (!relative) {
      void message(formatError(new Error(src)), { title: t('fileReadFailed'), kind: 'error' })
      return
    }
    // 手动输入地址时，光标已离开编辑器（DOM 选区在对话框输入框里）。
    // muya.insertImage 依赖活动光标，此时会静默放弃插入；
    // 先恢复焦点以还原最后光标位置，保证图片插入到原位置。
    if (!muya.hasFocus()) muya.focus()
    muya.insertImage({ src: relative })
  } catch (err) {
    console.error('insert image', err)
  }
}

function registerHost(el: HTMLElement | null, tab: EditorTab) {
  if (!el || instances.has(tab.id)) return
  // 大文件只读预览标签：不进入 WYSIWYG 内核，直接由模板渲染只读纯文本，避免重建 block 树卡死
  // 纯文本文档标签：同样不进入 WYSIWYG，由模板渲染可编辑 textarea
  if (tab.bigFile || tab.plainText) return
  try {
    const docDir = () => (tab.path ? tab.path.replace(/[^\\/]+$/, '') : '')
    const muya = createMuya(
      el,
      tab.content,
      {
        onChange: () => {
          if (suppressingChange.has(tab.id)) return
          const md = muya.getMarkdown()
          store.setTabInfo(tab.id, {
            content: md,
            dirty: true,
            words: md.replace(/\s/g, '').length,
          })
        },
        onToc: (toc) => store.setTabInfo(tab.id, { toc }),
      },
      docDir,
      store.uiLang,
    )
    // 输入 / 的快捷插入菜单里选择了「图片」：由 EditorPane 弹出图片插入对话框
    muya.eventCenter.on('muya-insert-image', () => {
      insertImageTab.value = store.tabs.find((tb) => tb.id === tab.id) ?? null
      insertImageOpen.value = true
    })
    instances.set(tab.id, muya)
  } catch (err) {
    console.error('createMuya failed for tab', tab.id, err)
    el.textContent = '[编辑器初始化失败] ' + String(err instanceof Error ? err.message : err)
    el.style.padding = '24px'
    el.style.color = '#c00'
    el.style.font = '13px monospace'
  }
}

watch(
  () => store.uiLang,
  (lang) => {
    const locale = resolveLocale(lang)
    for (const muya of instances.values()) muya.locale(locale)
  },
)

/** 纯文本标签输入：更新缓冲并标记 dirty（字数统计不适用非 MD 文档，状态栏改为显示后缀名） */
function onPlainInput(tab: EditorTab, e: Event) {
  const value = (e.target as HTMLTextAreaElement).value
  store.setTabInfo(tab.id, { content: value, dirty: true })
}

async function saveDoc(tab: EditorTab): Promise<boolean> {
  // 可编辑纯文本标签：缓冲即全文，直接按原编码落盘（lossy 文件仍拒绝覆盖）
  if (tab.plainText && !tab.bigFile) {
    if (tab.lossy) {
      throw new Error(LOSSY_CODE)
    }
    if (!tab.path) return false
    await api.saveFile(tab.path, tab.content, tab.encoding)
    store.setTabInfo(tab.id, { dirty: false })
    return true
  }
  const muya = instances.get(tab.id)
  // 大文件只读预览标签：没有 muya 内核，无法产生编辑内容；保存时后端确保写完整文件
  if (!muya) {
    if (tab.lossy) {
      throw new Error(LOSSY_CODE)
    }
    if (!tab.path) return false
    const fullContent = await api.ensureFullContent(tab.path)
    await api.saveFile(tab.path, fullContent, tab.encoding)
    store.setTabInfo(tab.id, { isFull: true, dirty: false, lossy: false })
    return true
  }
  muya.flush()
  let content = muya.getMarkdown()
  if (!tab.path) {
    const title = tab.title.toLowerCase().endsWith('.md') ? tab.title : `${tab.title}.md`
    const saved = await api.saveFileAsDialog(content, title)
    if (!saved) return false
    store.setTabInfo(tab.id, {
      path: saved,
      title: saved.split(/[\\/]/).pop() ?? tab.title,
      encoding: 'utf-8',
      lossy: false,
      isFull: true,
      dirty: false,
    })
    return true
  }
  // 🌟 原文件解码有信息丢失时拒绝覆盖，避免损坏原文件
  if (tab.lossy) {
    throw new Error(LOSSY_CODE)
  }
  // 🌟 大文件截断模式：保存必须先拿到完整内容，防止截断落盘
  if (tab.isFull === false) {
    // 用户已在截断缓冲上编辑：直接覆盖会丢文件尾部，拒绝
    if (tab.dirty) {
      throw new Error(LARGE_FILE_EDITED_CODE)
    }
    const fullContent = await api.ensureFullContent(tab.path)
    suppressingChange.add(tab.id)
    muya.setContent(fullContent, false)
    setTimeout(() => suppressingChange.delete(tab.id), 150)
    content = fullContent
    store.setTabInfo(tab.id, {
      content: fullContent,
      isFull: true,
      words: fullContent.replace(/\s/g, '').length,
    })
  }
  await api.saveFile(tab.path, content, tab.encoding)
  store.setTabInfo(tab.id, { dirty: false })
  return true
}

/** 完整加载大文件（只读预览标签）：手动触发「加载完整内容」按钮。
 *  加载后仍保持只读纯文本视图（不进入 WYSIWYG），仅更新预览缓冲内容。 */
async function loadBigFull(tab: EditorTab) {
  if (tab.isFull || !tab.path) return
  try {
    const fullContent = await api.ensureFullContent(tab.path)
    const current = store.tabs.find((t) => t.id === tab.id)
    if (!current) return
    store.setTabInfo(tab.id, {
      content: fullContent,
      isFull: true,
      words: fullContent.replace(/\s/g, '').length,
    })
  } catch (err) {
    const current = store.tabs.find((t) => t.id === tab.id)
    const code = formatError(err)
    if (current && code === LOSSY_CODE) {
      // 全文解码有信息丢失：标记阻止覆盖保存
      store.setTabInfo(current.id, { lossy: true })
    } else if (code === FULL_LOAD_TOO_LARGE_CODE) {
      // 超过完整加载上限（100 MB）：明确告知用户，而不是静默失败
      void message(translateIoError(err), { title: t('fileReadFailed'), kind: 'error' })
    } else {
      console.error('加载完整内容失败', err)
    }
  }
}

function destroyTab(id: string) {
  const muya = instances.get(id)
  if (muya) {
    muya.destroy()
    instances.delete(id)
  }
  store.forceClose(id)
}

async function reloadFromDisk(id: string): Promise<boolean> {
  const tab = store.tabs.find((tb) => tb.id === id)
  if (!tab?.path) return false
  // 大文件只读预览 / 纯文本文档标签：直接刷新纯文本缓冲，不涉及 muya
  if (tab.bigFile || tab.plainText) {
    const payload = await api.readFile(tab.path)
    store.setTabInfo(id, {
      content: payload.content,
      title: payload.name,
      encoding: payload.encoding,
      lossy: payload.lossy,
      isFull: payload.isFull,
      dirty: false,
      words: payload.content.replace(/\s/g, '').length,
    })
    return true
  }
  const muya = instances.get(id)
  if (!muya) return false
  const payload = await api.readFile(tab.path)
  suppressingChange.add(id)
  muya.setContent(payload.content, false)
  setTimeout(() => suppressingChange.delete(id), 150)
  store.setTabInfo(id, {
    content: payload.content,
    title: payload.name,
    encoding: payload.encoding,
    lossy: payload.lossy,
    isFull: payload.isFull,
    dirty: false,
    words: payload.content.replace(/\s/g, '').length,
  })
  return true
}

defineExpose({
  saveDoc,
  loadBigFull,
  async saveAs() {
    const tab = store.activeTab
    if (!tab) return false
    // 大文件只读预览标签：另存为当前已加载内容（未加载满时提示截断副本）
    const muya = instances.get(tab.id)
    if (!muya) {
      const saved = await api.saveFileAsDialog(tab.content, tab.title)
      if (!saved) return false
      store.setTabInfo(tab.id, {
        path: saved,
        title: saved.split(/[\\/]/).pop() ?? tab.title,
        encoding: 'utf-8',
        lossy: false,
        isFull: true,
        dirty: false,
      })
      return true
    }
    muya.flush()
    const saved = await api.saveFileAsDialog(muya.getMarkdown(), tab.title)
    if (!saved) return false
    store.setTabInfo(tab.id, {
      path: saved,
      title: saved.split(/[\\/]/).pop() ?? tab.title,
      encoding: 'utf-8',
      lossy: false,
      isFull: true,
      dirty: false,
    })
    return true
  },
  undo() {
    instances.get(store.activeId ?? '')?.undo()
  },
  redo() {
    instances.get(store.activeId ?? '')?.redo()
  },
  flush() {
    instances.get(store.activeId ?? '')?.flush()
  },
  getMuya(id: string) {
    return instances.get(id) ?? null
  },
  destroyTab,
  reloadFromDisk,
})
</script>

<template>
  <div class="panes">
    <div
      v-for="tab in store.tabs"
      :key="tab.id"
      class="editor-host"
      :data-tab="tab.id"
      :style="{ zoom: String(store.zoom) }"
      v-show="tab.id === store.activeId"
    >
      <!-- 大文件只读纯文本预览：不进入 WYSIWYG，秒开、可滚动、不卡死 -->
      <div v-if="tab.bigFile" class="big-preview">
        <div v-if="!tab.isFull" class="big-notice">
          <span>{{ t('largeFileTruncated') }}</span>
          <button class="big-load-btn" @click="loadBigFull(tab)">{{ t('largeFileLoadFull') }}</button>
        </div>
        <textarea
          readonly
          class="big-textarea"
          :value="tab.content"
          spellcheck="false"
        ></textarea>
      </div>
      <!-- 纯文本文档（非 MD 文本格式）：原文加载、可编辑、不做任何解析 -->
      <div v-else-if="tab.plainText" class="plain-preview">
        <textarea
          class="plain-textarea"
          :value="tab.content"
          spellcheck="false"
          wrap="off"
          @input="onPlainInput(tab, $event)"
        ></textarea>
      </div>
      <div v-else :ref="(el: any) => registerHost(el, tab)" class="mu-host"></div>
    </div>
  </div>
  <InsertImageDialog
    v-if="insertImageOpen"
    @insert="onInsertImage"
    @cancel="insertImageOpen = false"
  />
</template>

<style scoped>
.panes {
  position: relative;
  flex: 1;
  overflow: hidden;
}
.editor-host {
  position: absolute;
  inset: 0;
  background: var(--editor-bg);
}
.mu-host {
  width: 100%;
  height: 100%;
  overflow: auto;
}
.big-preview {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  position: relative;
}
.big-notice {
  display: flex;
  align-items: center;
  gap: 10px;
  flex-shrink: 0;
  padding: 6px 12px;
  background: rgba(255, 165, 0, 0.14);
  color: #d9930e;
  font-size: 12px;
  border-bottom: 1px solid var(--border);
}
.big-load-btn {
  padding: 3px 12px;
  border: 1px solid currentColor;
  border-radius: 14px;
  background: transparent;
  color: inherit;
  font-size: 12px;
  cursor: pointer;
  white-space: nowrap;
}
.big-load-btn:hover {
  background: rgba(255, 165, 0, 0.18);
}
.big-textarea {
  flex: 1;
  width: 100%;
  border: none;
  outline: none;
  resize: none;
  background: var(--editor-bg);
  color: var(--text-primary);
  font-family: var(--mono, ui-monospace, Consolas, monospace);
  font-size: 13px;
  line-height: 1.6;
  padding: 12px 16px;
  box-sizing: border-box;
  overflow: auto;
}
.plain-preview {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
}
.plain-textarea {
  flex: 1;
  width: 100%;
  border: none;
  outline: none;
  resize: none;
  background: var(--editor-bg);
  color: var(--text-primary);
  font-family: var(--mono, ui-monospace, Consolas, monospace);
  font-size: 13px;
  line-height: 1.6;
  padding: 12px 16px;
  box-sizing: border-box;
  overflow: auto;
  tab-size: 4;
}
</style>