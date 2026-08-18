<script setup lang="ts">
import { t } from '../i18n'

defineProps<{
  title: string
  message: string
}>()

const emit = defineEmits<{
  (e: 'reload'): void
  (e: 'keep'): void
}>()
</script>

<template>
  <!-- 不绑定「点遮罩关闭」：外部修改提示必须由用户通过按钮明确决定，误点外部不应让弹窗消失 -->
  <div class="dialog-mask">
    <div class="dialog" role="dialog" aria-modal="true">
      <div class="dialog-title">{{ title }}</div>
      <div class="dialog-message">{{ message }}</div>
      <div class="dialog-actions">
        <button class="btn primary" @click="emit('reload')">{{ t('reloadFile') }}</button>
        <button class="btn" @click="emit('keep')">{{ t('keepCurrent') }}</button>
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
  width: 400px;
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
  margin-bottom: 10px;
}
.dialog-message {
  font-size: 13px;
  line-height: 1.6;
  color: var(--text-secondary);
  word-break: break-all;
  margin-bottom: 18px;
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