import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { ref, computed } from 'vue'

export const useAppStore = defineStore('app', () => {
  const projectPaths = ref([])
  const ignorePatterns = ref([])
  const codeExtensions = ref([])
  const searchQuery = ref('')
  const caseSensitive = ref(false)
  const isRegex = ref(false)
  const results = ref([])
  const selectedIndex = ref(0)
  const selectedFileContent = ref('')
  const selectedFilePath = ref('')
  const loading = ref(false)
  const searchProgressProcessed = ref(0)
  const searchProgressTotal = ref(0)
  const searchDurationMs = ref(0)
  const lastScannedFiles = ref(0)
  const searchHistory = ref([])
  const replaceText = ref('')
  const replaceSummary = ref('')
  const SEARCH_HISTORY_KEY = 'repogrep-search-history'
  const SEARCH_HISTORY_LIMIT = 12

  try {
    const raw = localStorage.getItem(SEARCH_HISTORY_KEY)
    const parsed = raw ? JSON.parse(raw) : []
    searchHistory.value = Array.isArray(parsed) ? parsed.filter((v) => typeof v === 'string') : []
  } catch (_) {
    searchHistory.value = []
  }

  listen('search-progress', (event) => {
    const processed = Number(event.payload?.processed ?? 0)
    const total = Number(event.payload?.total ?? 0)
    searchProgressProcessed.value = Number.isFinite(processed) ? processed : 0
    searchProgressTotal.value = Number.isFinite(total) ? total : 0
  }).catch((e) => {
    console.warn('search-progress listener failed', e)
  })

  const selectedResult = computed(() => {
    const i = selectedIndex.value
    const list = results.value
    return list[i] ?? null
  })

  async function loadPaths() {
    try {
      projectPaths.value = await invoke('get_project_paths')
    } catch (e) {
      console.error('loadPaths', e)
    }
  }

  async function addProjectPath(path) {
    try {
      await invoke('add_project_path', { path })
      await loadPaths()
    } catch (e) {
      console.error('addProjectPath', e)
    }
  }

  async function removeProjectPath(path) {
    try {
      await invoke('remove_project_path', { path })
      await loadPaths()
    } catch (e) {
      console.error('removeProjectPath', e)
    }
  }

  async function loadIgnores() {
    try {
      ignorePatterns.value = await invoke('get_ignore_patterns')
    } catch (e) {
      console.error('loadIgnores', e)
    }
  }

  async function addIgnorePattern(pattern) {
    if (!pattern?.trim()) return
    try {
      await invoke('add_ignore_pattern', { pattern: pattern.trim() })
      await loadIgnores()
    } catch (e) {
      console.error('addIgnorePattern', e)
    }
  }

  async function loadCodeExtensions() {
    try {
      codeExtensions.value = await invoke('get_code_extensions')
    } catch (e) {
      console.error('loadCodeExtensions', e)
    }
  }

  async function addCodeExtension(extension) {
    const ext = String(extension || '').trim().replace(/^\./, '').toLowerCase()
    if (!ext) return
    try {
      await invoke('add_code_extension', { extension: ext })
      await loadCodeExtensions()
    } catch (e) {
      console.error('addCodeExtension', e)
    }
  }

  async function removeCodeExtension(extension) {
    try {
      await invoke('remove_code_extension', { extension })
      await loadCodeExtensions()
    } catch (e) {
      console.error('removeCodeExtension', e)
    }
  }

  async function removeIgnorePattern(pattern) {
    try {
      await invoke('remove_ignore_pattern', { pattern })
      await loadIgnores()
    } catch (e) {
      console.error('removeIgnorePattern', e)
    }
  }

  async function openFolderPicker() {
    const { open } = await import('@tauri-apps/plugin-dialog')
    const selected = await open({
      directory: true,
      multiple: true,
    })
    if (selected) {
      const paths = Array.isArray(selected) ? selected : [selected]
      for (const p of paths) {
        const pathStr = typeof p === 'string' ? p : p?.path ?? ''
        if (pathStr) await addProjectPath(pathStr)
      }
    }
  }

  async function search() {
    const q = searchQuery.value?.trim()
    if (!q) {
      results.value = []
      selectedIndex.value = 0
      selectedFileContent.value = ''
      selectedFilePath.value = ''
      return
    }
    // Use the same paths we show in the UI so search never uses stale/wrong backend state
    const pathsToSearch =
      projectPaths.value?.length > 0
        ? projectPaths.value.map((p) => p.path)
        : null
    loading.value = true
    const startTimeMs = Date.now()
    searchProgressProcessed.value = 0
    searchProgressTotal.value = 0
    searchDurationMs.value = 0
    lastScannedFiles.value = 0
    replaceSummary.value = ''
    try {
      const list = await invoke('search_snippet', {
        args: {
          query: q,
          exact: true,
          caseSensitive: caseSensitive.value,
          isRegex: isRegex.value,
          pathsOverride: pathsToSearch,
        },
      })
      results.value = list
      selectedIndex.value = list.length > 0 ? 0 : -1
      selectedFilePath.value = ''
      selectedFileContent.value = ''
      if (list.length > 0) {
        await loadFileContent(list[0].file_path)
      }
      addSearchHistoryEntry(q)
    } catch (e) {
      console.error('search', e)
      results.value = []
    } finally {
      searchDurationMs.value = Date.now() - startTimeMs
      lastScannedFiles.value = searchProgressTotal.value
      loading.value = false
    }
  }

  async function openSelectedFileInEditor() {
    const path = selectedFilePath.value
    if (!path) return
    try {
      await invoke('open_file_in_editor', { path })
    } catch (e) {
      console.error('openSelectedFileInEditor', e)
    }
  }

  async function selectResult(index) {
    if (index < 0 || index >= results.value.length) return
    selectedIndex.value = index
    const r = results.value[index]
    await loadFileContent(r.file_path)
  }

  async function selectResultByFilePath(path) {
    const idx = results.value.findIndex((r) => r.file_path === path)
    if (idx < 0) return
    await selectResult(idx)
  }

  async function loadFileContent(path) {
    selectedFilePath.value = path
    try {
      selectedFileContent.value = await invoke('read_file_content', { path })
    } catch (e) {
      console.error('read_file_content', e)
      selectedFileContent.value = ''
    }
  }

  function setSearchQuery(q) {
    searchQuery.value = q
  }

  function addSearchHistoryEntry(query) {
    const trimmed = String(query || '').trim()
    if (!trimmed) return
    const deduped = [trimmed, ...searchHistory.value.filter((q) => q !== trimmed)]
    searchHistory.value = deduped.slice(0, SEARCH_HISTORY_LIMIT)
    localStorage.setItem(SEARCH_HISTORY_KEY, JSON.stringify(searchHistory.value))
  }

  function clearSearchHistory() {
    searchHistory.value = []
    localStorage.setItem(SEARCH_HISTORY_KEY, JSON.stringify([]))
  }

  async function replaceMatchesInResults() {
    const q = searchQuery.value?.trim()
    if (!q || !results.value.length) return
    const filePaths = results.value.map((r) => r.file_path)
    try {
      const out = await invoke('replace_snippet', {
        args: {
          query: q,
          replacement: replaceText.value ?? '',
          caseSensitive: caseSensitive.value,
          isRegex: isRegex.value,
          filePaths,
        },
      })
      const changed = Number(out?.filesChanged ?? 0)
      const replaced = Number(out?.replacementsMade ?? 0)
      replaceSummary.value = `Replaced ${replaced} occurrence${replaced === 1 ? '' : 's'} across ${changed} file${changed === 1 ? '' : 's'}.`
      await search()
    } catch (e) {
      console.error('replaceMatchesInResults', e)
      replaceSummary.value = 'Replace failed. Check console for details.'
    }
  }

  function exportResultsAsJson() {
    if (!results.value.length) return
    const payload = {
      query: searchQuery.value,
      caseSensitive: caseSensitive.value,
      isRegex: isRegex.value,
      generatedAt: new Date().toISOString(),
      results: results.value,
    }
    const blob = new Blob([JSON.stringify(payload, null, 2)], { type: 'application/json' })
    const a = document.createElement('a')
    a.href = URL.createObjectURL(blob)
    a.download = `repogrep-results-${Date.now()}.json`
    a.click()
    URL.revokeObjectURL(a.href)
  }

  function exportResultsAsCsv() {
    if (!results.value.length) return
    const escapeCsv = (value) => {
      const s = String(value ?? '')
      if (/[",\n]/.test(s)) return `"${s.replace(/"/g, '""')}"`
      return s
    }
    const header = ['root_hint', 'relative_path', 'file_path', 'match_count', 'lines']
    const rows = results.value.map((r) => [
      r.root_hint,
      r.relative_path,
      r.file_path,
      r.match_count ?? (r.lines?.length ?? 0),
      (r.lines || []).join(';'),
    ])
    const csv = [header, ...rows].map((row) => row.map(escapeCsv).join(',')).join('\n')
    const blob = new Blob([csv], { type: 'text/csv;charset=utf-8;' })
    const a = document.createElement('a')
    a.href = URL.createObjectURL(blob)
    a.download = `repogrep-results-${Date.now()}.csv`
    a.click()
    URL.revokeObjectURL(a.href)
  }

  return {
    projectPaths,
    ignorePatterns,
    codeExtensions,
    searchQuery,
    caseSensitive,
    isRegex,
    results,
    selectedIndex,
    selectedResult,
    selectedFileContent,
    selectedFilePath,
    loading,
    searchProgressProcessed,
    searchProgressTotal,
    searchDurationMs,
    lastScannedFiles,
    searchHistory,
    replaceText,
    replaceSummary,
    loadPaths,
    addProjectPath,
    removeProjectPath,
    loadIgnores,
    addIgnorePattern,
    removeIgnorePattern,
    loadCodeExtensions,
    addCodeExtension,
    removeCodeExtension,
    openFolderPicker,
    search,
    selectResult,
    selectResultByFilePath,
    setSearchQuery,
    openSelectedFileInEditor,
    clearSearchHistory,
    exportResultsAsJson,
    exportResultsAsCsv,
    replaceMatchesInResults,
  }
})
