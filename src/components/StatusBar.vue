<script setup lang="ts">
import { computed } from 'vue'
import { useEditorStore } from '../stores/editor'
import { t } from '../i18n'
import ZoomSlider from './ZoomSlider.vue'

const store = useEditorStore()

const encodingLabel = computed(() => {
  const enc = store.activeTab?.encoding ?? ''
  return enc.toUpperCase()
})

/** 纯文本文档在左下角显示的后缀名（如 .json），取自当前标签的文件名 */
const formatLabel = computed(() => {
  const tab = store.activeTab
  if (!tab) return ''
  const src = tab.path ?? tab.title
  const name = src.split(/[\\/]/).pop() ?? src
  const dot = name.lastIndexOf('.')
  return (dot > 0 ? name.slice(dot) : name).toLowerCase()
})
</script>

<template>
  <footer class="status-bar">
    <!-- 纯文本文档：显示后缀名表示文档格式（字数统计对代码/配置无意义） -->
    <span v-if="store.activeTab && store.activeTab.plainText" class="format">{{ formatLabel }}</span>
    <span v-else-if="store.activeTab">
      {{ t('words') }}: {{ store.activeTab.words }} · {{ t('chars') }}:
      {{ store.activeTab.content.length }}
    </span>
    <span class="spacer"></span>
    <span v-if="store.activeTab && !store.activeTab.isFull" class="notice warn" :title="t('largeFileTruncated')">
      {{ t('largeFileTruncated') }}
    </span>
    <span v-if="store.activeTab && store.activeTab.lossy" class="notice warn">
      {{ t('lossySaveBlocked') }}
    </span>
    <span class="encoding" v-if="store.activeTab">{{ encodingLabel }}</span>
    <ZoomSlider />
    <span class="path" v-if="store.activeTab && store.activeTab.path">{{ store.activeTab.path }}</span>
    <span v-else-if="store.activeTab">{{ t('untitled') }}</span>
  </footer>
</template>

<style scoped>
.status-bar {
  height: 24px;
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 0 10px;
  background: var(--titlebar-bg);
  border-top: 1px solid var(--border);
  font-size: 11.5px;
  color: var(--text-tertiary);
  flex-shrink: 0;
  user-select: none;
}
.spacer {
  flex: 1;
}
.path {
  max-width: 45%;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.encoding {
  font-family: var(--mono, monospace);
  font-size: 10.5px;
  opacity: 0.85;
}
.format {
  font-family: var(--mono, monospace);
  font-size: 11px;
  color: var(--text-secondary);
}
.notice {
  max-width: 40%;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  padding: 1px 6px;
  border-radius: 8px;
}
.notice.warn {
  background: rgba(255, 165, 0, 0.16);
  color: #d9930e;
}
</style>