<script setup lang="ts">
import { t, UI_LANGS, type I18nKey } from '../i18n'
import { useEditorStore, THEMES } from '../stores/editor'

const store = useEditorStore()

const emit = defineEmits<{
  (e: 'close'): void
}>()

const themeKeyMap: Record<string, I18nKey> = {
  light: 'themeLight',
  dark: 'themeDark',
  indigo: 'themeIndigo',
  emerald: 'themeEmerald',
  sunset: 'themeSunset',
  ocean: 'themeOcean',
  rose: 'themeRose',
  dawn: 'themeDawn',
  mint: 'themeMint',
  sky: 'themeSky',
  peach: 'themePeach',
  lavender: 'themeLavender',
}

function themeLabel(name: string): string {
  return t(themeKeyMap[name] ?? 'themeLight')
}

function onLangChange(e: Event) {
  const code = (e.target as HTMLSelectElement).value
  if (UI_LANGS.some((l) => l.code === code)) store.setLang(code as (typeof UI_LANGS)[number]['code'])
}

function onThemeChange(e: Event) {
  const name = (e.target as HTMLSelectElement).value
  if ((THEMES as readonly string[]).includes(name)) store.setTheme(name as (typeof THEMES)[number])
}
</script>

<template>
  <div class="settings-mask" @click.self="emit('close')">
    <div class="settings-dialog" role="dialog" aria-modal="true">
      <div class="settings-head">
        <span class="settings-title">{{ t('options') }}</span>
        <button class="settings-close" title="×" @click="emit('close')">✕</button>
      </div>

      <div class="settings-body">
        <div class="settings-section">
          <div class="section-label">{{ t('uiLang') }}</div>
          <select class="settings-select" :value="store.uiLang" @change="onLangChange">
            <option v-for="l in UI_LANGS" :key="l.code" :value="l.code">{{ l.label }}</option>
          </select>
        </div>

        <div class="settings-section">
          <div class="section-label">{{ t('skin') }}</div>
          <select class="settings-select" :value="store.theme" @change="onThemeChange">
            <option v-for="name in THEMES" :key="name" :value="name">{{ themeLabel(name) }}</option>
          </select>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.settings-mask {
  position: fixed;
  inset: 0;
  background: rgb(0 0 0 / 35%);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}
.settings-dialog {
  width: 440px;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
  background: var(--float-bg-color, var(--panel-bg));
  border: 1px solid var(--float-border-color, var(--border));
  border-radius: 10px;
  box-shadow: 0 10px 32px rgb(0 0 0 / 25%);
  overflow: hidden;
}
.settings-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 13px 18px;
  border-bottom: 1px solid var(--float-border-color, var(--border));
  flex-shrink: 0;
}
.settings-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}
.settings-close {
  border: none;
  background: transparent;
  color: var(--text-tertiary);
  font-size: 13px;
  padding: 3px 6px;
  border-radius: 5px;
  cursor: default;
}
.settings-close:hover {
  background: var(--float-hover-color, var(--hover-bg));
  color: var(--text-primary);
}
.settings-body {
  overflow-y: auto;
  padding: 16px 18px 18px;
  display: flex;
  flex-direction: column;
  gap: 18px;
}
.section-label {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-tertiary);
  margin-bottom: 8px;
}
.settings-select {
  width: 100%;
  height: 34px;
  border: 1px solid var(--float-border-color, var(--border));
  border-radius: 7px;
  background: var(--float-bg-color, var(--panel-bg));
  color: var(--text-primary);
  font-size: 13px;
  padding: 0 10px;
  outline: none;
  cursor: default;
  font-family: inherit;
}
.settings-select:hover,
.settings-select:focus {
  border-color: var(--accent);
}
</style>
