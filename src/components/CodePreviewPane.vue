<script setup>
import { ref, watch, nextTick, computed, onMounted, onUnmounted } from 'vue'
import hljs from 'highlight.js/lib/core'
import javascript from 'highlight.js/lib/languages/javascript'
import typescript from 'highlight.js/lib/languages/typescript'
import rust from 'highlight.js/lib/languages/rust'
import python from 'highlight.js/lib/languages/python'
import xml from 'highlight.js/lib/languages/xml'
import json from 'highlight.js/lib/languages/json'
import bash from 'highlight.js/lib/languages/bash'
import css from 'highlight.js/lib/languages/css'
import markdown from 'highlight.js/lib/languages/markdown'
import dart from 'highlight.js/lib/languages/dart'
import swift from 'highlight.js/lib/languages/swift'
import cpp from 'highlight.js/lib/languages/cpp'
import c from 'highlight.js/lib/languages/c'
import csharp from 'highlight.js/lib/languages/csharp'
import scala from 'highlight.js/lib/languages/scala'
import go from 'highlight.js/lib/languages/go'
import java from 'highlight.js/lib/languages/java'
import kotlin from 'highlight.js/lib/languages/kotlin'
import ruby from 'highlight.js/lib/languages/ruby'
import sql from 'highlight.js/lib/languages/sql'
import r from 'highlight.js/lib/languages/r'
import php from 'highlight.js/lib/languages/php'
import scss from 'highlight.js/lib/languages/scss'
import less from 'highlight.js/lib/languages/less'
import yaml from 'highlight.js/lib/languages/yaml'
import lua from 'highlight.js/lib/languages/lua'
import erlang from 'highlight.js/lib/languages/erlang'
import haskell from 'highlight.js/lib/languages/haskell'
import fsharp from 'highlight.js/lib/languages/fsharp'
import elixir from 'highlight.js/lib/languages/elixir'
import ocaml from 'highlight.js/lib/languages/ocaml'
import vim from 'highlight.js/lib/languages/vim'
import lisp from 'highlight.js/lib/languages/lisp'
import 'highlight.js/styles/github.css'
import { useAppStore } from '../stores/app'

hljs.registerLanguage('javascript', javascript)
hljs.registerLanguage('typescript', typescript)
hljs.registerLanguage('rust', rust)
hljs.registerLanguage('python', python)
hljs.registerLanguage('xml', xml)
hljs.registerLanguage('json', json)
hljs.registerLanguage('bash', bash)
hljs.registerLanguage('css', css)
hljs.registerLanguage('markdown', markdown)
hljs.registerLanguage('dart', dart)
hljs.registerLanguage('swift', swift)
hljs.registerLanguage('cpp', cpp)
hljs.registerLanguage('c', c)
hljs.registerLanguage('csharp', csharp)
hljs.registerLanguage('scala', scala)
hljs.registerLanguage('go', go)
hljs.registerLanguage('java', java)
hljs.registerLanguage('kotlin', kotlin)
hljs.registerLanguage('ruby', ruby)
hljs.registerLanguage('sql', sql)
hljs.registerLanguage('r', r)
hljs.registerLanguage('php', php)
hljs.registerLanguage('scss', scss)
hljs.registerLanguage('less', less)
hljs.registerLanguage('yaml', yaml)
hljs.registerLanguage('lua', lua)
hljs.registerLanguage('erlang', erlang)
hljs.registerLanguage('haskell', haskell)
hljs.registerLanguage('fsharp', fsharp)
hljs.registerLanguage('elixir', elixir)
hljs.registerLanguage('ocaml', ocaml)
hljs.registerLanguage('vim', vim)
hljs.registerLanguage('lisp', lisp)

const props = defineProps({
  content: { type: String, default: '' },
  highlightText: { type: String, default: '' },
  highlightCaseSensitive: { type: Boolean, default: false },
  highlightIsRegex: { type: Boolean, default: false },
})

const store = useAppStore()
const preRef = ref(null)
const codeRef = ref(null)
const currentMatchIndex = ref(-1)
const matchCount = ref(0)
const lineCount = computed(() => {
  if (!props.content) return 1
  return props.content.split('\n').length
})
const lineNumberItems = computed(() => Array.from({ length: lineCount.value }, (_, i) => i + 1))

// Extension → Highlight.js language (covers all CODE_EXTENSIONS from backend + common extras)
const EXT_LANG = {
  rs: 'rust', vue: 'xml', js: 'javascript', ts: 'typescript', jsx: 'javascript', tsx: 'typescript',
  mjs: 'javascript', cjs: 'javascript', dart: 'dart', py: 'python', go: 'go', rb: 'ruby',
  java: 'java', kt: 'kotlin', kts: 'kotlin', c: 'c', h: 'c', cpp: 'cpp', hpp: 'cpp', cc: 'cpp', cxx: 'cpp',
  cs: 'csharp', php: 'php', swift: 'swift', scala: 'scala', r: 'r', sql: 'sql', sh: 'bash', bash: 'bash', zsh: 'bash',
  html: 'xml', htm: 'xml', css: 'css', scss: 'scss', sass: 'scss', less: 'less',
  json: 'json', yaml: 'yaml', yml: 'yaml', toml: 'yaml', xml: 'xml', md: 'markdown', markdown: 'markdown',
  lua: 'lua', vim: 'vim', el: 'lisp', ex: 'elixir', exs: 'elixir', erl: 'erlang', hs: 'haskell', fs: 'fsharp', fsx: 'fsharp', ml: 'ocaml', mli: 'ocaml',
}
const lang = computed(() => {
  const path = store.selectedFilePath
  const ext = path?.split('.').pop()?.toLowerCase() ?? ''
  return EXT_LANG[ext] || 'plaintext'
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

const displayHtml = computed(() => highlighted.value)

function getQueryRegex() {
  const q = props.highlightText?.trim()
  if (!q) return null
  let pattern = q
  if (!props.highlightIsRegex) {
    pattern = q.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
  }
  const flags = props.highlightCaseSensitive ? 'g' : 'gi'
  try {
    return new RegExp(pattern, flags)
  } catch (_) {
    return null
  }
}

function applyDomHighlighting() {
  const root = codeRef.value
  if (!root) return

  // Clear any existing highlights
  const existing = root.querySelectorAll('.search-hit')
  existing.forEach((el) => {
    const parent = el.parentNode
    if (!parent) return
    while (el.firstChild) parent.insertBefore(el.firstChild, el)
    parent.removeChild(el)
    parent.normalize()
  })

  const re = getQueryRegex()
  if (!re) {
    matchCount.value = 0
    currentMatchIndex.value = -1
    return
  }

  // Walk text nodes and build a single searchable string so phrase queries
  // (e.g. "import Instruction") can match across syntax-highlight spans.
  const walker = document.createTreeWalker(root, NodeFilter.SHOW_TEXT, null)
  const textNodes = []
  let node = null
  while ((node = walker.nextNode())) {
    if (typeof node.nodeValue === 'string') textNodes.push(node)
  }
  if (!textNodes.length) {
    matchCount.value = 0
    currentMatchIndex.value = -1
    return
  }

  const segments = []
  let fullText = ''
  for (const textNode of textNodes) {
    const text = textNode.nodeValue || ''
    const start = fullText.length
    fullText += text
    segments.push({ node: textNode, start, end: fullText.length })
  }

  function locatePosition(pos) {
    for (const seg of segments) {
      if (pos >= seg.start && pos <= seg.end) {
        return { node: seg.node, offset: pos - seg.start }
      }
    }
    const last = segments[segments.length - 1]
    return { node: last.node, offset: (last.node.nodeValue || '').length }
  }

  re.lastIndex = 0
  const ranges = []
  let m
  while ((m = re.exec(fullText)) && m[0]) {
    ranges.push({ start: m.index, end: m.index + m[0].length })
    if (re.lastIndex === m.index) re.lastIndex += 1
  }
  if (!ranges.length) {
    matchCount.value = 0
    currentMatchIndex.value = -1
    return
  }

  // Apply from the end so earlier offsets stay valid.
  for (let i = ranges.length - 1; i >= 0; i -= 1) {
    const { start, end } = ranges[i]
    const startPos = locatePosition(start)
    const endPos = locatePosition(end)
    const range = document.createRange()
    range.setStart(startPos.node, startPos.offset)
    range.setEnd(endPos.node, endPos.offset)
    const mark = document.createElement('mark')
    mark.className = 'search-hit'
    const fragment = range.extractContents()
    mark.appendChild(fragment)
    range.insertNode(mark)
  }

  const marks = Array.from(root.querySelectorAll('.search-hit'))
  matchCount.value = marks.length
  if (!marks.length) {
    currentMatchIndex.value = -1
    return
  }
  currentMatchIndex.value = 0
  syncActiveMatch(marks)
}

watch([() => props.content, () => props.highlightText], () => {
  nextTick(() => {
    applyDomHighlighting()
    scrollToCurrentMatch()
    store.setPreviewMatchState(currentMatchIndex.value, matchCount.value)
  })
})

function syncActiveMatch(existingMarks = null) {
  const marks = existingMarks || Array.from(codeRef.value?.querySelectorAll('.search-hit') ?? [])
  marks.forEach((mark, idx) => {
    mark.classList.toggle('search-hit-current', idx === currentMatchIndex.value)
  })
}

function scrollToCurrentMatch() {
  const el = preRef.value
  if (!el) return
  const marks = Array.from(el.querySelectorAll('.search-hit'))
  if (!marks.length || currentMatchIndex.value < 0) return
  const mark = marks[currentMatchIndex.value]
  if (mark) mark.scrollIntoView({ block: 'center', behavior: 'smooth' })
}

function jumpMatch(step) {
  if (matchCount.value <= 0) return
  currentMatchIndex.value = (currentMatchIndex.value + step + matchCount.value) % matchCount.value
  syncActiveMatch()
  scrollToCurrentMatch()
  store.setPreviewMatchState(currentMatchIndex.value, matchCount.value)
}

function isEditableTarget(target) {
  if (!target || target.nodeType !== Node.ELEMENT_NODE) return false
  const el = target
  const tag = el.tagName?.toLowerCase()
  if (tag === 'textarea' || tag === 'select') return true
  if (tag === 'input') {
    const type = (el.type || 'text').toLowerCase()
    return !['button', 'checkbox', 'radio', 'submit', 'reset', 'file', 'hidden'].includes(type)
  }
  return el.isContentEditable === true
}

function onKeydown(e) {
  if (e.key !== 'F3') return
  if (isEditableTarget(e.target)) return
  e.preventDefault()
  jumpMatch(e.shiftKey ? -1 : 1)
}

onMounted(() => {
  window.addEventListener('keydown', onKeydown)
})
onUnmounted(() => {
  window.removeEventListener('keydown', onKeydown)
})
</script>

<template>
  <div class="pane code-preview-pane">
    <header class="pane-header">
      <span class="title">Preview</span>
      <span v-if="store.selectedFilePath" class="file-path">{{ store.selectedFilePath }}</span>
      <div class="header-actions">
        <button
          type="button"
          class="btn-action"
          :disabled="!store.selectedFilePath"
          @click="store.openSelectedFileInEditor"
        >
          Open File
        </button>
        <button type="button" class="btn-action" title="Previous match (Shift+F3)" :disabled="matchCount <= 0" @click="jumpMatch(-1)">Prev</button>
        <span class="match-meta">{{ matchCount > 0 ? `${currentMatchIndex + 1}/${matchCount}` : '0/0' }}</span>
        <button type="button" class="btn-action" title="Next match (F3)" :disabled="matchCount <= 0" @click="jumpMatch(1)">Next</button>
      </div>
    </header>
    <div class="pre-wrap">
      <div class="line-gutter" aria-hidden="true">
        <span v-for="n in lineNumberItems" :key="n" class="line-number">{{ n }}</span>
      </div>
      <pre ref="preRef" class="code-block"><code ref="codeRef" class="hljs" v-html="displayHtml"></code></pre>
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
.header-actions {
  display: inline-flex;
  align-items: center;
  gap: 8px;
}
.btn-action {
  border: 1px solid var(--border);
  background: var(--bg-elevated);
  color: var(--text);
  border-radius: 6px;
  padding: 3px 8px;
  font-size: 11px;
  cursor: pointer;
}
.btn-action:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
.match-meta {
  min-width: 34px;
  font-size: 11px;
  color: var(--text-muted);
  text-align: center;
}
.pre-wrap {
  flex: 1;
  display: flex;
  align-items: flex-start;
  overflow: auto;
  padding: 16px;
}
.line-gutter {
  position: sticky;
  left: 0;
  z-index: 1;
  margin-right: 12px;
  padding-right: 10px;
  border-right: 1px solid var(--border);
  text-align: right;
  user-select: none;
  background: var(--bg-base);
}
.line-number {
  display: block;
  min-width: 38px;
  font-family: var(--font-mono);
  font-size: 13px;
  line-height: 1.5;
  color: var(--text-muted);
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
  background: rgba(56, 189, 248, 0.16);
  box-shadow: inset 0 0 0 1px rgba(56, 189, 248, 0.38);
  border-radius: 2px;
  padding: 0 2px;
}
.code-block :deep(.search-hit-current) {
  background: rgba(16, 185, 129, 0.22);
  box-shadow: inset 0 0 0 1px rgba(16, 185, 129, 0.9), 0 0 0 2px rgba(16, 185, 129, 0.45), 0 0 10px rgba(16, 185, 129, 0.35);
}
</style>
