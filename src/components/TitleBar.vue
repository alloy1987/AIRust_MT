<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'
import { t } from '../i18n'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { useEditorStore } from '../stores/editor'

const win = getCurrentWindow()
const store = useEditorStore()

const emit = defineEmits<{
  (e: 'menu-action', action: string): void
}>()

const openMenu = ref<string | null>(null)

interface MenuItem {
  action: string
  label: string
  shortcut?: string
  separator?: boolean
}

const menus = computed<{ key: string; label: string; items: MenuItem[] }[]>(() => [
  {
    key: 'file',
    label: t('menuFile'),
    items: [
      { action: 'm-new', label: t('newFile'), shortcut: 'Ctrl+N' },
      { action: 'm-open', label: t('openFile'), shortcut: 'Ctrl+O' },
      { action: 'm-open-folder', label: t('openFolder'), shortcut: 'Ctrl+Shift+O' },
      { action: 'sep-1', label: '', separator: true },
      { action: 'm-save', label: t('saveFile'), shortcut: 'Ctrl+S' },
      { action: 'm-save-as', label: t('saveFileAs'), shortcut: 'Ctrl+Shift+S' },
      { action: 'sep-2', label: '', separator: true },
      { action: 'm-settings', label: t('options') },
      { action: 'sep-3', label: '', separator: true },
      { action: 'm-quit', label: t('quit'), shortcut: 'Alt+F4' },
    ],
  },
  {
    key: 'edit',
    label: t('menuEdit'),
    items: [
      { action: 'm-undo', label: t('undo'), shortcut: 'Ctrl+Z' },
      { action: 'm-redo', label: t('redo'), shortcut: 'Ctrl+Y' },
      { action: 'sep-1', label: '', separator: true },
      { action: 'm-find', label: t('find'), shortcut: 'Ctrl+F' },
      { action: 'm-select-all', label: t('selectAll'), shortcut: 'Ctrl+A' },
    ],
  },
  {
    key: 'view',
    label: t('menuView'),
    items: [
      { action: 'm-toggle-sidebar', label: t('toggleSidebar'), shortcut: 'Ctrl+Alt+F' },
      { action: 'sep-1', label: '', separator: true },
      { action: 'm-zoom-in', label: t('zoomIn'), shortcut: t('zoomWheelIn') },
      { action: 'm-zoom-out', label: t('zoomOut'), shortcut: t('zoomWheelOut') },
      { action: 'm-zoom-reset', label: t('zoomReset'), shortcut: 'Ctrl+0' },
    ],
  },
  {
    key: 'help',
    label: t('menuHelp'),
    items: [
      { action: 'about', label: t('about') },
      { action: 'license', label: t('license') },
    ],
  },
])

function isDisabled(item: MenuItem): boolean {
  switch (item.action) {
    case 'm-save':
    case 'm-save-as':
    case 'm-undo':
    case 'm-redo':
    case 'm-select-all':
      return !store.activeTab
    default:
      return false
  }
}

function onItemClick(action: string) {
  openMenu.value = null
  if (action === 'm-quit') {
    // 统一走关闭流程：后端拦截 → 前端检查未保存标签
    void win.close()
    return
  }
  emit('menu-action', action)
}

function onClickOutside() {
  openMenu.value = null
}

async function toggleMaximize() {
  if (await win.isMaximized()) {
    await win.unmaximize()
  } else {
    await win.maximize()
  }
}

onMounted(() => document.addEventListener('click', onClickOutside))
onBeforeUnmount(() => document.removeEventListener('click', onClickOutside))
</script>

<template>
  <header class="title-bar">
    <div class="brand" data-tauri-drag-region>
      <img class="brand-icon" src="/app-icon.svg" alt="" draggable="false" />
      <span class="brand-name">{{ t('appTitle') }}</span>
    </div>

    <nav class="menu-bar">
      <div v-for="menu in menus" :key="menu.key" class="menu-item-wrap">
        <button
          class="menu-item"
          :class="{ open: openMenu === menu.key }"
          @click.stop="openMenu = openMenu === menu.key ? null : menu.key"
        >
          {{ menu.label }}
        </button>
        <div v-if="openMenu === menu.key" class="menu-dropdown">
          <template v-for="item in menu.items" :key="item.action">
            <div v-if="item.separator" class="menu-sep"></div>
            <button
              v-else
              class="menu-entry"
              :disabled="isDisabled(item)"
              @click.stop="onItemClick(item.action)"
            >
              <span class="menu-entry-label">{{ item.label }}</span>
              <span v-if="item.shortcut" class="menu-entry-shortcut">{{ item.shortcut }}</span>
            </button>
          </template>
        </div>
      </div>
    </nav>

    <div class="titlebar-space" data-tauri-drag-region></div>

    <div class="window-controls">
      <button class="wc-btn" :title="t('minimize')" @click="win.minimize()">
        <svg width="10" height="10" viewBox="0 0 10 10"><rect x="0.5" y="4.5" width="9" height="1" fill="currentColor"/></svg>
      </button>
      <button class="wc-btn" :title="t('maximize')" @click="toggleMaximize">
        <svg width="10" height="10" viewBox="0 0 10 10"><rect x="0.5" y="0.5" width="9" height="9" fill="none" stroke="currentColor"/></svg>
      </button>
      <button class="wc-btn wc-close" :title="t('close')" @click="win.close()">
        <svg width="10" height="10" viewBox="0 0 10 10"><path d="M0.5 0.5 L9.5 9.5 M9.5 0.5 L0.5 9.5" stroke="currentColor" stroke-width="1.2"/></svg>
      </button>
    </div>
  </header>
</template>

<style scoped>
.title-bar {
  height: 36px;
  display: flex;
  align-items: center;
  background: var(--titlebar-bg);
  border-bottom: 1px solid var(--border);
  user-select: none;
  flex-shrink: 0;
  position: relative;
  z-index: 100;
}
.brand {
  display: flex;
  align-items: center;
  gap: 8px;
  padding-left: 12px;
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary);
  flex-shrink: 0;
}
.brand-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background: var(--accent);
}
.brand-icon {
  width: 18px;
  height: 18px;
  border-radius: 4px;
  flex-shrink: 0;
}
.menu-bar {
  display: flex;
  align-items: center;
  margin-left: 18px;
  height: 100%;
}
.titlebar-space {
  flex: 1;
  height: 100%;
}
.menu-item-wrap {
  position: relative;
  height: 100%;
  display: flex;
  align-items: center;
}
.menu-item {
  height: 26px;
  padding: 0 10px;
  border: none;
  border-radius: 5px;
  background: transparent;
  color: var(--text-secondary);
  font-size: 12.5px;
  cursor: default;
}
.menu-item:hover,
.menu-item.open {
  background: var(--hover-bg);
  color: var(--text-primary);
}
.menu-dropdown {
  position: absolute;
  top: calc(100% - 2px);
  left: 0;
  min-width: 200px;
  background: var(--float-bg-color, var(--panel-bg));
  border: 1px solid var(--float-border-color, var(--border));
  border-radius: 8px;
  box-shadow: 0 6px 20px rgb(0 0 0 / 18%);
  padding: 5px;
  display: flex;
  flex-direction: column;
}
.menu-entry {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 24px;
  height: 28px;
  padding: 0 9px;
  border: none;
  border-radius: 5px;
  background: transparent;
  color: var(--text-secondary);
  font-size: 12.5px;
  cursor: default;
  text-align: left;
}
.menu-entry:hover:not(:disabled) {
  background: var(--accent);
  color: #fff;
}
.menu-entry:disabled {
  opacity: 0.4;
  cursor: default;
}
.menu-entry-shortcut {
  font-size: 11px;
  opacity: 0.7;
  margin-left: auto;
}
.menu-sep {
  height: 1px;
  background: var(--float-border-color, var(--border));
  margin: 4px 8px;
}
.window-controls {
  display: flex;
  height: 100%;
  margin-left: auto;
}
.wc-btn {
  width: 46px;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: default;
}
.wc-btn:hover {
  background: var(--hover-bg);
  color: var(--text-primary);
}
.wc-close:hover {
  background: #e81123;
  color: #fff;
}
</style>
