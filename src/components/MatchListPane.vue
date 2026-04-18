<script setup>
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { useVirtualList } from '@vueuse/core'
import { useAppStore } from '../stores/app'

const store = useAppStore()

const list = computed(() => store.results)
const progressPercent = computed(() => {
  const total = store.searchProgressTotal || 0
  const processed = store.searchProgressProcessed || 0
  if (!total) return 0
  return Math.min(100, Math.round((processed / total) * 100))
})
const hasSearchStats = computed(() => !store.loading && !!store.searchQuery && store.searchDurationMs > 0)
const collapsedFolders = ref({})

function matchCount(r) {
  return r.match_count != null ? r.match_count : (r.lines?.length ?? 0)
}

const { list: virtualList, containerProps, wrapperProps } = useVirtualList(list, {
  itemHeight: 88,
  overscan: 10,
})
const treeRows = computed(() => {
  const byRoot = new Map()
  for (const r of list.value) {
    const root = r.root_hint || 'root'
    if (!byRoot.has(root)) byRoot.set(root, [])
    byRoot.get(root).push(r)
  }

  const roots = []
  const rootEntries = Array.from(byRoot.entries()).sort((a, b) => a[0].localeCompare(b[0]))
  for (const [root, files] of rootEntries) {
    const rootNode = { name: root, path: root, type: 'root', children: [], file: null }
    const folders = new Map()

    for (const f of files) {
      const parts = (f.relative_path || '').split('/').filter(Boolean)
      let parentPath = root
      let parentNode = rootNode

      for (let i = 0; i < parts.length; i += 1) {
        const part = parts[i]
        const isFile = i === parts.length - 1
        const nodePath = `${parentPath}/${part}`
        if (isFile) {
          parentNode.children.push({
            name: part,
            path: nodePath,
            type: 'file',
            children: [],
            file: f,
          })
        } else {
          let folderNode = folders.get(nodePath)
          if (!folderNode) {
            folderNode = {
              name: part,
              path: nodePath,
              type: 'folder',
              children: [],
              file: null,
            }
            folders.set(nodePath, folderNode)
            parentNode.children.push(folderNode)
          }
          parentNode = folderNode
          parentPath = nodePath
        }
      }
    }

    const sortNode = (node) => {
      node.children.sort((a, b) => {
        if (a.type === b.type) return a.name.localeCompare(b.name)
        if (a.type === 'file') return 1
        if (b.type === 'file') return -1
        return 0
      })
      node.children.forEach(sortNode)
    }
    sortNode(rootNode)
    roots.push(rootNode)
  }

  const rows = []
  const walk = (node, depth) => {
    rows.push({ node, depth })
    if (node.type === 'file') return
    if (collapsedFolders.value[node.path]) return
    for (const child of node.children) walk(child, depth + 1)
  }
  for (const rootNode of roots) walk(rootNode, 0)
  return rows
})

function toggleFolder(path) {
  collapsedFolders.value[path] = !collapsedFolders.value[path]
}

function hasChildren(row) {
  return row.node.type !== 'file' && row.node.children.length > 0
}

function isEditableTarget(target) {
  if (!target || target.nodeType !== Node.ELEMENT_NODE) return false
  const el = target
  const tag = el.tagName?.toLowerCase()
  if (tag === 'textarea' || tag === 'select') return true
  if (tag === 'input') {
    const type = (el.type || 'text').toLowerCase()
    if (['button', 'checkbox', 'radio', 'submit', 'reset', 'file', 'hidden'].includes(type)) return false
    return true
  }
  return el.isContentEditable === true
}

function onKeydown(e) {
  if (e.key !== 'ArrowDown' && e.key !== 'ArrowUp' && e.key !== 'Enter') return
  if (isEditableTarget(e.target)) return
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
      <div class="header-right">
        <span v-if="list.length > 0" class="count">{{ list.length }} files</span>
        <span v-if="hasSearchStats" class="stats">
          {{ store.lastScannedFiles }} scanned in {{ store.searchDurationMs }}ms
        </span>
        <button
          type="button"
          class="btn-head"
          :disabled="list.length === 0"
          @click="store.toggleTreeView"
        >
          {{ store.treeView ? 'List' : 'Tree' }}
        </button>
        <button
          type="button"
          class="btn-head"
          :disabled="list.length === 0"
          @click="store.exportResultsAsJson"
        >
          JSON
        </button>
        <button
          type="button"
          class="btn-head"
          :disabled="list.length === 0"
          @click="store.exportResultsAsCsv"
        >
          CSV
        </button>
      </div>
    </header>
    <div v-if="store.loading" class="loading">
      <span class="spinner" />
      <div class="loading-copy">
        <span>Searching…</span>
        <span v-if="store.searchProgressTotal > 0" class="progress-text">
          {{ store.searchProgressProcessed }}/{{ store.searchProgressTotal }} files ({{ progressPercent }}%)
        </span>
      </div>
    </div>
    <div v-else-if="list.length === 0" class="empty">
      <p v-if="store.searchQuery">No files contain this snippet.</p>
      <p v-else>Paste a snippet and press Search.</p>
    </div>
    <div v-else-if="!store.treeView" v-bind="containerProps" class="list-container">
      <div v-bind="wrapperProps" class="list-wrapper">
        <div
          v-for="{ data: r, index } in virtualList"
          :key="r.file_path"
          class="list-item"
          :class="{ active: store.selectedIndex === index }"
          @click="store.selectResult(index)"
        >
          <div class="list-item-main">
            <span class="root-hint">{{ r.root_hint }}</span>
            <span class="relative-path">{{ r.relative_path }}</span>
            <span class="line-info">{{ matchCount(r) }} match{{ matchCount(r) !== 1 ? 'es' : '' }}</span>
          </div>
          <div v-if="r.context" class="context-snippets">
            <div
              v-for="ctx in r.context.slice(0, 3)"
              :key="ctx.line_number"
              class="context-line"
              :class="{ 'context-match': ctx.is_match }"
            >
              <span class="context-num">{{ ctx.line_number }}</span>
              <span class="context-txt">{{ ctx.content }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
    <div v-else class="tree-container">
      <div
        v-for="row in treeRows"
        :key="row.node.path"
        class="tree-row"
        :class="{
          'tree-root-row': row.node.type === 'root',
          'tree-folder-row': row.node.type === 'folder',
          active: row.node.type === 'file' && store.selectedResult?.file_path === row.node.file.file_path,
        }"
        :style="{ paddingLeft: `${16 + row.depth * 14}px` }"
        @click="row.node.type === 'file' ? store.selectResultByFilePath(row.node.file.file_path) : hasChildren(row) ? toggleFolder(row.node.path) : null"
      >
        <span v-if="hasChildren(row)" class="tree-caret">{{ collapsedFolders[row.node.path] ? '▸' : '▾' }}</span>
        <span v-else class="tree-caret tree-caret-empty">·</span>
        <span class="tree-kind" :class="`tree-kind-${row.node.type}`">
          {{ row.node.type === 'file' ? '•' : row.node.type === 'folder' ? '▣' : '◆' }}
        </span>
        <span class="tree-name">{{ row.node.name }}</span>
        <span v-if="row.node.type === 'file'" class="line-info">
          {{ matchCount(row.node.file) }} match{{ matchCount(row.node.file) !== 1 ? 'es' : '' }}
        </span>
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
.header-right {
  display: inline-flex;
  align-items: center;
  gap: 10px;
}
.stats {
  font-size: 11px;
  color: var(--text-muted);
}
.btn-head {
  border: 1px solid var(--border);
  background: var(--bg-elevated);
  color: var(--text-muted);
  border-radius: 6px;
  font-size: 11px;
  padding: 2px 8px;
  cursor: pointer;
}
.btn-head:disabled {
  opacity: 0.45;
  cursor: not-allowed;
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
.loading-copy {
  display: flex;
  flex-direction: column;
  gap: 2px;
}
.progress-text {
  font-size: 11px;
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
.tree-container {
  flex: 1;
  overflow: auto;
  padding: 8px 0 12px;
}
.tree-row {
  display: flex;
  align-items: center;
  gap: 7px;
  padding-top: 6px;
  padding-bottom: 6px;
  padding-right: 12px;
  cursor: pointer;
  border-left: 3px solid transparent;
  border-radius: 0 8px 8px 0;
}
.tree-row:hover {
  background: color-mix(in srgb, var(--bg-hover) 72%, transparent);
}
.tree-root-row {
  color: var(--accent);
  font-size: 11px;
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 0.04em;
}
.tree-folder-row {
  color: var(--text);
  font-size: 12px;
}
.tree-row.active {
  background: color-mix(in srgb, var(--accent-subtle) 85%, transparent);
  border-left: 3px solid var(--accent);
}
.tree-caret {
  width: 12px;
  text-align: center;
  color: var(--text-muted);
  font-size: 10px;
}
.tree-caret-empty {
  color: color-mix(in srgb, var(--text-muted) 50%, transparent);
}
.tree-kind {
  width: 12px;
  text-align: center;
  font-size: 9px;
}
.tree-kind-root {
  color: var(--accent);
}
.tree-kind-folder {
  color: color-mix(in srgb, var(--text-muted) 75%, var(--accent) 25%);
}
.tree-kind-file {
  color: color-mix(in srgb, var(--text-muted) 70%, transparent);
}
.tree-name {
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: 12px;
}
.list-wrapper {
  position: relative;
  width: 100%;
}
.list-item {
  display: flex;
  flex-direction: column;
  padding: 10px 16px;
  min-height: 88px;
  box-sizing: border-box;
  cursor: pointer;
  border-left: 3px solid transparent;
  transition: background 0.12s, border-color 0.12s;
  overflow: hidden;
}
.list-item-main {
  display: flex;
  flex-direction: column;
  gap: 2px;
  margin-bottom: 6px;
}
.context-snippets {
  display: flex;
  flex-direction: column;
  gap: 2px;
  opacity: 0.85;
}
.context-line {
  display: flex;
  gap: 8px;
  font-family: var(--font-mono);
  font-size: 10px;
  line-height: 1.2;
  white-space: pre;
  color: var(--text-muted);
}
.context-match {
  color: var(--text);
  font-weight: 500;
  background: color-mix(in srgb, var(--accent-subtle) 40%, transparent);
}
.context-num {
  width: 24px;
  text-align: right;
  flex-shrink: 0;
  color: color-mix(in srgb, var(--text-muted) 50%, transparent);
}
.context-txt {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
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
