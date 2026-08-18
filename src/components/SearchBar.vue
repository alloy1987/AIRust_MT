<script setup lang="ts">
import { ref, watch, nextTick } from 'vue'
import { useEditorStore } from '../stores/editor'
import { t } from '../i18n'
import type { Muya } from '@muyajs/core'

const store = useEditorStore()

const props = defineProps<{ muya: Muya | null }>()
const keyword = ref('')
const replaceText = ref('')
const caseSensitive = ref(false)
const wholeWord = ref(false)
const regexp = ref(false)
const matchInfo = ref('')
const replaceAllOpen = ref(false)

watch(
  () => store.searchOpen,
  async (open) => {
    if (open) {
      await nextTick()
      inputRef.value?.focus()
      if (keyword.value) runSearch()
    }
  },
)

const inputRef = ref<HTMLInputElement | null>(null)

function updateMatchInfo(searcher: { matches?: unknown; index?: number }) {
  const matches = searcher.matches
  const total = Array.isArray(matches) ? matches.length : 0
  const cur = searcher.index ?? -1
  if (total > 0) {
    const shownIdx = cur >= 0 ? cur + 1 : 1
    matchInfo.value = `${shownIdx}/${total}`
  } else {
    matchInfo.value = t('noResult')
  }
}

function runSearch() {
  const muya = props.muya
  if (!muya) return
  const result = muya.search(keyword.value, {
    isCaseSensitive: caseSensitive.value,
    isWholeWord: wholeWord.value,
    isRegexp: regexp.value,
  })
  updateMatchInfo(result)
}

function findNext() {
  const muya = props.muya
  if (!muya) return
  const result = muya.find('next')
  updateMatchInfo(result)
}

function findPrev() {
  const muya = props.muya
  if (!muya) return
  const result = muya.find('previous')
  updateMatchInfo(result)
}

function doReplace() {
  props.muya?.replace(replaceText.value, { isSingle: true, isRegexp: regexp.value })
  runSearch()
}

function doReplaceAll() {
  props.muya?.replace(replaceText.value, { isSingle: false, isRegexp: regexp.value })
  runSearch()
}

function close() {
  store.searchOpen = false
  const muya = props.muya
  if (muya) muya.search('')
}
</script>

<template>
  <div class="search-bar">
    <div class="search-row">
      <input v-model="keyword" class="s-input" :placeholder="t('searchPlaceholder')" @input="runSearch" @keydown.enter.prevent="findNext" />
      <button class="s-btn" :title="t('prev')" @click="findPrev">↑</button>
      <button class="s-btn" :title="t('next')" @click="findNext">↓</button>
      <span class="s-info">{{ matchInfo }}</span>
      <label class="s-check"><input v-model="caseSensitive" type="checkbox" @change="runSearch" />{{ t('caseSensitive') }}</label>
      <label class="s-check"><input v-model="wholeWord" type="checkbox" @change="runSearch" />{{ t('wholeWord') }}</label>
      <label class="s-check"><input v-model="regexp" type="checkbox" @change="runSearch" />{{ t('regexp') }}</label>
      <button class="s-btn" @click="close">✕</button>
    </div>
    <div v-if="replaceAllOpen" class="search-row">
      <input v-model="replaceText" class="s-input" :placeholder="t('replacePlaceholder')" @keydown.enter.prevent="doReplace" />
      <button class="s-btn" @click="doReplace">{{ t('replaceOne') }}</button>
      <button class="s-btn" @click="doReplaceAll">{{ t('replaceAll') }}</button>
    </div>
    <div class="search-row">
      <button class="s-btn s-toggle" @click="replaceAllOpen = !replaceAllOpen">
        {{ replaceAllOpen ? t('done') : t('replace') }}
      </button>
    </div>
  </div>
</template>

<style scoped>
.search-bar {
  background: var(--panel-bg);
  border-bottom: 1px solid var(--border);
  padding: 6px 10px;
  display: flex;
  flex-direction: column;
  gap: 4px;
  flex-shrink: 0;
}
.search-row {
  display: flex;
  align-items: center;
  gap: 6px;
  flex-wrap: wrap;
}
.s-input {
  flex: 1;
  min-width: 160px;
  height: 26px;
  border: 1px solid var(--border);
  border-radius: 4px;
  background: var(--input-bg);
  color: var(--text-primary);
  padding: 0 8px;
  font-size: 12.5px;
  outline: none;
}
.s-input:focus {
  border-color: var(--accent);
}
.s-btn {
  height: 26px;
  border: 1px solid var(--border);
  border-radius: 4px;
  background: var(--button-bg);
  color: var(--text-secondary);
  font-size: 12px;
  padding: 0 10px;
  cursor: pointer;
}
.s-btn:hover {
  background: var(--hover-bg);
  color: var(--text-primary);
}
.s-toggle {
  align-self: flex-start;
}
.s-info {
  font-size: 12px;
  color: var(--text-tertiary);
  min-width: 40px;
}
.s-check {
  display: flex;
  align-items: center;
  gap: 3px;
  font-size: 12px;
  color: var(--text-secondary);
}
</style>