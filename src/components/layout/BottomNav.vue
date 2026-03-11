<script setup lang="ts">
import { useRoute } from 'vue-router';
import { markRaw } from 'vue';
import IconNote from '../icons/IconNote.vue';
import IconTag from '../icons/IconTag.vue';
import IconCollection from '../icons/IconCollection.vue';

const route = useRoute();

const navItems = [
  { to: '/', name: 'NoteList', label: '笔记', icon: markRaw(IconNote) },
  { to: '/tags', name: 'TagList', label: '标签', icon: markRaw(IconTag) },
  { to: '/collections', name: 'CollectionList', label: '合集', icon: markRaw(IconCollection) },
];

function isActive(itemName: string) {
  const routeName = route.name as string;
  if (!routeName) return false;
  if (itemName === 'NoteList') return routeName.startsWith('Note');
  if (itemName === 'TagList') return routeName.startsWith('Tag');
  if (itemName === 'CollectionList') return routeName.startsWith('Collection');
  return false;
}
</script>

<template>
  <nav class="bottom-nav">
    <router-link
      v-for="item in navItems"
      :key="item.name"
      :to="item.to"
      class="bottom-nav-item"
      :class="{ active: isActive(item.name) }"
    >
      <component :is="item.icon" :size="22" />
      <span class="bottom-nav-label">{{ item.label }}</span>
    </router-link>
  </nav>
</template>

<style scoped>
.bottom-nav {
  height: var(--bottom-nav-height);
  display: flex;
  align-items: center;
  justify-content: space-around;
  background: var(--color-bg);
  border-top: 1px solid var(--color-border-light);
  flex-shrink: 0;
  z-index: 100;
  padding-bottom: env(safe-area-inset-bottom, 0);
}

.bottom-nav-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 2px;
  flex: 1;
  height: 100%;
  color: var(--color-text-tertiary);
  text-decoration: none;
  font-size: var(--font-xs);
  font-weight: 500;
  transition: color var(--transition-fast);
}

.bottom-nav-item.active {
  color: var(--color-primary);
}

.bottom-nav-label {
  line-height: 1;
}
</style>
