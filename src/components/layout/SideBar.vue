<script setup lang="ts">
import { useRoute } from 'vue-router';
import { markRaw } from 'vue';
import IconNote from '../icons/IconNote.vue';
import IconTag from '../icons/IconTag.vue';
import IconCollection from '../icons/IconCollection.vue';
import IconSettings from '../icons/IconSettings.vue';

const route = useRoute();

const navItems = [
  { to: '/', name: 'NoteList', label: '笔记', icon: markRaw(IconNote) },
  { to: '/tags', name: 'TagList', label: '标签', icon: markRaw(IconTag) },
  { to: '/collections', name: 'CollectionList', label: '合集', icon: markRaw(IconCollection) },
  { to: '/settings', name: 'Settings', label: '设置', icon: markRaw(IconSettings) },
];

function isActive(itemName: string) {
  const routeName = route.name as string;
  if (!routeName) return false;
  if (itemName === 'NoteList') return routeName.startsWith('Note');
  if (itemName === 'TagList') return routeName.startsWith('Tag');
  if (itemName === 'CollectionList') return routeName.startsWith('Collection');
  if (itemName === 'Settings') return routeName.startsWith('Settings');
  return false;
}
</script>

<template>
  <nav class="sidebar">
    <div class="sidebar-brand">
      <div class="brand-icon">
        <svg xmlns="http://www.w3.org/2000/svg" width="22" height="22" viewBox="0 0 24 24" fill="none"
          stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z" />
          <path d="M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z" />
        </svg>
      </div>
      <span class="brand-text">ConNote</span>
    </div>

    <div class="nav-list">
      <router-link v-for="item in navItems.filter(i => i.name !== 'Settings')" :key="item.name" :to="item.to"
        class="nav-item" :class="{ active: isActive(item.name) }">
        <component :is="item.icon" :size="20" />
        <span>{{ item.label }}</span>
      </router-link>
    </div>

    <div class="sidebar-spacer"></div>

    <div class="nav-list secondary-list">
      <router-link v-for="item in navItems.filter(i => i.name === 'Settings')" :key="item.name" :to="item.to"
        class="nav-item" :class="{ active: isActive(item.name) }">
        <component :is="item.icon" :size="20" />
        <span>{{ item.label }}</span>
      </router-link>
    </div>
  </nav>
</template>

<style scoped>
.sidebar {
  width: var(--sidebar-width);
  height: 100%;
  background: var(--color-surface);
  border-right: 1px solid var(--color-border-light);
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
  overflow-y: auto;
}

.sidebar-spacer {
  flex: 1;
}

.secondary-list {
  margin-top: auto;
  margin-bottom: var(--space-lg);
  padding-bottom: var(--space-lg);
}

@media (max-width: 767px) {
  .sidebar-spacer {
    display: none;
  }

  .secondary-list {
    margin-top: 0;
    padding-bottom: 0;
  }
}

.sidebar-brand {
  display: flex;
  align-items: center;
  gap: var(--space-md);
  padding: var(--space-xl) var(--space-lg);
}

.brand-icon {
  width: 38px;
  height: 38px;
  border-radius: var(--radius-md);
  background: var(--color-primary);
  color: #fff;
  display: flex;
  align-items: center;
  justify-content: center;
}

.brand-text {
  font-size: var(--font-lg);
  font-weight: 700;
  color: var(--color-text);
}

.nav-list {
  display: flex;
  flex-direction: column;
  padding: 0 var(--space-sm);
  gap: 2px;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: var(--space-md);
  padding: var(--space-md) var(--space-lg);
  border-radius: var(--radius-md);
  color: var(--color-text-secondary);
  font-weight: 500;
  font-size: var(--font-sm);
  text-decoration: none;
  transition: all var(--transition-fast);
}

.nav-item:hover {
  background: var(--color-border-light);
  color: var(--color-text);
}

.nav-item.active {
  background: var(--color-primary-bg);
  color: var(--color-primary);
}
</style>
