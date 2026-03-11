<script setup lang="ts">
import AppLayout from './components/layout/AppLayout.vue';
import { releaseWindow } from './utils/window';
import { onMounted } from 'vue';

onMounted(() => {
    releaseWindow();
});

</script>

<template>
  <AppLayout>
    <router-view v-slot="{ Component }">
      <transition name="fade" mode="out-in">
        <component :is="Component" />
      </transition>
    </router-view>
  </AppLayout>
</template>