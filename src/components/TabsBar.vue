<script setup lang="ts">
import { nextTick, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { useEditorStore } from '../stores/editor'
import { t } from '../i18n'

const store = useEditorStore()

const emit = defineEmits<{
  (e: 'close-request', id: string): void
}>()

const tabsRef = ref<HTMLElement | null>(null)
const canScrollLeft = ref(false)
const canScrollRight = ref(false)

function updateArrows() {
  const el = tabsRef.value
  if (!el)
    return
  canScrollLeft.value = el.scrollLeft > 1
  canScrollRight.value = el.scrollLeft + el.clientWidth < el.scrollWidth - 1
}

function scrollByDir(dir: 1 | -1) {
  const el = tabsRef.value
  if (!el)
    return
  el.scrollBy({ left: dir * el.clientWidth * 0.8, behavior: 'smooth' })
}

function scrollActiveIntoView(behavior: ScrollBehavior = 'smooth') {
  const el = tabsRef.value
  const id = store.activeId
  if (!el || !id)
    return
  const tabEl = el.querySelector<HTMLElement>(`[data-tab-id="${CSS.escape(id)}"]`)
  if (!tabEl)
    return
  const cRect = el.getBoundingClientRect()
  const tRect = tabEl.getBoundingClientRect()
  if (tRect.left < cRect.left)
    el.scrollBy({ left: tRect.left - cRect.left - 8, behavior })
  else if (tRect.right > cRect.right)
    el.scrollBy({ left: tRect.right - cRect.right + 8, behavior })
}

function onWheel(e: WheelEvent) {
  const el = tabsRef.value
  if (!el || Math.abs(e.deltaY) <= Math.abs(e.deltaX))
    return
  e.preventDefault()
  el.scrollLeft += e.deltaY
}

let resizeObserver: ResizeObserver | null = null

onMounted(() => {
  const el = tabsRef.value
  if (el) {
    resizeObserver = new ResizeObserver(() => updateArrows())
    resizeObserver.observe(el)
  }
  updateArrows()
  scrollActiveIntoView('auto')
})

onBeforeUnmount(() => {
  resizeObserver?.disconnect()
  resizeObserver = null
})

watch(() => store.activeId, () => {
  nextTick(() => {
    scrollActiveIntoView()
    updateArrows()
  })
})

watch(() => store.tabs.length, () => {
  nextTick(updateArrows)
})
</script>

<template>
  <div class="tabs-bar">
    <button
      v-if="canScrollLeft"
      class="tab-scroll-btn"
      @click="scrollByDir(-1)"
    >
      <svg width="10" height="10" viewBox="0 0 10 10"><path d="M6.5 1 L3 5 L6.5 9" stroke="currentColor" stroke-width="1.4" fill="none"/></svg>
    </button>
    <div ref="tabsRef" class="tabs" @scroll.passive="updateArrows" @wheel="onWheel">
      <div
        v-for="tab in store.tabs"
        :key="tab.id"
        :data-tab-id="tab.id"
        class="tab"
        :class="{ active: tab.id === store.activeId }"
        @click="store.activate(tab.id)"
        @contextmenu.prevent.stop
      >
        <span class="tab-title">{{ tab.title }}</span>
        <span v-if="tab.dirty" class="dirty-dot"></span>
        <button class="tab-close" :title="t('close')" @click.stop="emit('close-request', tab.id)">
          <svg width="8" height="8" viewBox="0 0 10 10"><path d="M0.5 0.5 L9.5 9.5 M9.5 0.5 L0.5 9.5" stroke="currentColor" stroke-width="1.4"/></svg>
        </button>
      </div>
      <button class="tab-new" :title="t('newFile')" @click="store.newTab()">
        <svg width="12" height="12" viewBox="0 0 12 12"><path d="M6 1 L6 11 M1 6 L11 6" stroke="currentColor" stroke-width="1.4"/></svg>
      </button>
    </div>
    <button
      v-if="canScrollRight"
      class="tab-scroll-btn"
      @click="scrollByDir(1)"
    >
      <svg width="10" height="10" viewBox="0 0 10 10"><path d="M3.5 1 L7 5 L3.5 9" stroke="currentColor" stroke-width="1.4" fill="none"/></svg>
    </button>
  </div>
</template>

<style scoped>
.tabs-bar {
  height: 34px;
  display: flex;
  align-items: flex-end;
  background: var(--tabs-bg);
  border-bottom: 1px solid var(--border);
  flex-shrink: 0;
}
.tabs {
  display: flex;
  align-items: flex-end;
  gap: 2px;
  padding: 4px 6px 0;
  overflow-x: auto;
  scrollbar-width: none;
  flex: 1;
  min-width: 0;
}
.tab-scroll-btn {
  border: none;
  background: var(--tabs-bg);
  color: var(--text-tertiary);
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  align-self: stretch;
  flex-shrink: 0;
}
.tab-scroll-btn:hover {
  color: var(--text-primary);
  background: var(--hover-bg);
}
.tab {
  display: flex;
  align-items: center;
  gap: 6px;
  height: 30px;
  padding: 0 10px;
  font-size: 12.5px;
  color: var(--text-secondary);
  background: transparent;
  border: 1px solid transparent;
  border-bottom: none;
  border-radius: 6px 6px 0 0;
  cursor: default;
  white-space: nowrap;
  max-width: 180px;
  flex-shrink: 0;
}
.tab:hover {
  background: var(--hover-bg);
}
.tab.active {
  background: var(--editor-bg);
  color: var(--text-primary);
  border-color: var(--border);
}
.tab-title {
  overflow: hidden;
  text-overflow: ellipsis;
}
.dirty-dot {
  width: 7px;
  height: 7px;
  border-radius: 50%;
  background: var(--accent);
  flex-shrink: 0;
}
.tab-close {
  border: none;
  background: transparent;
  color: var(--text-tertiary);
  display: flex;
  padding: 2px;
  border-radius: 4px;
}
.tab-close:hover {
  color: var(--text-primary);
  background: var(--hover-bg);
}
.tab-new {
  margin-left: 4px;
  margin-bottom: 2px;
  border: none;
  background: transparent;
  color: var(--text-tertiary);
  display: flex;
  padding: 4px;
  border-radius: 6px;
  flex-shrink: 0;
}
.tab-new:hover {
  color: var(--text-primary);
  background: var(--hover-bg);
}
</style>
