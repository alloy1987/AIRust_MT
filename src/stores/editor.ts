import { defineStore } from 'pinia'
import type { ITocItem } from '@muyajs/core'
import { message } from '@tauri-apps/plugin-dialog'
import { api } from '../api'
import { setUiLang, t, translateIoError, UI_LANGS, type LangCode } from '../i18n'

// 各语言下「未命名」的历史快照，切换界面语言时同步翻译存量 tab 标题
const UNTITLED_SNAPSHOTS = ['未命名', 'Untitled', '無題', 'Без имени', '제목 없음', 'Sin título', 'Sans titre']

export interface EditorTab {
  id: string
  path: string | null
  title: string
  encoding: string
  /** 原文件解码是否有信息丢失（有则拒绝覆盖保存） */
  lossy: boolean
  /** 是否已加载完整内容（大文件打开时为 false，保存前需 ensureFullContent） */
  isFull: boolean
  /** 是否为大文件快速预览标签（>5MB 截断预览，使用只读纯文本视图，不进入 WYSIWYG） */
  bigFile: boolean
  /** 是否为纯文本文档（非 Markdown 的文本格式：可编辑纯文本视图，不做解析） */
  plainText: boolean
  dirty: boolean
  content: string
  toc: ITocItem[]
  words: number
}

let seq = 0
function nextId(): string {
  seq += 1
  return `tab-${seq}-${Date.now().toString(36)}`
}

// 文件监视：打开/关闭/换路径时维护后端 watcher（去重避免重复 watch）
const trackedPaths = new Set<string>()

/** 把当前活动文档所在目录写入 window.DIRNAME，供 muya 渲染相对路径图片时锚定。
 *  无活动文件（未命名）时清空，回退到 file:// 形式。 */
function syncDirname(tabs: EditorTab[], activeId: string | null) {
  const tab = tabs.find((t) => t.id === activeId) ?? null
  const dir = tab?.path ? tab.path.replace(/[^\\/]+$/, '') : undefined
  if (typeof window !== 'undefined') {
    window.DIRNAME = dir
  }
}

function track(path: string | null) {
  if (!path) return
  if (!trackedPaths.has(path)) {
    trackedPaths.add(path)
    void api.watchFile(path).catch((err) => console.error('watch_file 失败', path, err))
  }
}

function untrack(path: string | null) {
  if (!path) return
  if (trackedPaths.delete(path)) {
    void api.unwatchFile(path).catch((err) => console.error('unwatch_file 失败', path, err))
  }
}

export const THEMES = [
  'light',
  'dark',
  'indigo',
  'emerald',
  'sunset',
  'ocean',
  'rose',
  'dawn',
  'mint',
  'sky',
  'peach',
  'lavender',
] as const
export type ThemeName = (typeof THEMES)[number]

export const ZOOM_MIN = 0.5
export const ZOOM_MAX = 4
export const ZOOM_STEP = 0.1

export const useEditorStore = defineStore('editor', {
  state: () => ({
    tabs: [] as EditorTab[],
    activeId: null as string | null,
    sidebarOpen: true,
    theme: 'light' as ThemeName,
    uiLang: 'zh-CN' as LangCode,
    searchOpen: false,
    currentFolder: null as string | null,
    fps: 0,
    zoom: 1,
  }),

  getters: {
    activeTab(state): EditorTab | null {
      return state.tabs.find((t) => t.id === state.activeId) ?? null
    },
  },

  actions: {
    async boot() {
      const settings = await api.getSettings()
      if (settings.theme && (THEMES as readonly string[]).includes(String(settings.theme))) {
        this.theme = settings.theme as ThemeName
      }
      if (settings.uiLang) {
        this.uiLang = String(settings.uiLang) as LangCode
        setUiLang(this.uiLang)
      } else {
        // 首次启动且未在设置里选过语言：采用安装器选择的语言
        // （NSIS 安装时写入注册表 HKCU\Software\AIRust_MT\InstallLang）
        const installLang = await api.getInstallLang()
        if (installLang && UI_LANGS.some((l) => l.code === String(installLang))) {
          this.uiLang = String(installLang) as LangCode
          setUiLang(this.uiLang)
        }
      }
      document.documentElement.dataset.theme = this.theme
      const initial = await api.takeInitialFile()
      if (initial) {
        try {
          await this.openPath(initial)
        } catch (err) {
          console.error('打开起始文件失败', err)
          void message(translateIoError(err), { title: t('fileReadFailed'), kind: 'error' })
        }
      }
      if (this.tabs.length === 0) {
        this.newTab()
      }
    },

    async persist() {
      await api.setSettings({ theme: this.theme, uiLang: this.uiLang })
    },

    toggleTheme() {
      this.theme = this.theme === 'light' ? 'dark' : this.theme === 'dark' ? 'indigo' : 'light'
      document.documentElement.dataset.theme = this.theme
      void this.persist()
    },

    setTheme(name: ThemeName) {
      this.theme = name
      document.documentElement.dataset.theme = name
      void this.persist()
    },

    setLang(code: LangCode) {
      this.uiLang = code
      setUiLang(code)
      const untitled = t('untitled')
      for (const tab of this.tabs) {
        if (UNTITLED_SNAPSHOTS.includes(tab.title)) tab.title = untitled
      }
      void this.persist()
    },

    newTab(): EditorTab {
      const tab: EditorTab = {
        id: nextId(),
        path: null,
        title: t('untitled'),
        encoding: 'utf-8',
        lossy: false,
        isFull: true,
        dirty: false,
        content: '',
        toc: [],
        words: 0,
        bigFile: false,
        plainText: false,
      }
      this.tabs.push(tab)
      this.activeId = tab.id
      syncDirname(this.tabs, this.activeId)
      return tab
    },

    addTab(payload: { path: string; title: string; encoding: string; content: string; lossy?: boolean; isFull?: boolean; bigFile?: boolean; plainText?: boolean }): EditorTab {
      const tab: EditorTab = {
        id: nextId(),
        path: payload.path,
        title: payload.title,
        encoding: payload.encoding,
        lossy: payload.lossy ?? false,
        isFull: payload.isFull ?? true,
        bigFile: payload.bigFile ?? false,
        plainText: payload.plainText ?? false,
        dirty: false,
        content: payload.content,
        toc: [],
        words: 0,
      }
      this.tabs.push(tab)
      this.activeId = tab.id
      this.sidebarOpen = true
      syncDirname(this.tabs, this.activeId)
      return tab
    },

    activate(id: string) {
      this.activeId = id
      syncDirname(this.tabs, this.activeId)
    },

    closeTab(id: string): boolean {
      const idx = this.tabs.findIndex((t) => t.id === id)
      if (idx === -1) return false
      const tab = this.tabs[idx]
      if (tab.dirty) {
        return true // 由调用方决定保存/丢弃
      }
      untrack(tab.path)
      this.tabs.splice(idx, 1)
      if (this.activeId === id) {
        const next = this.tabs[idx] ?? this.tabs[idx - 1] ?? this.tabs[0]
        this.activeId = next ? next.id : null
      }
      syncDirname(this.tabs, this.activeId)
      return false
    },

    forceClose(id: string) {
      const idx = this.tabs.findIndex((t) => t.id === id)
      if (idx === -1) return
      untrack(this.tabs[idx].path)
      this.tabs.splice(idx, 1)
      if (this.activeId === id) {
        const next = this.tabs[idx] ?? this.tabs[idx - 1] ?? this.tabs[0]
        this.activeId = next ? next.id : null
      }
      syncDirname(this.tabs, this.activeId)
    },

    markDirty(id: string, dirty: boolean) {
      const tab = this.tabs.find((t) => t.id === id)
      if (tab) tab.dirty = dirty
    },

    setTabInfo(id: string, patch: Partial<Pick<EditorTab, 'path' | 'title' | 'encoding' | 'lossy' | 'isFull' | 'dirty' | 'words' | 'toc' | 'content'>>) {
      const tab = this.tabs.find((t) => t.id === id)
      if (!tab) return
      if (patch.path && patch.path !== tab.path) {
        untrack(tab.path)
        track(patch.path)
      }
      Object.assign(tab, patch)
      if (patch.path || patch.path === null) syncDirname(this.tabs, this.activeId)
    },

    async openPath(filePath: string) {
      const payload = await api.openMarkdownPreview(filePath)
      const existing = this.tabs.find((t) => t.path === payload.path)
      if (existing) {
        this.activeId = existing.id
        return existing
      }
      track(payload.path)
      return this.addTab({
        path: payload.path,
        title: payload.name,
        encoding: payload.encoding,
        content: payload.content,
        lossy: payload.lossy,
        isFull: payload.isFull,
        bigFile: !payload.isFull,
        plainText: payload.format === 'text',
      })
    },

    async openDialogAndFile() {
      const payload = await api.openFileDialog()
      if (!payload) return
      const existing = this.tabs.find((t) => t.path === payload.path)
      if (existing) {
        this.activeId = existing.id
        return
      }
      track(payload.path)
      this.addTab({
        path: payload.path,
        title: payload.name,
        encoding: payload.encoding,
        content: payload.content,
        lossy: payload.lossy,
        isFull: payload.isFull,
        bigFile: !payload.isFull,
        plainText: payload.format === 'text',
      })
    },

    zoomIn() {
      this.zoom = Math.min(ZOOM_MAX, Math.round((this.zoom + ZOOM_STEP) * 100) / 100)
    },

    zoomOut() {
      this.zoom = Math.max(ZOOM_MIN, Math.round((this.zoom - ZOOM_STEP) * 100) / 100)
    },

    resetZoom() {
      this.zoom = 1
    },
  },
})