<script setup lang="ts">
import { ref, watch } from 'vue'
import { useEditorStore } from '../stores/editor'
import { api } from '../api'
import { t, translateIoError } from '../i18n'
import { message } from '@tauri-apps/plugin-dialog'
import { scrollToHeading } from '../editor/muya-factory'
import FileTreeItem from './FileTreeItem.vue'
import { toNode, type TreeNode } from './tree-node'

const store = useEditorStore()

const rootNodes = ref<TreeNode[] | null>(null)
const treeError = ref('')
const loadingTree = ref(false)
const selectedPath = ref<string | null>(null)

watch(
  () => store.currentFolder,
  async (folder) => {
    if (!folder) {
      rootNodes.value = null
      return
    }
    loadingTree.value = true
    treeError.value = ''
    try {
      const entries = await api.listDir(folder, 1)
      rootNodes.value = entries.map(toNode)
    } catch (err) {
      treeError.value = String(err)
    } finally {
      loadingTree.value = false
    }
  },
)

async function pickFolder() {
  const folder = await api.openFolderDialog()
  if (folder) store.currentFolder = folder
}

async function openEntry(entry: { path: string }) {
  selectedPath.value = entry.path
  try {
    await store.openPath(entry.path)
  } catch (err) {
    // 二进制/不可读文件等：弹框告知（后端错误码由 translateIoError 翻译为当前语言）
    void message(translateIoError(err), { title: t('fileReadFailed'), kind: 'error' })
  }
}

function jumpHeading(level: number, ordinal: number) {
  const host = document.querySelector(`.editor-host[data-tab="${store.activeId}"]`) as HTMLElement | null
  if (!host) return
  // 大文件只读标签没有 muya/大纲，忽略
  if (store.activeTab?.bigFile) return
  scrollToHeading(host, level, ordinal)
}

/** 计算某个 toc 条目在其同级别标题中的出现序号（从 0 起），供 scrollToHeading 定位。 */
function ordinalInLevel(target: { lvl: number }): number {
  const toc = store.activeTab?.toc ?? []
  let ordinal = 0
  for (const item of toc) {
    if (item === target) return ordinal
    if (item.lvl === target.lvl) ordinal += 1
  }
  return 0
}
</script>

<template>
  <aside class="side-panel">
    <div class="panel-header">
      <span class="panel-title">{{ t('files') }}</span>
      <button class="folder-btn" :title="t('openFolder')" @click="pickFolder">
        <svg width="13" height="13" viewBox="0 0 16 16"><path d="M1.5 3.5h4l1.5 2h7.5v7.5h-13z" fill="none" stroke="currentColor" stroke-width="1.3"/></svg>
      </button>
    </div>

    <div class="tree">
      <template v-if="store.currentFolder">
        <div class="tree-root">{{ store.currentFolder }}</div>
        <div v-if="loadingTree" class="tree-empty">加载中…</div>
        <div v-else-if="treeError" class="tree-empty">{{ treeError }}</div>
        <template v-else>
          <div v-if="rootNodes && rootNodes.length === 0" class="tree-empty">（空文件夹）</div>
          <FileTreeItem
            v-for="node in rootNodes ?? []"
            :key="node.key"
            :node="node"
            :depth="0"
            :selected-path="selectedPath"
            @open="openEntry"
          />
        </template>
      </template>
      <div v-else class="tree-empty">{{ t('openFolder') }}</div>
    </div>

    <div class="panel-header panel-header-outline">
      <span class="panel-title">{{ t('outline') }}</span>
    </div>
    <div class="tree outline">
      <template v-if="store.activeTab && store.activeTab.toc.length">
        <div
          v-for="(item, idx) in store.activeTab.toc"
          :key="idx"
          class="outline-item"
          :style="{ paddingLeft: `${(item.lvl - 1) * 14 + 8}px` }"
          :class="`lv-${item.lvl}`"
          @click="jumpHeading(item.lvl, ordinalInLevel(item))"
        >
          {{ item.content }}
        </div>
      </template>
      <div v-else class="tree-empty">{{ t('outline') }}</div>
    </div>
  </aside>
</template>

<style scoped>
.side-panel {
  width: 230px;
  min-width: 180px;
  background: var(--panel-bg);
  border-right: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
  overflow: hidden;
}
.panel-header {
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 10px;
  flex-shrink: 0;
}
.panel-header-outline {
  border-top: 1px solid var(--border);
  margin-top: auto;
}
.panel-title {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-tertiary);
}
.folder-btn {
  border: none;
  background: transparent;
  color: var(--text-secondary);
  display: flex;
  padding: 3px;
  border-radius: 4px;
}
.folder-btn:hover {
  background: var(--hover-bg);
}
.tree {
  flex: 1;
  overflow-y: auto;
  padding: 2px 4px 8px;
}
.tree-root {
  font-size: 11px;
  color: var(--text-tertiary);
  padding: 4px 8px 6px;
  word-break: break-all;
}
.tree-empty {
  padding: 8px;
  font-size: 12px;
  color: var(--text-tertiary);
}
.outline-item {
  font-size: 12.5px;
  color: var(--text-secondary);
  padding: 2px 0;
  cursor: default;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.outline-item:hover {
  color: var(--accent);
}
.lv-1 { font-weight: 600; }
</style>