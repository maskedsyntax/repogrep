<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import SnippetInputPane from './components/SnippetInputPane.vue'
import MatchListPane from './components/MatchListPane.vue'
import CodePreviewPane from './components/CodePreviewPane.vue'
import { useAppStore } from './stores/app'

const store = useAppStore()

const MIN_PCT = 20
const MAX_PCT = 60
const DEFAULT_LEFT = 32
const DEFAULT_MID = 34

const leftPct = ref(DEFAULT_LEFT)
const midPct = ref(DEFAULT_MID)

let dragging = null
let startX = 0
let startLeft = 0
let startMid = 0

function onDividerMouseDown(which, e) {
  dragging = which
  startX = e.clientX
  startLeft = leftPct.value
  startMid = midPct.value
  e.preventDefault()
}

function onMouseMove(e) {
  if (dragging === null) return
  const dx = e.clientX - startX
  const container = document.querySelector('.app')
  if (!container) return
  const total = container.offsetWidth
  const deltaPct = (dx / total) * 100
  if (dragging === 1) {
    let newLeft = startLeft + deltaPct
    let newMid = startMid - deltaPct
    if (newLeft < MIN_PCT) { newLeft = MIN_PCT; newMid = startMid + (startLeft - MIN_PCT) }
    if (newLeft > MAX_PCT) { newLeft = MAX_PCT; newMid = startMid + (startLeft - MAX_PCT) }
    if (newMid < MIN_PCT) { newMid = MIN_PCT; newLeft = startLeft + (startMid - MIN_PCT) }
    if (newMid > MAX_PCT) { newMid = MAX_PCT; newLeft = startLeft + (startMid - MAX_PCT) }
    leftPct.value = Math.round(newLeft)
    midPct.value = Math.round(newMid)
  } else {
    let newMid = startMid + deltaPct
    const rightPct = 100 - leftPct.value - newMid
    if (newMid < MIN_PCT) newMid = MIN_PCT
    if (rightPct < MIN_PCT) newMid = 100 - leftPct.value - MIN_PCT
    if (newMid > MAX_PCT) newMid = MAX_PCT
    midPct.value = Math.round(newMid)
  }
}

function onMouseUp() {
  dragging = null
}

onMounted(() => {
  window.addEventListener('mousemove', onMouseMove)
  window.addEventListener('mouseup', onMouseUp)
})
onUnmounted(() => {
  window.removeEventListener('mousemove', onMouseMove)
  window.removeEventListener('mouseup', onMouseUp)
})

const gridStyle = () => ({
  gridTemplateColumns: `minmax(220px, ${leftPct.value}%) 6px minmax(220px, ${midPct.value}%) 6px minmax(280px, 1fr)`,
})
</script>

<template>
  <div class="app" :style="gridStyle()">
    <div class="pane-col snippet-col">
      <SnippetInputPane />
    </div>
    <div
      class="resizer"
      aria-label="Resize panes"
      @mousedown="onDividerMouseDown(1, $event)"
    />
    <div class="pane-col list-col">
      <MatchListPane />
    </div>
    <div
      class="resizer"
      aria-label="Resize panes"
      @mousedown="onDividerMouseDown(2, $event)"
    />
    <div class="pane-col preview-col">
      <CodePreviewPane
        :content="store.selectedFileContent"
        :highlight-text="store.searchQuery"
        :highlight-case-sensitive="store.caseSensitive"
      />
    </div>
  </div>
</template>

<style scoped>
.app {
  display: grid;
  height: 100vh;
  min-height: 0;
  background: var(--bg-base);
  user-select: none;
}
.pane-col {
  min-width: 0;
  min-height: 0;
  overflow: hidden;
  border-right: 1px solid var(--border);
}
.pane-col:last-child {
  border-right: none;
}
.resizer {
  width: 6px;
  background: var(--border);
  cursor: col-resize;
  flex-shrink: 0;
  transition: background 0.15s;
}
.resizer:hover {
  background: var(--accent);
}
</style>
