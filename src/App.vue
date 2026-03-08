<script setup>
import SnippetInputPane from './components/SnippetInputPane.vue'
import MatchListPane from './components/MatchListPane.vue'
import CodePreviewPane from './components/CodePreviewPane.vue'
import { useAppStore } from './stores/app'

const store = useAppStore()
</script>

<template>
  <div class="app">
    <div class="pane-col snippet-col">
      <SnippetInputPane />
    </div>
    <div class="pane-col list-col">
      <MatchListPane />
    </div>
    <div class="pane-col preview-col">
      <CodePreviewPane
        :content="store.selectedFileContent"
        :highlight-text="store.searchQuery"
      />
    </div>
  </div>
</template>

<style scoped>
.app {
  display: grid;
  grid-template-columns: minmax(280px, 1fr) minmax(260px, 380px) 1fr;
  height: 100vh;
  min-height: 0;
  background: var(--bg-base);
}
.pane-col {
  min-width: 0;
  min-height: 0;
  overflow: hidden;
  border-right: 1px solid var(--border);
}
.pane-col:last-child {
  border-right: none;
}
</style>
