export interface FilePayload {
  path: string
  name: string
  content: string
  encoding: string
  lossy: boolean
  isFull: boolean
  /** 显示格式（后端按扩展名判定）：markdown = 所见即所得解析渲染；text = 可编辑纯文本 */
  format: 'markdown' | 'text'
}

/** 后端错误码（与 src-tauri/src/encoding.rs 约定） */
export const UNMAPPABLE_CODE = 'ENCODING_UNMAPPABLE'
export const LOSSY_CODE = 'LOSSY_SAVE_BLOCKED'
/** 后端错误码（与 src-tauri/src/large_file.rs 约定）：文件超过完整加载上限（100 MB） */
export const FULL_LOAD_TOO_LARGE_CODE = 'FULL_LOAD_TOO_LARGE'
/** 后端错误码（与 src-tauri/src/large_file.rs 约定）：内容嗅探判定为非纯文本（二进制文件） */
export const NOT_TEXT_CODE = 'NOT_TEXT_FILE'
/** 前端错误码：截断缓冲已被编辑，覆盖保存会截断文件 */
export const LARGE_FILE_EDITED_CODE = 'LARGE_FILE_EDITED'

export interface TreeEntry {
  name: string
  path: string
  /** 后端 serde camelCase：Rust 侧 is_dir 序列化为 isDir */
  isDir: boolean
  children: TreeEntry[] | null
}

export interface Settings {
  theme?: string
  uiLang?: string
  [key: string]: unknown
}

export type Option<T> = T | null

async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  const { invoke } = await import('@tauri-apps/api/core')
  return invoke<T>(cmd, args)
}

export const api = {
  takeInitialFile: () => invoke<Option<string>>('take_initial_file'),
  quitApp: () => invoke<void>('quit_app'),
  watchFile: (path: string) => invoke<void>('watch_file', { path }),
  unwatchFile: (path: string) => invoke<void>('unwatch_file', { path }),
  openFileDialog: () => invoke<Option<FilePayload>>('open_file_dialog'),
  openFolderDialog: () => invoke<Option<string>>('open_folder_dialog'),
  openImageDialog: () => invoke<Option<string>>('open_image_dialog'),
  readFile: (path: string) => invoke<FilePayload>('read_file', { path }),
  openMarkdownPreview: (path: string, maxBytes?: number) =>
    invoke<FilePayload>('open_markdown_preview', { path, maxBytes }),
  ensureFullContent: (path: string) => invoke<string>('ensure_full_content', { path }),
  saveFile: (path: string, content: string, encoding: string) =>
    invoke<void>('save_file', { path, content, encoding }),
  saveFileAsDialog: (content: string, suggestedName: string) =>
    invoke<Option<string>>('save_file_as_dialog', { content, suggestedName }),
  listDir: (path: string, depth = 5) => invoke<TreeEntry[]>('list_dir', { path, depth }),
  saveImage: (bytes: number[], docDir: string, extension: string) =>
    invoke<string>('save_image', { bytes, docDir, extension }),
  readBinaryFile: (path: string) => invoke<number[]>('read_binary_file', { path }),
  getSettings: () => invoke<Settings>('get_settings'),
  setSettings: (value: Settings) => invoke<void>('set_settings', { value }),
  getInstallLang: () => invoke<Option<string>>('get_install_lang'),
}

export async function openUrlInBrowser(url: string) {
  const { openUrl } = await import('@tauri-apps/plugin-opener')
  await openUrl(url)
}

export async function readClipboardImageAsBytes(): Promise<{ bytes: number[]; ext: string } | null> {
  try {
    const items = await navigator.clipboard.read()
    for (const item of items) {
      const types = item.types.filter((t) => t.startsWith('image/'))
      if (types.length === 0) continue
      const blob = await item.getType(types[0])
      const bytes = Array.from(new Uint8Array(await blob.arrayBuffer()))
      const ext = types[0].split('/')[1] === 'jpeg' ? 'jpg' : types[0].split('/')[1] ?? 'png'
      return { bytes, ext }
    }
  } catch {
    // 剪贴板无图片或权限被拒
  }
  return null
}