<script setup lang="ts">
import { ref } from 'vue'
import { t } from '../i18n'
import { api } from '../api'

const emit = defineEmits<{
  (e: 'insert', src: string): void
  (e: 'cancel'): void
}>()

const path = ref('')
const error = ref('')

// 「按下处」是否位于遮罩层上：从对话框内部按下再拖到遮罩上松开时，
// click 事件会派发到两者的共同祖先（遮罩层），若直接按 click.self 取消
// 会把「拖出框外」误判为「点击遮罩关闭」。只有按下与松开都在遮罩上才取消。
let pressOnMask = false

function onMaskMouseDown(event: MouseEvent) {
  pressOnMask = event.target === event.currentTarget
}

function onMaskClick() {
  if (pressOnMask) emit('cancel')
}

/** 去除路径首尾的引号（Win 用 Ctrl+Shift+C 复制的路径常带双引号）。 */
function cleanImagePath(raw: string): string {
  let s = raw.trim()
  while (s.startsWith('"') || s.startsWith("'")) {
    s = s.slice(1).trimLeft()
  }
  while (s.endsWith('"') || s.endsWith("'")) {
    s = s.slice(0, -1).trimEnd()
  }
  return s
}

// 只在「粘贴」时清理引号：Windows 复制路径常带首尾引号。
// 绝不监听每次键入实时清理——手动输入路径时，路径中合法的撇号/引号
// （如文件夹名 O'Brien）一旦位于字符串末尾就会被误删，导致路径损坏。
function onPaste(event: ClipboardEvent) {
  event.preventDefault()
  const text = event.clipboardData?.getData('text') ?? ''
  const input = event.currentTarget as HTMLInputElement
  const start = input.selectionStart ?? input.value.length
  const end = input.selectionEnd ?? input.value.length
  const next = input.value.slice(0, start) + cleanImagePath(text) + input.value.slice(end)
  path.value = next
  error.value = ''
  requestAnimationFrame(() => {
    const pos = start + cleanImagePath(text).length
    input.setSelectionRange(pos, pos)
  })
}

const PATH_PATTERN = /\.(png|jpe?g|gif|bmp|svg|svgz|webp|avif|ico|tiff?)$/i
const URL_PATTERN = /^https?:\/\/|^data:image\//
const LOCAL_PATH_PATTERN = /^(?:[a-zA-Z]:[\\/]|\\\\|file:\/\/|\/)/

function looksLikeImageSource(src: string): boolean {
  if (URL_PATTERN.test(src) || LOCAL_PATH_PATTERN.test(src)) return true
  return PATH_PATTERN.test(src)
}

async function chooseFile() {
  const picked = await api.openImageDialog()
  if (picked) {
    path.value = picked
    error.value = ''
  }
}

function confirmInsert() {
  const src = cleanImagePath(path.value)
  if (!src) {
    error.value = t('imagePathEmpty') ?? ''
    return
  }
  if (!looksLikeImageSource(src)) {
    error.value = t('imagePathInvalid') ?? ''
    return
  }
  error.value = ''
  emit('insert', src)
}
</script>

<template>
  <div class="dialog-mask" @mousedown="onMaskMouseDown" @click.self="onMaskClick">
    <div class="dialog" role="dialog" aria-modal="true">
      <div class="dialog-title">{{ t('insertImage') }}</div>
      <div class="field">
        <input
          v-model="path"
          class="path-input"
          :class="{ 'input-error': error }"
          :placeholder="t('imagePath')"
          spellcheck="false"
          @keydown.enter="confirmInsert"
          @paste="onPaste"
        />
        <div v-if="error" class="error-hint">{{ error }}</div>
      </div>
      <div class="dialog-actions">
        <button class="btn" @click="chooseFile">{{ t('chooseImage') }}</button>
        <button class="btn primary" @click="confirmInsert">{{ t('insert') }}</button>
        <button class="btn" @click="emit('cancel')">{{ t('cancel') }}</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.dialog-mask {
  position: fixed;
  inset: 0;
  background: rgb(0 0 0 / 35%);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}
.dialog {
  width: 460px;
  background: var(--float-bg-color, var(--panel-bg));
  border: 1px solid var(--float-border-color, var(--border));
  border-radius: 10px;
  box-shadow: 0 10px 32px rgb(0 0 0 / 25%);
  padding: 18px 20px 16px;
}
.dialog-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 12px;
}
.field {
  margin-bottom: 16px;
}
.path-input {
  width: 100%;
  height: 32px;
  box-sizing: border-box;
  padding: 0 10px;
  border: 1px solid var(--border);
  border-radius: 6px;
  background: var(--editor-bg);
  color: var(--text-primary);
  font-family: var(--mono, ui-monospace, Consolas, monospace);
  font-size: 12.5px;
  outline: none;
}
.path-input:focus {
  border-color: var(--accent);
}
.path-input.input-error {
  border-color: #e5484d;
}
.error-hint {
  color: #e5484d;
  font-size: 12px;
  margin-top: 6px;
}
.dialog-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}
.btn {
  height: 30px;
  padding: 0 16px;
  border: 1px solid var(--border);
  border-radius: 6px;
  background: var(--panel-bg);
  color: var(--text-primary);
  font-size: 12.5px;
  cursor: default;
}
.btn:hover {
  background: var(--hover-bg);
}
.btn.primary {
  background: var(--accent);
  border-color: var(--accent);
  color: #fff;
}
.btn.primary:hover {
  filter: brightness(1.08);
}
</style>