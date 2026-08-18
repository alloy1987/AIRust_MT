<script setup lang="ts">
import { computed, ref } from 'vue'
import { useEditorStore, ZOOM_MIN, ZOOM_MAX } from '../stores/editor'

const store = useEditorStore()

const MIN = ZOOM_MIN
const MAX = ZOOM_MAX
const trackRef = ref<HTMLElement | null>(null)
const dragging = ref(false)

const percent = computed(() => Math.round(store.zoom * 100))
const thumbPos = computed(() => ((store.zoom - MIN) / (MAX - MIN)) * 100)
const markPos = computed(() => ((1 - MIN) / (MAX - MIN)) * 100)

function setFromClientX(clientX: number) {
  const el = trackRef.value
  if (!el) return
  const rect = el.getBoundingClientRect()
  let ratio = (clientX - rect.left) / rect.width
  ratio = Math.min(1, Math.max(0, ratio))
  const snapped = Math.round(ratio * 100) / 100
  if (snapped === Math.round(markPos.value) / 100) {
    store.zoom = 1
  } else {
    store.zoom = Math.round((MIN + ratio * (MAX - MIN)) * 100) / 100
  }
}

function onPointerDown(e: PointerEvent) {
  dragging.value = true
  ;(e.target as HTMLElement).setPointerCapture?.(e.pointerId)
  setFromClientX(e.clientX)
}

function onPointerMove(e: PointerEvent) {
  if (dragging.value) setFromClientX(e.clientX)
}

function onPointerUp() {
  dragging.value = false
}

function resetTo100() {
  store.zoom = 1
}

function onWheel(e: WheelEvent) {
  e.preventDefault()
  if (e.deltaY < 0 && store.zoom < MAX) store.zoomIn()
  else if (e.deltaY > 0 && store.zoom > MIN) store.zoomOut()
}
</script>

<template>
  <div class="zoom-slider" :class="{ dragging }" @wheel.prevent="onWheel">
    <span class="zoom-label">{{ percent }}%</span>
    <div
      ref="trackRef"
      class="zoom-track"
      @pointerdown="onPointerDown"
      @pointermove="onPointerMove"
      @pointerup="onPointerUp"
      @pointercancel="onPointerUp"
    >
      <div class="zoom-fill" :style="{ width: thumbPos + '%' }"></div>
      <button
        class="zoom-mark"
        :style="{ left: markPos + '%' }"
        title="恢复 100%"
        @click.stop="resetTo100"
      ></button>
      <div class="zoom-thumb" :style="{ left: thumbPos + '%' }"></div>
    </div>
  </div>
</template>

<style scoped>
.zoom-slider {
  display: flex;
  align-items: center;
  gap: 7px;
  user-select: none;
  touch-action: none;
}
.zoom-label {
  font-size: 11px;
  color: var(--text-tertiary);
  min-width: 34px;
  text-align: right;
  font-variant-numeric: tabular-nums;
}
.zoom-track {
  position: relative;
  width: 110px;
  height: 16px;
  cursor: pointer;
  display: flex;
  align-items: center;
}
.zoom-fill {
  position: absolute;
  height: 3px;
  left: 0;
  border-radius: 2px;
  background: var(--accent);
  opacity: 0.45;
}
.zoom-mark {
  position: absolute;
  top: 50%;
  width: 9px;
  height: 9px;
  border-radius: 50%;
  background: var(--titlebar-bg, #fff);
  border: 1.5px solid var(--text-tertiary);
  transform: translate(-50%, -50%);
  padding: 0;
  cursor: pointer;
}
.zoom-mark:hover {
  border-color: var(--accent);
  box-shadow: 0 0 0 3px rgb(0 0 0 / 10%);
}
.zoom-thumb {
  position: absolute;
  top: 50%;
  width: 12px;
  height: 12px;
  border-radius: 50%;
  background: var(--accent);
  border: 2px solid var(--titlebar-bg, #fff);
  transform: translate(-50%, -50%);
  box-shadow: 0 1px 3px rgb(0 0 0 / 35%);
  transition: box-shadow 0.15s;
}
.zoom-thumb:hover,
.dragging .zoom-thumb {
  box-shadow: 0 0 0 4px rgb(0 0 0 / 12%);
}
</style>