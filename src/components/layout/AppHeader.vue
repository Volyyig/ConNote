<script setup lang="ts">
import { computed } from 'vue';
import { useRoute } from 'vue-router';

const route = useRoute();

const pageTitle = computed(() => {
  const map: Record<string, string> = {
    NoteList: '笔记',
    NoteCreate: '新建笔记',
    NoteEdit: '编辑笔记',
    TagList: '标签',
    CollectionList: '合集',
    CollectionCreate: '新建合集',
    CollectionEdit: '编辑合集',
    CollectionResults: '合集结果',
  };
  return map[route.name as string] || '笔记';
});

const showBack = computed(() => {
  const noBack = ['NoteList', 'TagList', 'CollectionList'];
  return !noBack.includes(route.name as string);
});
</script>

<template>
  <header class="app-header">
    <button v-if="showBack" class="back-btn" @click="$router.back()">
      <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none"
        stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <line x1="19" y1="12" x2="5" y2="12" />
        <polyline points="12 19 5 12 12 5" />
      </svg>
    </button>
    <h1 class="header-title">{{ pageTitle }}</h1>
    <div class="header-spacer"></div>
  </header>
</template>

<style scoped>
.app-header {
  height: var(--header-height);
  display: flex;
  align-items: center;
  padding: 0 var(--space-lg);
  background: var(--color-bg);
  border-bottom: 1px solid var(--color-border-light);
  flex-shrink: 0;
  gap: var(--space-sm);
  z-index: 100;
}

.header-title {
  font-size: var(--font-lg);
  font-weight: 600;
  color: var(--color-text);
  white-space: nowrap;
}

.back-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border-radius: var(--radius-md);
  color: var(--color-text);
  transition: background var(--transition-fast);
}
.back-btn:hover {
  background: var(--color-surface);
}

.header-spacer {
  flex: 1;
}

@media (min-width: 768px) {
  .app-header {
    padding: 0 var(--space-xl);
  }
}
</style>
