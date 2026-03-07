<script setup>
import { ref, watch, onMounted } from 'vue'
import { useDebounceFn } from '@vueuse/core'
import { useMagicKeys } from '@vueuse/core'
import { useSnippetStore } from '../stores/snippets'

const store = useSnippetStore()
const query = ref('')
const selectedTags = ref([])
const inputEl = ref(null)

const doSearch = useDebounceFn(() => {
  store.search(query.value, selectedTags.value, true)
}, 200)

watch([query, selectedTags], () => {
  doSearch()
}, { deep: true })

const { slash } = useMagicKeys()
watch(slash, (v) => {
  if (v) inputEl.value?.focus()
})
</script>

<template>
  <div class="search-bar">
    <span class="search-icon" aria-hidden="true">
      <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="11" cy="11" r="8"/>
        <path d="m21 21-4.35-4.35"/>
      </svg>
    </span>
    <input
      ref="inputEl"
      v-model="query"
      type="search"
      class="search-input"
      placeholder="Search snippets… (press / to focus)"
      autocomplete="off"
      aria-label="Search"
    />
  </div>
</template>

<style scoped>
.search-bar {
  flex: 1;
  position: relative;
  max-width: 480px;
}

.search-icon {
  position: absolute;
  left: 12px;
  top: 50%;
  transform: translateY(-50%);
  color: var(--text-muted);
  pointer-events: none;
  transition: color 0.2s;
}

.search-input {
  width: 100%;
  height: 40px;
  padding: 0 12px 0 38px;
  border: 1px solid var(--border);
  border-radius: var(--radius);
  background: var(--bg-elevated);
  color: var(--text);
  font-size: 14px;
  font-family: inherit;
  transition: border-color 0.2s, box-shadow 0.2s;
}
.search-input::placeholder {
  color: var(--text-muted);
}
.search-input:hover {
  border-color: var(--text-muted);
}
.search-input:focus {
  outline: none;
  border-color: var(--accent);
  box-shadow: 0 0 0 2px var(--accent-subtle);
}
</style>
