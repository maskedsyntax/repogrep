<script setup>
import { computed } from 'vue'
import { useSnippetStore } from '../stores/snippets'

const store = useSnippetStore()

const list = computed(() => store.snippets)

function langIcon(lang) {
  const icons = {
    rust: '🦀',
    javascript: '📜',
    typescript: '📘',
    vue: '💚',
    python: '🐍',
    java: '☕',
    go: '🐹',
    sql: '🗄️',
    bash: '💻',
    html: '🌐',
    css: '🎨',
    json: '📋',
    markdown: '📝',
  }
  return icons[lang?.toLowerCase()] || '📄'
}
</script>

<template>
  <div class="snippet-list">
    <div v-if="store.loading" class="list-loading">
      <span class="spinner" aria-hidden="true" />
      <span>Searching…</span>
    </div>
    <div v-else-if="list.length === 0" class="list-empty">
      <svg xmlns="http://www.w3.org/2000/svg" width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" class="empty-icon">
        <path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"/>
        <polyline points="14 2 14 8 20 8"/>
        <line x1="16" y1="13" x2="8" y2="13"/>
        <line x1="16" y1="17" x2="8" y2="17"/>
      </svg>
      <p>No snippets yet</p>
      <p class="hint">Add directories in Settings or paste from clipboard</p>
    </div>
    <ul v-else class="list" role="list">
      <li
        v-for="s in list"
        :key="s.id"
        class="list-item"
        :class="{ active: store.selected?.id === s.id }"
        @click="store.select(s.id)"
      >
        <span class="item-lang" :title="s.language">{{ langIcon(s.language) }}</span>
        <div class="item-body">
          <span class="item-title">{{ s.title || 'Untitled' }}</span>
          <div v-if="s.tags?.length" class="item-tags">
            <span v-for="t in s.tags.slice(0, 3)" :key="t" class="tag">{{ t }}</span>
          </div>
        </div>
      </li>
    </ul>
  </div>
</template>

<style scoped>
.snippet-list {
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
}

.list-loading {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  padding: 24px;
  color: var(--text-muted);
  font-size: 13px;
}

.spinner {
  width: 20px;
  height: 20px;
  border: 2px solid var(--border);
  border-top-color: var(--accent);
  border-radius: 50%;
  animation: spin 0.7s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.list-empty {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 24px;
  text-align: center;
  color: var(--text-muted);
  animation: fadeIn 0.3s ease-out;
}
.list-empty .empty-icon {
  margin-bottom: 12px;
  opacity: 0.6;
}
.list-empty p {
  margin: 0;
  font-size: 14px;
}
.list-empty .hint {
  margin-top: 4px;
  font-size: 12px;
  opacity: 0.8;
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

.list {
  list-style: none;
  margin: 0;
  padding: 4px 0;
}

.list-item {
  display: flex;
  align-items: flex-start;
  gap: 10px;
  padding: 10px 14px;
  cursor: pointer;
  border-left: 3px solid transparent;
  transition: background 0.15s, border-color 0.15s;
  animation: itemIn 0.2s ease-out backwards;
}
.list-item:hover {
  background: var(--bg-hover);
}
.list-item.active {
  background: var(--accent-subtle);
  border-left-color: var(--accent);
}

.item-lang {
  font-size: 16px;
  flex-shrink: 0;
  line-height: 1.2;
}

.item-body {
  min-width: 0;
  flex: 1;
}

.item-title {
  display: block;
  font-size: 13px;
  font-weight: 500;
  color: var(--text);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.item-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
  margin-top: 4px;
}

.tag {
  font-size: 10px;
  padding: 2px 6px;
  border-radius: 4px;
  background: var(--bg-hover);
  color: var(--text-muted);
}

.list-item:nth-child(1) { animation-delay: 0.02s; }
.list-item:nth-child(2) { animation-delay: 0.04s; }
.list-item:nth-child(3) { animation-delay: 0.06s; }
.list-item:nth-child(4) { animation-delay: 0.08s; }
.list-item:nth-child(5) { animation-delay: 0.1s; }
.list-item:nth-child(n+6) { animation-delay: 0.12s; }

@keyframes itemIn {
  from {
    opacity: 0;
    transform: translateX(-6px);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}
</style>
