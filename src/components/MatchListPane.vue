<script setup>
import { computed, onMounted, onUnmounted } from 'vue'
import { useVirtualList } from '@vueuse/core'
import { useAppStore } from '../stores/app'

const store = useAppStore()

const list = computed(() => store.results)

function matchCount(r) {
  return r.match_count != null ? r.match_count : (r.lines?.length ?? 0)
}

const { list: virtualList, containerProps, wrapperProps } = useVirtualList(list, {
  itemHeight: 52,
  overscan: 10,
})

function onKeydown(e) {
  if (e.key !== 'ArrowDown' && e.key !== 'ArrowUp' && e.key !== 'Enter') return
  const n = store.results.length
  if (n === 0) return
  e.preventDefault()
  if (e.key === 'ArrowDown') {
    store.selectResult(Math.min(store.selectedIndex + 1, n - 1))
  } else if (e.key === 'ArrowUp') {
    store.selectResult(Math.max(store.selectedIndex - 1, 0))
  } else if (e.key === 'Enter' && store.selectedResult) {
    store.selectResult(store.selectedIndex)
  }
}

onMounted(() => {
  window.addEventListener('keydown', onKeydown)
})
onUnmounted(() => {
  window.removeEventListener('keydown', onKeydown)
})
</script>

<template>
  <div class="pane match-list-pane">
    <header class="pane-header">
      <span class="title">Matches</span>
      <span v-if="list.length > 0" class="count">{{ list.length }} files</span>
    </header>
    <div v-if="store.loading" class="loading">
      <span class="spinner" />
      <span>Searching…</span>
    </div>
    <div v-else-if="list.length === 0" class="empty">
      <p v-if="store.searchQuery">No files contain this snippet.</p>
      <p v-else>Paste a snippet and press Search.</p>
    </div>
    <div v-else v-bind="containerProps" class="list-container">
      <div v-bind="wrapperProps" class="list-wrapper">
        <div
          v-for="{ data: r, index } in virtualList"
          :key="r.file_path"
          class="list-item"
          :class="{ active: store.selectedIndex === index }"
          @click="store.selectResult(index)"
        >
          <span class="root-hint">{{ r.root_hint }}</span>
          <span class="relative-path">{{ r.relative_path }}</span>
          <span class="line-info">{{ matchCount(r) }} match{{ matchCount(r) !== 1 ? 'es' : '' }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.pane {
  display: flex;
  flex-direction: column;
  min-width: 0;
  height: 100%;
}
.pane-header {
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
  padding: 12px 16px;
  border-bottom: 1px solid var(--border);
}
.title {
  font-size: 11px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--text-muted);
}
.count {
  font-size: 12px;
  color: var(--text-muted);
}
.loading,
.empty {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  padding: 24px;
  font-size: 13px;
  color: var(--text-muted);
}
.spinner {
  width: 18px;
  height: 18px;
  border: 2px solid var(--border);
  border-top-color: var(--accent);
  border-radius: 50%;
  animation: spin 0.7s linear infinite;
}
@keyframes spin {
  to { transform: rotate(360deg); }
}
.list-container {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
}
.list-wrapper {
  position: relative;
  width: 100%;
}
.list-item {
  display: flex;
  flex-direction: column;
  gap: 2px;
  padding: 10px 16px;
  min-height: 52px;
  box-sizing: border-box;
  cursor: pointer;
  border-left: 3px solid transparent;
  transition: background 0.12s, border-color 0.12s;
}
.list-item:hover {
  background: var(--bg-hover);
}
.list-item.active {
  background: var(--accent-subtle);
  border-left-color: var(--accent);
}
.root-hint {
  font-size: 11px;
  font-weight: 600;
  color: var(--accent);
  text-transform: uppercase;
  letter-spacing: 0.03em;
}
.relative-path {
  font-size: 12px;
  color: var(--text);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.line-info {
  font-size: 11px;
  color: var(--text-muted);
}
</style>
