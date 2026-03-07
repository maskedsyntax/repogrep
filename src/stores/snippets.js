import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import { ref, shallowRef } from 'vue'

export const useSnippetStore = defineStore('snippets', () => {
  const snippets = shallowRef([])
  const selected = ref(null)
  const loading = ref(false)
  const indexProgress = ref(null)

  async function loadAll() {
    loading.value = true
    try {
      snippets.value = await invoke('get_snippets')
    } finally {
      loading.value = false
    }
  }

  async function search(query, tags = [], fuzzy = true) {
    loading.value = true
    try {
      snippets.value = await invoke('search_snippets', {
        query: query || '',
        tags: tags.length ? tags : null,
        fuzzy,
      })
    } finally {
      loading.value = false
    }
  }

  async function select(id) {
    if (!id) {
      selected.value = null
      return
    }
    try {
      selected.value = await invoke('get_snippet', { id })
    } catch {
      selected.value = null
    }
  }

  async function addSnippet({ title, code, language, tags = [] }) {
    const snippet = await invoke('add_snippet', {
      title,
      code,
      language: language || 'plaintext',
      tags: tags.length ? tags : null,
    })
    snippets.value = [snippet, ...snippets.value]
    return snippet
  }

  async function updateSnippet(id, { title, code, language, tags }) {
    const snippet = await invoke('update_snippet', {
      id,
      title: title ?? undefined,
      code: code ?? undefined,
      language: language ?? undefined,
      tags: tags ?? undefined,
    })
    const idx = snippets.value.findIndex((s) => s.id === id)
    if (idx >= 0) snippets.value[idx] = snippet
    if (selected.value?.id === id) selected.value = snippet
    return snippet
  }

  async function deleteSnippet(id) {
    await invoke('delete_snippet', { id })
    snippets.value = snippets.value.filter((s) => s.id !== id)
    if (selected.value?.id === id) selected.value = null
  }

  async function indexDirs(dirs) {
    indexProgress.value = { progress: 0, current: '' }
    await invoke('index_directories', { dirs })
    indexProgress.value = null
    await loadAll()
  }

  function setIndexProgress(p) {
    if (p?.done) {
      indexProgress.value = null
    } else {
      indexProgress.value = p
    }
  }

  async function importClipboard(content, language) {
    const snippet = await invoke('import_from_clipboard', {
      content,
      language: language || null,
    })
    snippets.value = [snippet, ...snippets.value]
    return snippet
  }

  async function exportSnippets(ids = null) {
    return invoke('export_snippets', { ids })
  }

  return {
    snippets,
    selected,
    loading,
    indexProgress,
    loadAll,
    search,
    select,
    addSnippet,
    updateSnippet,
    deleteSnippet,
    indexDirs,
    setIndexProgress,
    importClipboard,
    exportSnippets,
  }
})
