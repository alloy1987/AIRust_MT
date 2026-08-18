<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { getCurrentWebview } from '@tauri-apps/api/webview'
import { message } from '@tauri-apps/plugin-dialog'
import type { Muya } from '@muyajs/core'
import TitleBar from './components/TitleBar.vue'
import TabsBar from './components/TabsBar.vue'
import SidePanel from './components/SidePanel.vue'
import EditorPane from './components/EditorPane.vue'
import SearchBar from './components/SearchBar.vue'
import StatusBar from './components/StatusBar.vue'
import UnsavedDialog from './components/UnsavedDialog.vue'
import SettingsDialog from './components/SettingsDialog.vue'
import FileChangedDialog from './components/FileChangedDialog.vue'
import { useEditorStore } from './stores/editor'
import { t, formatError, translateIoError } from './i18n'
import { api } from './api'
import licenseText from '../LICENSE?raw'
import pkg from '../package.json'

// 版本号唯一来源是 package.json，避免「关于」对话框里的版本写死过期
const APP_VERSION = pkg.version

const store = useEditorStore()
const paneRef = ref<InstanceType<typeof EditorPane> | null>(null)
const pendingCloseId = ref<string | null>(null)
const settingsOpen = ref(false)
const fileChangedId = ref<string | null>(null)
// 是否正在「退出应用」的未保存确认流程
const quitPending = ref(false)

// 用户选择「保留当前」后，对该路径短暂静默，避免同一外部变更反复弹窗
const changeCooldown = new Map<string, number>()

const activeMuya = computed<Muya | null>(() => paneRef.value?.getMuya(store.activeId ?? '') ?? null)

let unlistens: UnlistenFn[] = []

async function registerListeners() {
  unlistens.push(
    await listen<string>('menu', (event) => {
      void handleMenuAction(event.payload)
    }),
    await listen<string>('open-file', (event) => {
      void store.openPath(event.payload).catch((err) => {
        void message(formatError(err), { title: t('fileReadFailed'), kind: 'error' })
      })
    }),
    await listen<string>('file-changed', (event) => {
      onFileChanged(event.payload)
    }),
    await listen('app-close-requested', () => {
      void onQuitRequested()
    }),
  )

  const webview = getCurrentWebview()
  await webview.onDragDropEvent((event) => {
    const payload = event.payload
    if (payload.type !== 'drop') return
    void handleDrop(payload.paths)
  })
}

async function handleDrop(paths: string[]) {
  const imageRe = /\.(png|jpe?g|gif|bmp|svg|webp)$/i
  for (const p of paths) {
    if (imageRe.test(p)) {
      const tab = store.activeTab
      const muya = activeMuya.value
      if (!tab || !muya) continue
      const docDir = tab.path ? tab.path.replace(/[^\\/]+$/, '') : ''
      try {
        const bytes = await api.readBinaryFile(p)
        const ext = p.split('.').pop()!.toLowerCase()
        const relative = await api.saveImage(bytes, docDir, ext)
        await muya.pasteImage(relative)
      } catch (err) {
        void message(formatError(err), { title: t('imageSaveFailed'), kind: 'error' })
      }
    } else {
      try {
        await store.openPath(p)
      } catch (err) {
        void message(formatError(err), { title: t('fileReadFailed'), kind: 'error' })
      }
    }
  }
}

function handleMenuAction(action: string) {
  switch (action) {
    case 'm-new':
      store.newTab()
      break
    case 'm-open':
      void store.openDialogAndFile().catch((err) => {
        void message(translateIoError(err), { title: t('fileReadFailed'), kind: 'error' })
      })
      break
    case 'm-open-folder':
      void openFolder()
      break
    case 'm-save':
      void saveActive()
      break
    case 'm-save-as':
      void saveAsActive()
      break
    case 'm-settings':
      settingsOpen.value = true
      break
    case 'm-undo':
      paneRef.value?.undo()
      break
    case 'm-redo':
      paneRef.value?.redo()
      break
    case 'm-find':
      store.searchOpen = true
      break
    case 'm-select-all':
      activeMuya.value?.selectAll()
      break
    case 'm-toggle-sidebar':
      store.sidebarOpen = !store.sidebarOpen
      break
    case 'm-zoom-in':
      store.zoomIn()
      break
    case 'm-zoom-out':
      store.zoomOut()
      break
    case 'm-zoom-reset':
      store.resetZoom()
      break
    case 'm-toggle-theme':
      store.toggleTheme()
      break
    case 'm-quit':
      void onQuitRequested()
      break
    case 'about':
      void message(t('aboutDialogBody', { version: APP_VERSION }), { title: t('aboutDialogTitle') })
      break
    case 'license':
      void message(licenseText, { title: t('license') })
      break
  }
}

async function openFolder() {
  const folder = await api.openFolderDialog()
  if (folder) {
    store.currentFolder = folder
    store.sidebarOpen = true
  }
}

function onKeydown(e: KeyboardEvent) {
  if (e.isComposing) return
  const mod = e.ctrlKey || e.metaKey
  if (!mod) return
  // 用物理键位 e.code 判定：Ctrl/Shift 组合下 e.key 可能返回控制字符或大写变体导致匹配失败
  switch (e.code) {
    case 'KeyN':
      e.preventDefault()
      store.newTab()
      break
    case 'KeyO':
      e.preventDefault()
      if (e.shiftKey) void openFolder()
      else void store.openDialogAndFile()
      break
    case 'KeyS':
      e.preventDefault()
      if (e.shiftKey) void saveAsActive()
      else void saveActive()
      break
    case 'KeyF':
      e.preventDefault()
      // Ctrl+Alt+F：切换侧边栏；Ctrl+F：打开查找
      if (e.altKey) store.sidebarOpen = !store.sidebarOpen
      else store.searchOpen = true
      break
    case 'Digit0':
    case 'Numpad0':
      e.preventDefault()
      store.resetZoom()
      break
    case 'KeyZ':
      // 纯文本标签走 textarea 原生撤销栈：拦截会导致 Ctrl+Z/Ctrl+Shift+Z 完全失效
      if (store.activeTab?.plainText) break
      // 统一走 muya 历史栈（单通道），preventDefault 阻止浏览器原生 contenteditable 撤销，避免双触发
      e.preventDefault()
      if (e.shiftKey) paneRef.value?.redo()
      else paneRef.value?.undo()
      break
    case 'KeyY':
      if (store.activeTab?.plainText) break
      e.preventDefault()
      paneRef.value?.redo()
      break
  }
}

// Ctrl + 滚轮缩放：前滚（deltaY < 0）放大，后滚（deltaY > 0）缩小。
// 与 Ctrl+0 一样直接改 store.zoom，右下角滑条会同步显示。
function onWheel(e: WheelEvent) {
  if (!e.ctrlKey) return
  e.preventDefault()
  if (e.deltaY < 0) store.zoomIn()
  else if (e.deltaY > 0) store.zoomOut()
}

async function saveActive() {
  const tab = store.activeTab
  if (!tab) return
  try {
    await paneRef.value?.saveDoc(tab)
  } catch (err) {
    void message(translateIoError(err), { title: t('fileSaveFailed'), kind: 'error' })
  }
}

async function saveAsActive() {
  // 截断预览模式下另存为：新文件只包含窗口中可见的部分，保存后提示
  const wasPartial = !!paneRef.value && store.activeTab?.isFull === false
  const ok = await paneRef.value?.saveAs()
  if (ok && wasPartial) {
    void message(t('largeFilePartialCopySaved'), { title: t('saveFileAs'), kind: 'info' })
  }
}

async function closeRequested(id: string) {
  const tab = store.tabs.find((tb) => tb.id === id)
  if (!tab) return
  if (!tab.dirty) {
    paneRef.value?.destroyTab(id)
    return
  }
  pendingCloseId.value = id
}

async function saveAndClosePending() {
  const id = pendingCloseId.value
  pendingCloseId.value = null
  if (!id) return
  const tab = store.tabs.find((tb) => tb.id === id)
  if (!tab) return
  try {
    const ok = await paneRef.value?.saveDoc(tab)
    if (ok) paneRef.value?.destroyTab(id)
  } catch (err) {
    void message(translateIoError(err), { title: t('fileSaveFailed'), kind: 'error' })
  }
}

function discardAndClosePending() {
  const id = pendingCloseId.value
  pendingCloseId.value = null
  if (id) paneRef.value?.destroyTab(id)
}

function cancelPendingClose() {
  pendingCloseId.value = null
}

/** 退出应用请求：检查是否有未保存标签，有则弹确认框，否则直接退出。 */
async function onQuitRequested() {
  if (quitPending.value) return
  const dirtyTabs = store.tabs.filter((tb) => tb.dirty)
  if (dirtyTabs.length === 0) {
    await api.quitApp()
    return
  }
  quitPending.value = true
}

/** 退出确认框：保存所有未保存标签后退出。 */
async function quitAndSaveAll() {
  quitPending.value = false
  const dirtyTabs = store.tabs.filter((tb) => tb.dirty)
  for (const tab of dirtyTabs) {
    try {
      const ok = await paneRef.value?.saveDoc(tab)
      if (!ok) return // 用户取消了某个另存为对话框，中止退出
    } catch (err) {
      void message(translateIoError(err), { title: t('fileSaveFailed'), kind: 'error' })
      return // 保存失败：中止退出，避免丢改动
    }
  }
  await api.quitApp()
}

/** 退出确认框：放弃所有未保存改动后退出。 */
async function quitAndDiscard() {
  quitPending.value = false
  await api.quitApp()
}

/** 退出确认框：取消，留在应用。 */
function cancelQuit() {
  quitPending.value = false
}

function onFileChanged(path: string) {
  const tab = store.tabs.find((tb) => tb.path === path)
  if (!tab || fileChangedId.value) return
  if (Date.now() < (changeCooldown.get(path) ?? 0)) return
  fileChangedId.value = tab.id
}

async function reloadFileChanged() {
  const id = fileChangedId.value
  fileChangedId.value = null
  if (!id) return
  const tab = store.tabs.find((tb) => tb.id === id)
  if (tab?.path) changeCooldown.delete(tab.path)
  try {
    await paneRef.value?.reloadFromDisk(id)
  } catch (err) {
    void message(formatError(err), { title: t('fileReadFailed'), kind: 'error' })
  }
}

function keepFileChanged() {
  const id = fileChangedId.value
  fileChangedId.value = null
  if (!id) return
  const tab = store.tabs.find((tb) => tb.id === id)
  if (tab?.path) changeCooldown.set(tab.path, Date.now() + 2000)
}

onMounted(() => {
  void store.boot().catch((err) => console.error(err))
  void registerListeners()
  window.addEventListener('keydown', onKeydown)
  window.addEventListener('wheel', onWheel, { passive: false })
})

onBeforeUnmount(() => {
  window.removeEventListener('keydown', onKeydown)
  window.removeEventListener('wheel', onWheel)
  for (const un of unlistens) un()
})
</script>

<template>
  <div class="app-shell">
    <TitleBar @menu-action="handleMenuAction" />
    <TabsBar @close-request="closeRequested" />
    <div class="app-body">
      <SidePanel v-if="store.sidebarOpen" />
      <div class="editor-area">
        <SearchBar v-if="store.searchOpen" :muya="activeMuya" />
        <EditorPane ref="paneRef" />
      </div>
    </div>
    <StatusBar />
    <SettingsDialog v-if="settingsOpen" @close="settingsOpen = false" />
    <UnsavedDialog
      v-if="pendingCloseId"
      :title="t('unsavedTitle')"
      :message="t('unsavedMessage', { name: store.tabs.find((tb) => tb.id === pendingCloseId)?.title ?? '' })"
      @save="saveAndClosePending"
      @discard="discardAndClosePending"
      @cancel="cancelPendingClose"
    />
    <UnsavedDialog
      v-if="quitPending"
      :title="t('quitUnsavedTitle')"
      :message="t('quitUnsavedMessage')"
      @save="quitAndSaveAll"
      @discard="quitAndDiscard"
      @cancel="cancelQuit"
    />
    <FileChangedDialog
      v-if="fileChangedId"
      :title="t('fileChangedTitle')"
      :message="t('fileChangedMessage', { name: store.tabs.find((tb) => tb.id === fileChangedId)?.title ?? '' })"
      @reload="reloadFileChanged"
      @keep="keepFileChanged"
    />
  </div>
</template>

<style scoped>
.app-shell {
  display: flex;
  flex-direction: column;
  height: 100vh;
}
.app-body {
  flex: 1;
  display: flex;
  min-height: 0;
}
.editor-area {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-width: 0;
}
</style>