<script setup lang="ts">
import { computed } from 'vue'
import { api, type TreeEntry } from '../api'
import { toNode, type TreeNode } from './tree-node'
import fileIcons from '@marktext/file-icons'
import '@marktext/file-icons/build/index.css'

// Octicons 官方文件夹图标路径（VS Code 同款）
const DIR_CLOSED =
  'M0 2.75C0 1.784.784 1 1.75 1H5c.55 0 1.07.26 1.4.7l.9 1.2a.25.25 0 0 0 .2.1h6.75c.966 0 1.75.784 1.75 1.75v8.5A1.75 1.75 0 0 1 14.25 15H1.75A1.75 1.75 0 0 1 0 13.25Zm1.75-.25a.25.25 0 0 0-.25.25v10.5c0 .138.112.25.25.25h12.5a.25.25 0 0 0 .25-.25v-8.5a.25.25 0 0 0-.25-.25H7.5c-.55 0-1.07-.26-1.4-.7l-.9-1.2a.25.25 0 0 0-.2-.1Z'
const DIR_OPEN =
  'M.513 1.513A1.75 1.75 0 0 1 1.75 1h3.5c.55 0 1.07.26 1.4.7l.9 1.2a.25.25 0 0 0 .2.1H13a1 1 0 0 1 1 1v.5H2.75a.75.75 0 0 0 0 1.5h11.978a1 1 0 0 1 .994 1.117L15 13.25A1.75 1.75 0 0 1 13.25 15H1.75A1.75 1.75 0 0 1 0 13.25V2.75c0-.464.184-.91.513-1.237Z'

const props = defineProps<{
  node: TreeNode
  depth: number
  selectedPath: string | null
}>()

const emit = defineEmits<{
  (e: 'open', entry: TreeEntry): void
}>()

/** 文件图标：优先按扩展名匹配 @marktext/file-icons，保证同后缀文件图标一致
 * （库内对 README 等文件名有特殊图标，按扩展名匹配可避免 README.md 与 log.md 图标不一致）；
 * 无扩展名的文件（LICENSE、Makefile 等）退回按完整文件名匹配；未匹配时用通用文本文件图标 */
const fileIconClass = computed(() => {
  const { name } = props.node.entry
  const dot = name.lastIndexOf('.')
  const ext = dot > 0 ? name.slice(dot) : null
  const icon = (ext && fileIcons.matchName(ext, false)) || fileIcons.matchName(name, false)
  if (icon && icon.icon) {
    const colours = Array.isArray(icon.colour) ? icon.colour.filter(Boolean).join(' ') : ''
    return ['icon', icon.icon, colours].filter(Boolean).join(' ')
  }
  return 'icon icon-file-text'
})

const dirPath = computed(() => (props.node.expanded ? DIR_OPEN : DIR_CLOSED))

async function onToggle() {
  const node = props.node
  if (!node.entry.isDir) {
    emit('open', node.entry)
    return
  }
  node.expanded = !node.expanded
  if (!node.expanded || node.children) return
  node.loading = true
  try {
    const entries = await api.listDir(node.entry.path, 1)
    node.children = entries.map(toNode)
  } catch (err) {
    console.error('list_dir 失败', node.entry.path, err)
  } finally {
    node.loading = false
  }
}
</script>

<template>
  <div class="tree-branch">
    <div
      class="tree-entry"
      :class="{ selected: selectedPath === node.entry.path }"
      :style="{ paddingLeft: `${depth * 14 + 8}px` }"
      @click="onToggle"
    >
      <span class="tree-caret">{{ node.entry.isDir ? (node.expanded ? '▾' : '▸') : '' }}</span>
      <span v-if="node.entry.isDir" class="tree-icon" :class="{ 'dir-open': node.expanded }">
        <svg width="16" height="16" viewBox="0 0 16 16" class="dir-svg">
          <path :d="dirPath" fill="currentColor" />
        </svg>
      </span>
      <span v-else :class="['tree-icon', fileIconClass]"></span>
      <span class="tree-name">{{ node.entry.name }}</span>
      <span v-if="node.loading" class="tree-loading">…</span>
    </div>
    <template v-if="node.expanded && node.children">
      <div v-if="node.children.length === 0" class="tree-empty" :style="{ paddingLeft: `${(depth + 1) * 14 + 20}px` }">
        （空文件夹）
      </div>
      <FileTreeItem
        v-for="child in node.children"
        :key="child.key"
        :node="child"
        :depth="depth + 1"
        :selected-path="selectedPath"
        @open="(e) => emit('open', e)"
      />
    </template>
  </div>
</template>

<style scoped>
.tree-entry {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 3px 8px;
  border-radius: 4px;
  font-size: 12.5px;
  color: var(--text-secondary);
  cursor: default;
  white-space: nowrap;
}
.tree-entry:hover {
  background: var(--hover-bg);
}
.tree-entry.selected {
  background: var(--accent-soft);
  color: var(--text-primary);
}
.tree-caret {
  font-size: 9px;
  width: 10px;
  flex-shrink: 0;
  color: var(--text-tertiary);
}
.tree-icon {
  width: 18px;
  flex-shrink: 0;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  line-height: 1;
}
.dir-svg {
  display: block;
  color: #dcb67a;
}
.tree-icon.dir-open .dir-svg {
  color: #e8a33d;
}
.tree-name {
  overflow: hidden;
  text-overflow: ellipsis;
}
.tree-loading {
  font-size: 10px;
  color: var(--text-tertiary);
}
.tree-empty {
  padding: 3px 8px;
  font-size: 12px;
  color: var(--text-tertiary);
}
</style>