<script setup>
import { ref } from 'vue'
import { useSnippetStore } from '../stores/snippets'

const store = useSnippetStore()
const pasteModal = ref(false)
const pasteContent = ref('')
const pasteLang = ref('plaintext')

function addFromClipboard() {
  pasteContent.value = ''
  pasteModal.value = true
}

async function submitPaste() {
  if (!pasteContent.value.trim()) return
  await store.importClipboard(pasteContent.value.trim(), pasteLang.value || null)
  pasteModal.value = false
  pasteContent.value = ''
  pasteLang.value = 'plaintext'
}

function closePaste() {
  pasteModal.value = false
  pasteContent.value = ''
}
</script>

<template>
  <nav class="sidebar-nav">
    <div class="sidebar-brand">
      <span class="brand-icon" aria-hidden="true">
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <polyline points="16 18 22 12 16 6"/>
          <polyline points="8 6 2 12 8 18"/>
        </svg>
      </span>
      <span class="brand-text">Repogrep</span>
    </div>
    <div class="sidebar-actions">
      <button
        class="side-btn"
        title="Paste from clipboard"
        aria-label="Paste from clipboard"
        @click="addFromClipboard"
      >
        <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
          <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/>
        </svg>
        <span>Paste</span>
      </button>
    </div>
    <p class="sidebar-hint">Use Settings to add directories and index code files.</p>
  </nav>
  <Teleport to="body">
    <div v-if="pasteModal" class="modal-backdrop" @click.self="closePaste">
      <div class="modal" role="dialog" aria-labelledby="paste-title">
        <h2 id="paste-title" class="modal-title">Import from clipboard</h2>
        <textarea
          v-model="pasteContent"
          class="modal-textarea"
          placeholder="Paste code here…"
          rows="8"
        />
        <div class="modal-row">
          <label class="modal-label">
            Language
            <input v-model="pasteLang" type="text" list="paste-langs" class="modal-input" placeholder="plaintext" />
          </label>
          <datalist id="paste-langs">
            <option value="plaintext"/>
            <option value="javascript"/>
            <option value="typescript"/>
            <option value="rust"/>
            <option value="python"/>
            <option value="sql"/>
            <option value="bash"/>
            <option value="json"/>
          </datalist>
        </div>
        <div class="modal-actions">
          <button class="btn btn-secondary" @click="closePaste">Cancel</button>
          <button class="btn btn-primary" :disabled="!pasteContent.trim()" @click="submitPaste">Add snippet</button>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<style scoped>
.sidebar-nav {
  padding: 16px 12px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.sidebar-brand {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 4px 0;
}

.brand-icon {
  display: flex;
  color: var(--accent);
  transition: transform 0.2s;
}
.sidebar-brand:hover .brand-icon {
  transform: scale(1.05);
}

.brand-text {
  font-weight: 700;
  font-size: 16px;
  color: var(--text);
  letter-spacing: -0.02em;
}

.sidebar-actions {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.side-btn {
  display: flex;
  align-items: center;
  gap: 10px;
  width: 100%;
  padding: 10px 12px;
  border: none;
  border-radius: var(--radius);
  background: transparent;
  color: var(--text-muted);
  font-size: 13px;
  font-family: inherit;
  cursor: pointer;
  transition: background 0.15s, color 0.15s;
  text-align: left;
}
.side-btn:hover {
  background: var(--bg-hover);
  color: var(--text);
}

.sidebar-hint {
  margin: 0;
  padding: 12px 0 0;
  font-size: 11px;
  color: var(--text-muted);
  line-height: 1.4;
  border-top: 1px solid var(--border);
}

/* Modal */
.modal-backdrop {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.4);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  animation: backdropIn 0.2s ease-out;
}

.modal {
  background: var(--bg-elevated);
  border-radius: 12px;
  padding: 24px;
  width: 90%;
  max-width: 480px;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.15);
  animation: modalIn 0.25s ease-out;
}

.modal-title {
  margin: 0 0 16px;
  font-size: 18px;
  font-weight: 600;
  color: var(--text);
}

.modal-textarea {
  width: 100%;
  padding: 12px;
  border: 1px solid var(--border);
  border-radius: var(--radius);
  background: var(--bg-base);
  color: var(--text);
  font-family: var(--font-mono);
  font-size: 13px;
  line-height: 1.5;
  resize: vertical;
  margin-bottom: 12px;
}

.modal-row {
  margin-bottom: 16px;
}
.modal-label {
  display: block;
  font-size: 12px;
  color: var(--text-muted);
  margin-bottom: 4px;
}
.modal-input {
  width: 140px;
  padding: 8px 10px;
  border: 1px solid var(--border);
  border-radius: var(--radius);
  background: var(--bg-base);
  color: var(--text);
  font-size: 13px;
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}

.btn {
  padding: 8px 16px;
  border-radius: var(--radius);
  font-size: 13px;
  font-weight: 500;
  font-family: inherit;
  cursor: pointer;
  transition: opacity 0.15s, background 0.15s;
}
.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
.btn-secondary {
  border: 1px solid var(--border);
  background: transparent;
  color: var(--text);
}
.btn-secondary:hover:not(:disabled) {
  background: var(--bg-hover);
}
.btn-primary {
  border: none;
  background: var(--accent);
  color: white;
}
.btn-primary:hover:not(:disabled) {
  filter: brightness(1.1);
}

@keyframes backdropIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

@keyframes modalIn {
  from {
    opacity: 0;
    transform: scale(0.96) translateY(-8px);
  }
  to {
    opacity: 1;
    transform: scale(1) translateY(0);
  }
}
</style>
