<script setup>
import { ref, watch, nextTick, computed } from 'vue'
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
})

const store = useAppStore()
const preRef = ref(null)

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

function highlightQuery(html) {
  const q = props.highlightText?.trim()
  if (!q) return html
  const esc = q.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
  const flags = props.highlightCaseSensitive ? 'g' : 'gi'
  const re = new RegExp(`(${esc})`, flags)
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
