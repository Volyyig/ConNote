<script setup lang="ts">
import AppHeader from './AppHeader.vue';
import SideBar from './SideBar.vue';
import BottomNav from './BottomNav.vue';
</script>

<template>
  <div class="app-layout">
    <!-- Desktop sidebar (hidden on mobile) -->
    <SideBar class="layout-sidebar" />

    <div class="layout-main">
      <AppHeader />
      <main class="layout-content">
        <slot />
      </main>
    </div>

    <!-- Mobile bottom nav (hidden on desktop) -->
    <BottomNav class="layout-bottom-nav" />
  </div>
</template>

<style scoped>
.app-layout {
  display: flex;
  flex-direction: column;
  height: 100%;
  width: 100%;
}

.layout-sidebar {
  display: none;
}

.layout-main {
  display: flex;
  flex-direction: column;
  flex: 1;
  min-height: 0;
}

.layout-content {
  flex: 1;
  overflow-y: auto;
  background: var(--color-bg);
}

.layout-bottom-nav {
  display: flex;
}

/* Desktop: show sidebar, hide bottom nav */
@media (min-width: 768px) {
  .app-layout {
    flex-direction: row;
  }

  .layout-sidebar {
    display: flex;
  }

  .layout-bottom-nav {
    display: none;
  }
}
</style>
