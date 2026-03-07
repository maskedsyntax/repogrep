<script setup>
import { onMounted, onUnmounted, ref } from 'vue'
import { useSnippetStore } from './stores/snippets'
import SearchBar from './components/SearchBar.vue'
import SnippetList from './components/SnippetList.vue'
import PreviewPane from './components/PreviewPane.vue'
import Sidebar from './components/Sidebar.vue'
import Settings from './components/Settings.vue'
import { useDark, useToggle } from '@vueuse/core'
import { listen } from '@tauri-apps/api/event'

const store = useSnippetStore()
const isDark = useDark({ selector: 'html', attribute: 'data-theme', valueDark: 'dark', valueLight: 'light' })
const toggleDark = useToggle(isDark)
const showSettings = ref(false)

onMounted(async () => {
  store.loadAll()
  const unlisten = await listen('index_progress', (e) => {
    store.setIndexProgress(e.payload)
  })
  onUnmounted(() => unlisten())
})
</script>

<template>
  <div class="app" data-theme>
    <aside class="sidebar">
      <Sidebar />
    </aside>
    <main class="main">
      <header class="header">
        <SearchBar />
        <div class="header-actions">
          <button
            class="icon-btn"
            :class="{ active: showSettings }"
            @click="showSettings = !showSettings"
            title="Settings"
            aria-label="Settings"
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <circle cx="12" cy="12" r="3"/>
              <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/>
            </svg>
          </button>
          <button
            class="icon-btn"
            @click="toggleDark()"
            :title="isDark ? 'Light mode' : 'Dark mode'"
            :aria-label="isDark ? 'Light mode' : 'Dark mode'"
          >
            <svg v-if="isDark" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="12" cy="12" r="4"/>
              <path d="M12 2v2M12 20v2M4.93 4.93l1.41 1.41M17.66 17.66l1.41 1.41M2 12h2M20 12h2M6.34 17.66l-1.41 1.41M19.07 4.93l-1.41 1.41"/>
            </svg>
            <svg v-else xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>
            </svg>
          </button>
        </div>
      </header>
      <div v-if="store.indexProgress" class="index-progress">
        <span class="index-progress-text">Indexing… {{ Math.round(store.indexProgress.progress || 0) }}%</span>
        <div class="index-progress-track">
          <div class="index-progress-bar" :style="{ width: (store.indexProgress.progress || 0) + '%' }" />
        </div>
      </div>
      <div class="content">
        <section class="list-section">
          <SnippetList />
        </section>
        <section class="preview-section">
          <PreviewPane />
        </section>
      </div>
    </main>
    <Settings v-if="showSettings" @close="showSettings = false" />
  </div>
</template>

<style scoped>
.app {
  display: flex;
  height: 100vh;
  overflow: hidden;
  background: var(--bg-base);
}

.sidebar {
  width: 220px;
  flex-shrink: 0;
  border-right: 1px solid var(--border);
  background: var(--bg-elevated);
  animation: slideIn 0.2s ease-out;
}

.main {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-width: 0;
}

.header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  border-bottom: 1px solid var(--border);
  background: var(--bg-base);
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 4px;
}

.icon-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border: none;
  border-radius: 8px;
  background: transparent;
  color: var(--text-muted);
  cursor: pointer;
  transition: color 0.15s, background 0.15s;
}
.icon-btn:hover {
  color: var(--text);
  background: var(--bg-hover);
}
.icon-btn.active {
  color: var(--accent);
  background: var(--accent-subtle);
}

.index-progress {
  padding: 6px 16px 8px;
  background: var(--bg-elevated);
  border-bottom: 1px solid var(--border);
}
.index-progress-text {
  display: block;
  font-size: 11px;
  color: var(--text-muted);
  margin-bottom: 4px;
}
.index-progress-track {
  height: 4px;
  background: var(--bg-hover);
  border-radius: 2px;
  overflow: hidden;
}
.index-progress-bar {
  height: 100%;
  background: var(--accent);
  border-radius: 2px;
  transition: width 0.2s ease-out;
}

.content {
  flex: 1;
  display: flex;
  min-height: 0;
}

.list-section {
  width: 320px;
  flex-shrink: 0;
  border-right: 1px solid var(--border);
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.preview-section {
  flex: 1;
  min-width: 0;
  overflow: hidden;
}

@keyframes slideIn {
  from {
    opacity: 0;
    transform: translateX(-4px);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}
</style>
