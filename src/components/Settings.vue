<script setup>
import { ref, onMounted } from 'vue'
import { useSnippetStore } from '../stores/snippets'
import { open as openDialog } from '@tauri-apps/plugin-dialog'

const emit = defineEmits(['close'])
const store = useSnippetStore()
const indexDirs = ref([])
const addingDir = ref(false)

async function pickDirectory() {
  addingDir.value = true
  try {
    const selected = await openDialog({
      directory: true,
      multiple: false,
    })
    if (selected && !indexDirs.value.includes(selected)) {
      indexDirs.value = [...indexDirs.value, selected]
    }
  } finally {
    addingDir.value = false
  }
}

function removeDir(dir) {
  indexDirs.value = indexDirs.value.filter((d) => d !== dir)
}

async function startIndexing() {
  if (indexDirs.value.length === 0) return
  await store.indexDirs(indexDirs.value)
  emit('close')
}

async function exportJson() {
  try {
    const json = await store.exportSnippets()
    const blob = new Blob([json], { type: 'application/json' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `repogrep-export-${Date.now()}.json`
    a.click()
    URL.revokeObjectURL(url)
  } catch (e) {
    console.error(e)
  }
}

function close() {
  emit('close')
}
</script>

<template>
  <div class="settings-backdrop" @click.self="close">
    <div class="settings-panel" role="dialog" aria-labelledby="settings-title">
      <header class="settings-header">
        <h2 id="settings-title" class="settings-title">Settings</h2>
        <button class="icon-btn" aria-label="Close" @click="close">
          <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="6" x2="6" y2="18"/>
            <line x1="6" y1="6" x2="18" y2="18"/>
          </svg>
        </button>
      </header>
      <div class="settings-body">
        <section class="settings-section">
          <h3 class="section-title">Index directories</h3>
          <p class="section-desc">Add folders to scan for code files. Snippets will be indexed for search.</p>
          <div class="dir-list">
            <div v-for="dir in indexDirs" :key="dir" class="dir-item">
              <span class="dir-path" :title="dir">{{ dir }}</span>
              <button class="icon-btn small" aria-label="Remove" @click="removeDir(dir)">×</button>
            </div>
          </div>
          <button
            class="btn btn-secondary"
            :disabled="addingDir"
            @click="pickDirectory"
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="vertical-align: -2px; margin-right: 6px;">
              <line x1="12" y1="5" x2="12" y2="19"/>
              <line x1="5" y1="12" x2="19" y2="12"/>
            </svg>
            Add directory
          </button>
          <button
            class="btn btn-primary"
            :disabled="indexDirs.length === 0 || !!store.indexProgress"
            style="margin-left: 8px;"
            @click="startIndexing"
          >
            {{ store.indexProgress ? 'Indexing…' : 'Index now' }}
          </button>
        </section>
        <section class="settings-section">
          <h3 class="section-title">Export</h3>
          <p class="section-desc">Download all snippets as JSON.</p>
          <button class="btn btn-secondary" @click="exportJson">Export JSON</button>
        </section>
      </div>
    </div>
  </div>
</template>

<style scoped>
.settings-backdrop {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.35);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  animation: fadeIn 0.2s ease-out;
}

.settings-panel {
  background: var(--bg-elevated);
  border-radius: 12px;
  width: 90%;
  max-width: 520px;
  max-height: 85vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  box-shadow: 0 24px 48px rgba(0, 0, 0, 0.2);
  animation: slideUp 0.25s ease-out;
}

.settings-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  border-bottom: 1px solid var(--border);
}

.settings-title {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--text);
}

.settings-body {
  padding: 20px;
  overflow-y: auto;
}

.settings-section {
  margin-bottom: 24px;
}
.settings-section:last-child {
  margin-bottom: 0;
}

.section-title {
  margin: 0 0 6px;
  font-size: 14px;
  font-weight: 600;
  color: var(--text);
}

.section-desc {
  margin: 0 0 12px;
  font-size: 13px;
  color: var(--text-muted);
  line-height: 1.4;
}

.dir-list {
  margin-bottom: 12px;
  max-height: 160px;
  overflow-y: auto;
}

.dir-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 10px;
  border-radius: var(--radius);
  background: var(--bg-base);
  margin-bottom: 4px;
  font-size: 12px;
}

.dir-path {
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  color: var(--text-muted);
}

.icon-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  border: none;
  border-radius: var(--radius);
  background: transparent;
  color: var(--text-muted);
  cursor: pointer;
  transition: color 0.15s, background 0.15s;
}
.icon-btn:hover {
  color: var(--text);
  background: var(--bg-hover);
}
.icon-btn.small {
  width: 24px;
  height: 24px;
  font-size: 18px;
  line-height: 1;
}

.btn {
  padding: 8px 16px;
  border-radius: var(--radius);
  font-size: 13px;
  font-weight: 500;
  font-family: inherit;
  cursor: pointer;
  transition: opacity 0.15s, background 0.15s;
}
.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
.btn-secondary {
  border: 1px solid var(--border);
  background: transparent;
  color: var(--text);
}
.btn-secondary:hover:not(:disabled) {
  background: var(--bg-hover);
}
.btn-primary {
  border: none;
  background: var(--accent);
  color: white;
}
.btn-primary:hover:not(:disabled) {
  filter: brightness(1.1);
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

@keyframes slideUp {
  from {
    opacity: 0;
    transform: translateY(12px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>
