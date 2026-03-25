<script setup>
import { ref, onMounted } from 'vue'
import { Sun, Moon } from 'lucide-vue-next'
import logoUrl from './assets/logo.png'

const isDark = ref(false)

const toggleTheme = () => {
  isDark.value = !isDark.value
  document.documentElement.setAttribute('data-theme', isDark.value ? 'dark' : 'light')
  localStorage.setItem('theme', isDark.value ? 'dark' : 'light')
}

onMounted(() => {
  const savedTheme = localStorage.getItem('theme') || (window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light')
  isDark.value = savedTheme === 'dark'
  document.documentElement.setAttribute('data-theme', savedTheme)
})
</script>

<template>
  <div class="app-layout">
    <!-- Navbar -->
    <nav class="nav">
      <div class="container nav-content">
        <div class="logo-wrapper">
          <img :src="logoUrl" alt="repogrep" class="logo-img" />
          <span class="logo-text">repogrep</span>
        </div>
        <div class="nav-links">
          <a href="#features">Features</a>
          <a href="#about">About</a>
          <a href="https://github.com/maskedsyntax/repogrep" class="github-link">GitHub</a>
          <button class="theme-toggle" @click="toggleTheme" :title="isDark ? 'Switch to Light' : 'Switch to Dark'">
            <Sun v-if="isDark" :size="20" />
            <Moon v-else :size="20" />
          </button>
        </div>
      </div>
    </nav>

    <!-- Hero -->
    <header class="hero section">
      <div class="container hero-content">
        <div class="hero-text">
          <h1 class="gradient-text">Search all your codebases <span class="font-serif">at once.</span></h1>
          <p class="hero-desc">
            A lightning-fast, local-first code search tool for developers. No indexing, no cloud, just your code and raw performance.
          </p>
          <div class="hero-ctas">
            <a href="https://github.com/maskedsyntax/repogrep/releases" class="btn btn-primary">Download v0.1.0</a>
            <a href="https://github.com/maskedsyntax/repogrep" class="btn btn-secondary">Star on GitHub</a>
          </div>
        </div>
        <div class="hero-visual">
          <div class="mockup">
             <!-- Simplified 3-pane mockup representation -->
             <div class="mockup-sidebar"></div>
             <div class="mockup-main">
                <div class="mockup-search"></div>
                <div class="mockup-results">
                   <div class="mockup-line" v-for="n in 5" :key="n"></div>
                </div>
             </div>
             <div class="mockup-preview"></div>
          </div>
        </div>
      </div>
    </header>

    <!-- Features -->
    <section id="features" class="section bg-alt">
      <div class="container">
        <h2 class="section-title">Built for Performance & Privacy</h2>
        <div class="features-grid">
          <div class="feature-card">
            <div class="feature-icon">⚡</div>
            <h3>Fast Native Search</h3>
            <p>Built with Rust for maximum performance. Search through thousands of files in milliseconds without pre-indexing.</p>
          </div>
          <div class="feature-card">
            <div class="feature-icon">🛡️</div>
            <h3>Local & Private</h3>
            <p>Your code never leaves your machine. No cloud processing, no tracking, just a powerful local utility.</p>
          </div>
          <div class="feature-card">
            <div class="feature-icon">📁</div>
            <h3>Multi-Repo Search</h3>
            <p>Add multiple project folders and search across all of them simultaneously. Perfect for microservices.</p>
          </div>
          <div class="feature-card">
            <div class="feature-icon">🔍</div>
            <h3>Precise Snippets</h3>
            <p>Find exact code snippets with case-sensitivity and language-specific filters for pin-point accuracy.</p>
          </div>
        </div>
      </div>
    </section>

    <!-- About/Philosophy -->
    <section id="about" class="section">
      <div class="container about-content">
        <div class="about-text">
          <h2>Why Repogrep?</h2>
          <p>
            We believe that code search should be simple, private, and incredibly fast. Repogrep doesn't use heavy database indexing; instead, it leverages modern hardware and Rust's speed to scan your code in real-time. 
          </p>
          <p>
            Whether you're looking for where an API is used across multiple services or refactoring a shared utility, Repogrep gives you the results you need without the overhead.
          </p>
        </div>
      </div>
    </section>

    <!-- Footer -->
    <footer class="footer">
      <div class="container footer-content">
        <div class="footer-left">
          <div class="logo-wrapper-small">
            <img :src="logoUrl" alt="repogrep" class="logo-img-small" />
            <span class="logo-text-small">repogrep</span>
          </div>
          <p>© 2026 Repogrep Team. MIT Licensed.</p>
        </div>
        <div class="footer-links">
          <a href="https://github.com/maskedsyntax/repogrep">GitHub</a>
          <a href="https://github.com/maskedsyntax/repogrep/issues">Issues</a>
          <a href="https://github.com/maskedsyntax/repogrep/releases">Releases</a>
        </div>
      </div>
    </footer>
  </div>
</template>

<style scoped>
.app-layout {
  min-height: 100vh;
}

.nav {
  height: var(--nav-height);
  border-bottom: 1px solid var(--border);
  position: sticky;
  top: 0;
  background: var(--bg-base);
  z-index: 100;
  display: flex;
  align-items: center;
}

.nav-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
}

.logo-wrapper {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.logo-img {
  height: 32px;
  width: 32px;
  object-fit: contain;
}

.logo-text {
  font-size: 1.5rem;
  font-weight: 800;
  letter-spacing: -0.025em;
  color: var(--accent);
}

.nav-links {
  display: flex;
  align-items: center;
  gap: 2rem;
}

.nav-links a {
  font-weight: 500;
  color: var(--text-muted);
}

.nav-links a:hover {
  color: var(--text);
}

.theme-toggle {
  padding: 0.5rem;
  border-radius: var(--radius);
  background: var(--bg-hover);
  border: 1px solid var(--border);
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--accent);
  transition: var(--transition);
}

.theme-toggle:hover {
  background: var(--border);
  color: var(--text);
}

/* Hero */
.hero {
  padding-top: 8rem;
  padding-bottom: 8rem;
}

.hero-content {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 4rem;
  align-items: center;
}

.hero-text h1 {
  font-size: 3.5rem;
  margin-bottom: 1.5rem;
  line-height: 1.1;
  letter-spacing: -0.02em;
}

.gradient-text {
  background: linear-gradient(135deg, var(--text) 30%, var(--accent) 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.hero-desc {
  font-size: 1.25rem;
  color: var(--text-muted);
  margin-bottom: 2.5rem;
  max-width: 500px;
}

.hero-ctas {
  display: flex;
  gap: 1rem;
}

.hero-visual {
  perspective: 1000px;
}

.mockup {
  background: var(--bg-elevated);
  border: 1px solid var(--border);
  border-radius: 12px;
  height: 350px;
  box-shadow: 0 20px 50px rgba(0,0,0,0.1);
  display: grid;
  grid-template-columns: 80px 1.5fr 1fr;
  overflow: hidden;
  transform: rotateY(-10deg) rotateX(5deg);
}

.mockup-sidebar { background: var(--bg-base); border-right: 1px solid var(--border); }
.mockup-main { padding: 1rem; border-right: 1px solid var(--border); }
.mockup-search { height: 30px; background: var(--bg-hover); border-radius: 4px; margin-bottom: 1rem; }
.mockup-line { height: 10px; background: var(--bg-hover); border-radius: 2px; margin-bottom: 0.75rem; width: 80%; }
.mockup-line:nth-child(2n) { width: 60%; }
.mockup-preview { background: var(--bg-base); }

/* Features - Styles now handled by global style.css */

/* About */
.about-content {
  max-width: 800px;
  text-align: center;
}

.about-text h2 {
  font-size: 2.5rem;
  margin-bottom: 1.5rem;
}

.about-text p {
  font-size: 1.15rem;
  color: var(--text-muted);
  margin-bottom: 1.5rem;
}

/* Footer */
.footer {
  padding: 4rem 0;
  border-top: 1px solid var(--border);
  margin-top: 5rem;
}

.footer-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.logo-wrapper-small {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  margin-bottom: 0.5rem;
}

.logo-img-small {
  height: 32px;
  width: 32px;
  object-fit: contain;
}

.logo-text-small {
  font-size: 1.25rem;
  font-weight: 800;
  color: var(--accent);
}

.footer-left p {
  color: var(--text-muted);
  font-size: 0.9rem;
}

.footer-links {
  display: flex;
  gap: 2rem;
}

.footer-links a {
  color: var(--text-muted);
  font-size: 0.9rem;
}

@media (max-width: 968px) {
  .hero-content {
    grid-template-columns: 1fr;
    text-align: center;
  }
  .hero-text h1 {
    font-size: 2.75rem;
  }
  .hero-desc {
    margin: 0 auto 2.5rem;
  }
  .hero-ctas {
    justify-content: center;
  }
  .hero-visual {
    margin-top: 4rem;
  }
  .mockup {
    transform: none;
    max-width: 600px;
    margin: 0 auto;
  }
}

@media (max-width: 640px) {
  .nav-links a { display: none; }
  .nav-links .github-link { display: flex; }
  .footer-content {
    flex-direction: column;
    text-align: center;
    gap: 2rem;
  }
}
</style>
