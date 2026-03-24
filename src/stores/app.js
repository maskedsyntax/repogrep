import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import { ref, computed } from 'vue'

export const useAppStore = defineStore('app', () => {
  const projectPaths = ref([])
  const ignorePatterns = ref([])
  const searchQuery = ref('')
  const caseSensitive = ref(false)
  const isRegex = ref(false)
  const results = ref([])
  const selectedIndex = ref(0)
  const selectedFileContent = ref('')
  const selectedFilePath = ref('')
  const loading = ref(false)

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
    } catch (e) {
      console.error('search', e)
      results.value = []
    } finally {
      loading.value = false
    }
  }

  async function selectResult(index) {
    if (index < 0 || index >= results.value.length) return
    selectedIndex.value = index
    const r = results.value[index]
    await loadFileContent(r.file_path)
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

  return {
    projectPaths,
    ignorePatterns,
    searchQuery,
    caseSensitive,
    isRegex,
    results,
    selectedIndex,
    selectedResult,
    selectedFileContent,
    selectedFilePath,
    loading,
    loadPaths,
    addProjectPath,
    removeProjectPath,
    loadIgnores,
    addIgnorePattern,
    removeIgnorePattern,
    openFolderPicker,
    search,
    selectResult,
    setSearchQuery,
  }
})
