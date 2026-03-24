<script setup>
import { ref, onMounted, inject } from 'vue'
import { Sun, Moon, Plus, X } from 'lucide-vue-next'
import { useAppStore } from '../stores/app'

const store = useAppStore()
const theme = inject('theme')
const toggleTheme = inject('toggleTheme')
const newPattern = ref('')

onMounted(() => {
  store.loadPaths()
  store.loadIgnores()
})

function handleAddIgnore() {
  if (!newPattern.value.trim()) return
  store.addIgnorePattern(newPattern.value.trim())
  newPattern.value = ''
}

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
            <Sun v-if="theme === 'dark'" :size="16" />
            <Moon v-else :size="16" />
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
    <section class="ignores-section">
      <div class="section-head">
        <span class="label">Ignored Patterns</span>
        <span class="count-badge" v-if="store.ignorePatterns.length">{{ store.ignorePatterns.length }}</span>
      </div>
      <div class="add-ignore-row">
        <div class="input-group">
          <input
            v-model="newPattern"
            type="text"
            class="input-mini"
            placeholder="Add pattern (e.g. *.log)..."
            @keydown.enter.stop="handleAddIgnore"
          />
          <button type="button" class="btn-add-inline" title="Add pattern" @click="handleAddIgnore">
            <Plus :size="14" />
          </button>
        </div>
      </div>
      <div v-if="store.ignorePatterns.length" class="ignore-tags">
        <div v-for="pat in store.ignorePatterns" :key="pat" class="tag">
          <span class="tag-text">{{ pat }}</span>
          <button
            type="button"
            class="tag-remove"
            title="Remove"
            @click="store.removeIgnorePattern(pat)"
          >
            <X :size="10" />
          </button>
        </div>
      </div>
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
      <div class="options-grid">
        <label class="option-row">
          <input
            v-model="store.caseSensitive"
            type="checkbox"
            class="checkbox"
          />
          <span class="option-label">Case sensitive</span>
        </label>
        <label class="option-row">
          <input
            v-model="store.isRegex"
            type="checkbox"
            class="checkbox"
          />
          <span class="option-label">Use Regex</span>
        </label>
      </div>
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
.ignores-section {
  flex-shrink: 0;
  padding: 12px 16px;
  border-bottom: 1px solid var(--border);
}
.count-badge {
  font-size: 10px;
  background: var(--bg-hover);
  color: var(--text-muted);
  padding: 1px 6px;
  border-radius: 10px;
  font-weight: 600;
}
.input-group {
  display: flex;
  align-items: center;
  background: var(--bg-base);
  border: 1px solid var(--border);
  border-radius: 6px;
  padding-right: 4px;
  transition: border-color 0.15s;
}
.input-group:focus-within {
  border-color: var(--accent);
}
.input-mini {
  flex: 1;
  padding: 6px 10px;
  font-size: 12px;
  background: transparent;
  border: none;
  color: var(--text);
  min-width: 0;
}
.input-mini:focus {
  outline: none;
}
.btn-add-inline {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 22px;
  height: 22px;
  background: transparent;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  color: var(--text-muted);
  transition: background 0.15s, color 0.15s;
}
.btn-add-inline:hover {
  background: var(--accent-subtle);
  color: var(--accent);
}
.ignore-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  margin-top: 8px;
  max-height: 100px;
  overflow-y: auto;
}
.tag {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 2px 8px;
  background: var(--bg-hover);
  border: 1px solid var(--border);
  border-radius: 14px;
  font-size: 11px;
}
.tag-text {
  color: var(--text);
  font-family: var(--font-mono);
  max-width: 120px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.tag-remove {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 14px;
  height: 14px;
  padding: 0;
  background: var(--bg-elevated);
  border: 1px solid var(--border);
  border-radius: 50%;
  cursor: pointer;
  color: var(--text-muted);
  transition: all 0.12s;
}
.tag-remove:hover {
  background: #fee2e2;
  color: #ef4444;
  border-color: #fecaca;
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
  color: var(--text);
  background: transparent;
  border: 1px solid var(--border);
  border-radius: 6px;
  cursor: pointer;
  transition: background 0.15s, border-color 0.15s, color 0.15s;
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
.options-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 8px;
  margin-top: 10px;
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
