<script setup>
import { onMounted, inject } from 'vue'
import { useAppStore } from '../stores/app'

const store = useAppStore()
const theme = inject('theme')
const toggleTheme = inject('toggleTheme')

onMounted(() => store.loadPaths())

function onKeydown(e) {
  if (e.key === 'Enter' && !e.shiftKey) {
    e.preventDefault()
    store.search()
  }
}
</script>

<template>
  <div class="pane snippet-input-pane">
    <section class="projects-section">
      <div class="section-head">
        <span class="label">Projects</span>
        <div class="section-actions">
          <button
            type="button"
            class="btn-icon"
            :title="theme === 'dark' ? 'Light mode' : 'Dark mode'"
            @click="toggleTheme"
          >
            <span v-if="theme === 'dark'" aria-hidden="true">☀️</span>
            <span v-else aria-hidden="true">🌙</span>
          </button>
          <button type="button" class="btn-add" @click="store.openFolderPicker">
            Add folder
          </button>
        </div>
      </div>
      <ul v-if="store.projectPaths.length" class="path-list">
        <li v-for="p in store.projectPaths" :key="p.path" class="path-item">
          <span class="path-hint">{{ p.root_hint || p.path }}</span>
          <button
            type="button"
            class="btn-remove"
            title="Remove"
            @click="store.removeProjectPath(p.path)"
          >
            ×
          </button>
        </li>
      </ul>
      <p v-else class="hint">No projects. Add a folder to search.</p>
    </section>
    <section class="search-section">
      <label class="label">Snippet to search</label>
      <textarea
        v-model="store.searchQuery"
        class="textarea"
        placeholder="Paste code snippet, then press Enter or click Search"
        rows="8"
        @keydown="onKeydown"
      />
      <label class="option-row">
        <input
          v-model="store.caseSensitive"
          type="checkbox"
          class="checkbox"
        />
        <span class="option-label">Case sensitive</span>
      </label>
      <button type="button" class="btn-search" @click="store.search">
        Search
      </button>
    </section>
  </div>
</template>

<style scoped>
.pane {
  display: flex;
  flex-direction: column;
  min-width: 0;
  height: 100%;
}
.projects-section {
  flex-shrink: 0;
  padding: 12px 16px;
  border-bottom: 1px solid var(--border);
}
.section-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
  margin-bottom: 8px;
}
.section-actions {
  display: flex;
  align-items: center;
  gap: 6px;
}
.btn-icon {
  width: 32px;
  height: 28px;
  padding: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 16px;
  background: transparent;
  border: 1px solid var(--border);
  border-radius: 6px;
  cursor: pointer;
  transition: background 0.15s, border-color 0.15s;
}
.btn-icon:hover {
  background: var(--bg-hover);
  border-color: var(--text-muted);
}
.label {
  font-size: 11px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--text-muted);
}
.btn-add {
  font-size: 12px;
  padding: 4px 10px;
  color: var(--accent);
  background: transparent;
  border: 1px solid var(--border);
  border-radius: 6px;
  cursor: pointer;
  transition: background 0.15s, border-color 0.15s;
}
.btn-add:hover {
  background: var(--bg-hover);
  border-color: var(--accent);
}
.path-list {
  list-style: none;
  margin: 0;
  padding: 0;
  max-height: 120px;
  overflow-y: auto;
}
.path-item {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 4px 0;
  font-size: 12px;
}
.path-hint {
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  color: var(--text);
}
.btn-remove {
  flex-shrink: 0;
  width: 20px;
  height: 20px;
  padding: 0;
  font-size: 16px;
  line-height: 1;
  color: var(--text-muted);
  background: none;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  transition: color 0.15s, background 0.15s;
}
.btn-remove:hover {
  color: var(--text);
  background: var(--bg-hover);
}
.hint {
  margin: 0;
  font-size: 12px;
  color: var(--text-muted);
}
.search-section {
  flex: 1;
  display: flex;
  flex-direction: column;
  padding: 16px;
  min-height: 0;
}
.search-section .label {
  margin-bottom: 8px;
}
.textarea {
  flex: 1;
  min-height: 120px;
  padding: 12px;
  font-family: var(--font-mono);
  font-size: 13px;
  line-height: 1.5;
  color: var(--text);
  background: var(--bg-elevated);
  border: 1px solid var(--border);
  border-radius: 8px;
  resize: none;
  transition: border-color 0.15s;
}
.textarea:focus {
  outline: none;
  border-color: var(--accent);
}
.textarea::placeholder {
  color: var(--text-muted);
}
.option-row {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 10px;
  font-size: 13px;
  color: var(--text-muted);
  cursor: pointer;
}
.checkbox {
  width: 16px;
  height: 16px;
  accent-color: var(--accent);
  cursor: pointer;
}
.option-label {
  user-select: none;
}
.btn-search {
  margin-top: 12px;
  padding: 10px 20px;
  font-size: 13px;
  font-weight: 600;
  color: white;
  background: var(--accent);
  border: none;
  border-radius: 8px;
  cursor: pointer;
  transition: opacity 0.15s, transform 0.1s;
}
.btn-search:hover {
  opacity: 0.95;
}
.btn-search:active {
  transform: scale(0.98);
}
</style>
