<script setup>
import { ref, watch, nextTick, computed } from 'vue'
import hljs from 'highlight.js/lib/core'
import javascript from 'highlight.js/lib/languages/javascript'
import typescript from 'highlight.js/lib/languages/typescript'
import rust from 'highlight.js/lib/languages/rust'
import python from 'highlight.js/lib/languages/python'
import vue from 'highlight.js/lib/languages/xml'
import json from 'highlight.js/lib/languages/json'
import bash from 'highlight.js/lib/languages/bash'
import css from 'highlight.js/lib/languages/css'
import xml from 'highlight.js/lib/languages/xml'
import markdown from 'highlight.js/lib/languages/markdown'
import dart from 'highlight.js/lib/languages/dart'
import 'highlight.js/styles/github.css'
import { useAppStore } from '../stores/app'

hljs.registerLanguage('javascript', javascript)
hljs.registerLanguage('typescript', typescript)
hljs.registerLanguage('rust', rust)
hljs.registerLanguage('python', python)
hljs.registerLanguage('vue', vue)
hljs.registerLanguage('json', json)
hljs.registerLanguage('bash', bash)
hljs.registerLanguage('css', css)
hljs.registerLanguage('html', xml)
hljs.registerLanguage('xml', xml)
hljs.registerLanguage('markdown', markdown)
hljs.registerLanguage('dart', dart)

const props = defineProps({
  content: { type: String, default: '' },
  highlightText: { type: String, default: '' },
})

const store = useAppStore()
const preRef = ref(null)

const lang = computed(() => {
  const path = store.selectedFilePath
  const ext = path?.split('.').pop()?.toLowerCase() ?? ''
  const map = { rs: 'rust', vue: 'vue', js: 'javascript', ts: 'typescript', dart: 'dart', py: 'python', json: 'json', sh: 'bash', md: 'markdown', html: 'html', css: 'css' }
  return map[ext] || 'plaintext'
})

const highlighted = computed(() => {
  const c = props.content
  if (!c) return ''
  try {
    const l = lang.value
    if (l && l !== 'plaintext' && hljs.getLanguage(l)) {
      return hljs.highlight(c, { language: l }).value
    }
  } catch (_) {}
  return escapeHtml(c)
})

function escapeHtml(s) {
  const div = document.createElement('div')
  div.textContent = s
  return div.innerHTML
}

function highlightQuery(html) {
  const q = props.highlightText?.trim()
  if (!q) return html
  const esc = q.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
  const re = new RegExp(`(${esc})`, 'gi')
  return html.replace(re, '<mark class="search-hit">$1</mark>')
}

const displayHtml = computed(() => highlightQuery(highlighted.value))

watch([() => props.content, () => props.highlightText], () => {
  nextTick(scrollToFirstMatch)
})

function scrollToFirstMatch() {
  const el = preRef.value
  if (!el) return
  const mark = el.querySelector('.search-hit')
  if (mark) mark.scrollIntoView({ block: 'center', behavior: 'smooth' })
}
</script>

<template>
  <div class="pane code-preview-pane">
    <header class="pane-header">
      <span class="title">Preview</span>
      <span v-if="store.selectedFilePath" class="file-path">{{ store.selectedFilePath }}</span>
    </header>
    <div class="pre-wrap">
      <pre ref="preRef" class="code-block"><code class="hljs" v-html="displayHtml"></code></pre>
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
.file-path {
  flex: 1;
  min-width: 0;
  font-size: 12px;
  color: var(--text-muted);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.pre-wrap {
  flex: 1;
  overflow: auto;
  padding: 16px;
}
.code-block {
  margin: 0;
  padding: 0;
  font-family: var(--font-mono);
  font-size: 13px;
  line-height: 1.5;
  background: transparent !important;
}
.code-block :deep(.search-hit) {
  background: rgba(255, 212, 0, 0.35);
  border-radius: 2px;
  padding: 0 2px;
}
</style>
