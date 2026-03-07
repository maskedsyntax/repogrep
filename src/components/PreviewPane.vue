<script setup>
import { computed, ref, watch } from 'vue'
import hljs from 'highlight.js'
import 'highlight.js/styles/github-dark.min.css'
import { useSnippetStore } from '../stores/snippets'

const store = useSnippetStore()
const editing = ref(false)
const editTitle = ref('')
const editCode = ref('')
const editLang = ref('')

const snippet = computed(() => store.selected)

const highlightedCode = computed(() => {
  const s = snippet.value
  if (!s?.code) return ''
  try {
    return hljs.highlight(s.code, { language: s.language || 'plaintext' }).value
  } catch {
    return hljs.highlightAuto(s.code).value
  }
})

watch(snippet, (s) => {
  editing.value = false
  if (s) {
    editTitle.value = s.title
    editCode.value = s.code
    editLang.value = s.language || 'plaintext'
  }
}, { immediate: true })

async function saveEdit() {
  if (!snippet.value) return
  await store.updateSnippet(snippet.value.id, {
    title: editTitle.value,
    code: editCode.value,
    language: editLang.value,
  })
  editing.value = false
}

function cancelEdit() {
  editing.value = false
  editTitle.value = snippet.value?.title ?? ''
  editCode.value = snippet.value?.code ?? ''
  editLang.value = snippet.value?.language ?? 'plaintext'
}

async function remove() {
  if (!snippet.value) return
  if (confirm('Delete this snippet?')) {
    await store.deleteSnippet(snippet.value.id)
  }
}
</script>

<template>
  <div class="preview-pane">
    <div v-if="!snippet" class="preview-empty">
      <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" class="empty-icon">
        <path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"/>
        <polyline points="14 2 14 8 20 8"/>
      </svg>
      <p>Select a snippet</p>
    </div>
    <template v-else>
      <header class="preview-header">
        <div v-if="!editing" class="preview-meta">
          <span class="preview-title">{{ snippet.title || 'Untitled' }}</span>
          <span class="preview-lang">{{ snippet.language }}</span>
        </div>
        <div v-else class="preview-edit-meta">
          <input v-model="editTitle" class="edit-title" placeholder="Title" />
          <input v-model="editLang" class="edit-lang" placeholder="Language" list="langs" />
          <datalist id="langs">
            <option value="plaintext"/>
            <option value="javascript"/>
            <option value="typescript"/>
            <option value="rust"/>
            <option value="python"/>
            <option value="sql"/>
            <option value="bash"/>
            <option value="json"/>
            <option value="html"/>
            <option value="css"/>
            <option value="markdown"/>
          </datalist>
        </div>
        <div class="preview-actions">
          <button v-if="!editing" class="icon-btn" title="Edit" aria-label="Edit" @click="editing = true">
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/>
              <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/>
            </svg>
          </button>
          <template v-else>
            <button class="icon-btn" title="Save" aria-label="Save" @click="saveEdit">✓</button>
            <button class="icon-btn" title="Cancel" aria-label="Cancel" @click="cancelEdit">✕</button>
          </template>
          <button class="icon-btn" title="Delete" aria-label="Delete" @click="remove">
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <polyline points="3 6 5 6 21 6"/>
              <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
              <line x1="10" y1="11" x2="10" y2="17"/>
              <line x1="14" y1="11" x2="14" y2="17"/>
            </svg>
          </button>
        </div>
      </header>
      <div class="preview-content">
        <div v-if="!editing" class="code-block">
          <pre><code class="hljs" v-html="highlightedCode" /></pre>
        </div>
        <textarea
          v-else
          v-model="editCode"
          class="code-edit"
          spellcheck="false"
        />
      </div>
    </template>
  </div>
</template>

<style scoped>
.preview-pane {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--bg-base);
  animation: fadeIn 0.25s ease-out;
}

.preview-empty {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: var(--text-muted);
}
.preview-empty .empty-icon {
  opacity: 0.5;
  margin-bottom: 12px;
}
.preview-empty p {
  margin: 0;
  font-size: 14px;
}

.preview-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  padding: 12px 16px;
  border-bottom: 1px solid var(--border);
  background: var(--bg-elevated);
}

.preview-meta {
  min-width: 0;
}
.preview-title {
  display: block;
  font-weight: 600;
  font-size: 14px;
  color: var(--text);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.preview-lang {
  font-size: 12px;
  color: var(--text-muted);
  margin-top: 2px;
}

.preview-edit-meta {
  flex: 1;
  display: flex;
  gap: 8px;
  min-width: 0;
}
.edit-title, .edit-lang {
  padding: 6px 10px;
  border: 1px solid var(--border);
  border-radius: var(--radius);
  background: var(--bg-base);
  color: var(--text);
  font-size: 13px;
  font-family: inherit;
}
.edit-title {
  flex: 1;
  min-width: 0;
}
.edit-lang {
  width: 100px;
}

.preview-actions {
  display: flex;
  gap: 4px;
}
.icon-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
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
.icon-btn:hover:last-of-type {
  color: #dc2626;
}

.preview-content {
  flex: 1;
  overflow: auto;
  padding: 16px;
}

.code-block {
  border-radius: var(--radius);
  overflow: hidden;
  background: #0d1117;
}
.code-block pre {
  margin: 0;
  padding: 16px;
  font-size: 13px;
  line-height: 1.5;
}
.code-block code {
  background: none;
  padding: 0;
}

.code-edit {
  width: 100%;
  min-height: 200px;
  padding: 16px;
  border: 1px solid var(--border);
  border-radius: var(--radius);
  background: var(--bg-elevated);
  color: var(--text);
  font-family: var(--font-mono);
  font-size: 13px;
  line-height: 1.5;
  resize: vertical;
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}
</style>

<style>
.preview-pane .hljs {
  background: transparent !important;
}
[data-theme="dark"] .code-block {
  background: #0d1117;
}
</style>
