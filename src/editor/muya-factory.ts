import {
  Muya,
  zhCN,
  en,
  ja,
  ko,
  es,
  fr,
  EmojiSelector,
  FootnoteTool,
  InlineFormatToolbar,
  ImageToolBar,
  ImageResizeBar,
  ImageEditTool,
  CodeBlockLanguageSelector,
  LinkTools,
  ParagraphFrontButton,
  ParagraphFrontMenu,
  ParagraphQuickInsertMenu,
  TableChessboard,
  TableColumnToolbar,
  TableDragBar,
  TableRowColumMenu,
  PreviewToolBar,
  type ITocItem,
  type ILocale,
} from '@muyajs/core'
import { api, openUrlInBrowser } from '../api'
import { muyaLocale, type LangCode } from '../i18n'

export interface IEditorEvents {
  onChange: () => void
  onToc: (toc: ITocItem[]) => void
}

// 当前活动编辑器的「文档目录」取值函数。ImageEditTool 是全局注册（Muya.use），
// 其 imageAction 需要在实例层拿到当前文档目录，因此用模块级变量持有。
let activeGetDocDir: () => string = () => ''

const localeMap: Record<string, ILocale> = {
  'zh-CN': zhCN,
  en,
  ja,
  ko,
  es,
  fr,
}

export function resolveLocale(lang: LangCode): ILocale {
  return localeMap[muyaLocale(lang)] ?? en
}

type PluginCtor = {
  pluginName: string
  new (muya: Muya, options: Record<string, unknown>): unknown
}

let registered = false

function registerPlugins() {
  if (registered) return
  registered = true

  Muya.use(EmojiSelector)
  Muya.use(FootnoteTool)
  Muya.use(InlineFormatToolbar)
  Muya.use(ImageToolBar)
  Muya.use(ImageResizeBar)
  Muya.use(ImageEditTool as unknown as PluginCtor, {
    // 点击图片选择器「选择图片」按钮时，弹出原生文件对话框选择图片文件
    imagePathPicker: async () => {
      const path = await api.openImageDialog()
      return path ?? ''
    },
    // 选中本地图片文件后：转存到当前文档目录的 images/ 下，返回可移植的相对路径
    imageAction: async ({ src }: { src: string }) => handleImageSource(src, activeGetDocDir()),
  })
  Muya.use(CodeBlockLanguageSelector)
  Muya.use(LinkTools, {
    jumpClick: (linkInfo: { href?: string } | null) => {
      const href = linkInfo?.href
      if (href && /^https?:\/\//.test(href)) {
        openUrlInBrowser(href).catch(err => console.error('[link] open failed:', err))
      }
    },
  })
  Muya.use(ParagraphFrontButton as unknown as PluginCtor)
  Muya.use(ParagraphFrontMenu)
  Muya.use(ParagraphQuickInsertMenu)
  Muya.use(TableChessboard)
  Muya.use(TableColumnToolbar)
  Muya.use(TableDragBar)
  Muya.use(TableRowColumMenu)
  Muya.use(PreviewToolBar)
}

export function createMuya(
  container: HTMLElement,
  markdown: string,
  events: IEditorEvents,
  getDocDir: () => string,
  lang: LangCode,
): Muya {
  registerPlugins()
  activeGetDocDir = getDocDir
  const muya = new Muya(container, {
    markdown,
    footnote: true,
    codeBlockLineNumbers: true,
    fontSize: 16,
    lineHeight: 1.6,
    spellcheckEnabled: false,
    imageAction: async ({ src }) => handleImageAction(src, getDocDir()),
    getPathForFile: (file: File) => file.name,
  })
  muya.locale(resolveLocale(lang))
  muya.init()

  // Ctrl/Cmd+点击链接时 muya 发 format-click；普通点击仅放置光标（WYSIWYG 惯例）。
  // 宿主在此订阅，把外链交给系统浏览器打开。
  muya.eventCenter.subscribe(
    'format-click',
    (payload: { formatType?: string; data?: { href?: string | null } }) => {
      if (payload?.formatType !== 'link') return
      const href = payload.data?.href
      if (href && /^https?:\/\//.test(href)) {
        openUrlInBrowser(href).catch(err => console.error('[link] open failed:', err))
      }
    },
  )

  let tocTimer: ReturnType<typeof setTimeout> | null = null
  const refreshToc = () => {
    events.onToc(muya.getTOC())
  }
  muya.on('json-change', () => {
    events.onChange()
    if (tocTimer) clearTimeout(tocTimer)
    tocTimer = setTimeout(refreshToc, 400)
  })
  muya.on('selection-change', () => {
    if (tocTimer) clearTimeout(tocTimer)
    tocTimer = setTimeout(refreshToc, 400)
  })

  // 初始化完成后立即输出一次大纲，避免新开文档大纲为空
  queueMicrotask(refreshToc)
  setTimeout(refreshToc, 100)

  return muya
}

/**
 * 剪贴板贴图：muya 会把粘贴的位图以 data: URL 交给 imageAction。
 * 这里落盘到当前文档目录的 images/ 下，返回相对路径，保证文档可移植。
 */
export async function handleImageAction(src: string, docDir: string): Promise<string> {
  if (!src.startsWith('data:image/')) return src
  const match = /^data:image\/(\w+);base64,([\s\S]+)$/.exec(src)
  if (!match) return src
  try {
    const binary = atob(match[2])
    const bytes = new Uint8Array(binary.length)
    for (let i = 0; i < binary.length; i++) bytes[i] = binary.charCodeAt(i)
    const ext = match[1] === 'jpeg' ? 'jpg' : match[1] === 'svg+xml' ? 'svg' : match[1]
    const relative = await api.saveImage(Array.from(bytes), docDir, ext)
    return relative
  } catch (err) {
    console.error('handleImageAction', err)
    return src
  }
}

/**
 * 统一处理图片来源并转存到当前文档目录的 images/ 下：
 * - data: URL（剪贴板贴图）→ 解码后转存
 * - http(s) 链接 → 原样返回（Muya 直接渲染）
 * - 本地路径（绝对或相对、正斜杠 / 反斜杠均可）→ 读取二进制后转存
 * 返回可移植的相对路径，保证文档移动后图片仍可用。
 * 本地图片读取 / 转存失败返回空字符串，由调用方给出错误提示。
 */
export async function handleImageSource(src: string, docDir: string): Promise<string> {
  if (src.startsWith('data:image/')) return handleImageAction(src, docDir)
  if (/^https?:\/\//.test(src)) return src

  // 剥离 file:// 前缀。file:// URL 中的空格/中文是百分号编码（%20 等），
  // 需解码回原始路径字符；手动输入的普通路径不解码，避免误伤文件名里本身的 %。
  const isFileUrl = /^file:\/\//.test(src)
  let norm = src.replace(/^file:\/\/(?:\/)?/, '')
  if (isFileUrl) {
    try {
      norm = decodeURIComponent(norm)
    } catch {
      // 编码序列非法时保留原值
    }
  }
  // 把 ./ .\ 这类相对前缀规整为普通相对路径
  norm = norm.replace(/^[.]{1,2}[\\/]/, '')

  const readAndSave = async (filePath: string): Promise<string> => {
    const bytes = await api.readBinaryFile(filePath)
    const ext = (filePath.split('.').pop() ?? 'png').toLowerCase()
    return api.saveImage(bytes, docDir, ext)
  }

  // 绝对路径（Windows 盘符 / UNC / POSIX 根）
  const isAbsPath = /^(?:[a-zA-Z]:[\\/]|\\\\|\/)/.test(norm)
  // 相对路径 / 文件名
  const isRelPath = /(?:[\\/]|\.(?:png|jpe?g|gif|bmp|svg|webp|avif|ico|tiff?)$)/i.test(norm)

  const candidates: string[] = []
  if (isAbsPath) {
    candidates.push(norm)
  } else if (isRelPath) {
    // 手动输入的相对路径以当前文档目录为基准试解析；同时直接以原始串尝试一次
    if (docDir) candidates.push(`${docDir.replace(/[\\/]+$/, '')}/${norm.replace(/\\/g, '/')}`)
    candidates.push(norm)
  } else {
    return ''
  }

  for (const candidate of candidates) {
    try {
      const relative = await readAndSave(candidate)
      if (relative) return relative
    } catch (err) {
      console.error('handleImageSource', candidate, err)
    }
  }
  return ''
}

/**
 * 滚动到指定标题。toc 的 content 是剥离内联标记的纯文本，DOM 标题 textContent
 * 常含 `**加粗**`、`[label](url)` 等内联标记，纯文本相等会匹配失败。
 * 改用「该 level 标题在文档中的出现序号」定位（toc 与 DOM 标题均按文档顺序），
 * 更稳定。ordinal 从 0 开始，表示第 ordinal+1 个该 level 的标题。
 */
export function scrollToHeading(container: HTMLElement, level: number, ordinal: number): boolean {
  const headings = Array.from(container.querySelectorAll<HTMLElement>('h1,h2,h3,h4,h5,h6'))
  let seen = -1
  for (const el of headings) {
    const tag = Number(el.tagName[1])
    if (tag === level) {
      seen += 1
      if (seen === ordinal) {
        el.scrollIntoView({ behavior: 'smooth', block: 'start' })
        return true
      }
    }
  }
  return false
}