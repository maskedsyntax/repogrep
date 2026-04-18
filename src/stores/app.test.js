import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useAppStore } from './app'

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(async (cmd, args) => {
    if (cmd === 'get_project_paths') return []
    if (cmd === 'get_ignore_patterns') return []
    if (cmd === 'get_code_extensions') return []
    if (cmd === 'search_snippet') {
      return [
        { file_path: '/test/file.js', relative_path: 'file.js', root_hint: 'test', lines: [1], match_count: 1 }
      ]
    }
    return null
  })
}))

vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn(async () => () => {})
}))

const mockStorage = {}
global.localStorage = {
  getItem: vi.fn(key => mockStorage[key] || null),
  setItem: vi.fn((key, val) => mockStorage[key] = val),
  clear: vi.fn(() => { for(let k in mockStorage) delete mockStorage[k] }),
}

describe('App Store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    localStorage.clear()
  })

  it('initializes with default values', () => {
    const store = useAppStore()
    expect(store.searchQuery).toBe('')
    expect(store.results).toEqual([])
  })

  it('updates search query and adds history', async () => {
    const store = useAppStore()
    store.setSearchQuery('test query')
    expect(store.searchQuery).toBe('test query')

    await store.search()
    expect(store.searchHistory).toContain('test query')
    expect(store.results.length).toBe(1)
  })

  it('clears history', async () => {
    const store = useAppStore()
    store.setSearchQuery('query 1')
    await store.search()
    expect(store.searchHistory.length).toBe(1)

    store.clearSearchHistory()
    expect(store.searchHistory.length).toBe(0)
  })
})