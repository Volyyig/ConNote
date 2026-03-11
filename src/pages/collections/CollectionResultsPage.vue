<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import NoteCard from '../../components/notes/NoteCard.vue';

const route = useRoute();
const router = useRouter();
const collectionId = parseInt(route.params.id as string);

const notes = ref<Array<any>>([]);
const isLoading = ref(true);

async function loadResults() {
  isLoading.value = true;
  try {
    notes.value = await invoke('query_collection', { id: collectionId });
  } catch (err) {
    console.error('Failed to load collection results:', err);
    alert('查询失败: ' + err);
  } finally {
    isLoading.value = false;
  }
}

function openNote(id: number) {
  router.push(`/notes/${id}`);
}

onMounted(() => {
  loadResults();
});
</script>

<template>
  <div class="page-container">
    <div v-if="isLoading" class="loading-state">查询中...</div>
    
    <div v-else-if="notes.length === 0" class="empty-state">
      <p>没有符合条件的笔记</p>
    </div>
    
    <div v-else class="notes-grid">
      <NoteCard 
        v-for="note in notes" 
        :key="note.id" 
        :note="note"
        @click="openNote"
      />
    </div>
  </div>
</template>

<style scoped>
.page-container {
  padding: var(--space-lg);
  height: 100%;
}

.loading-state {
  display: flex;
  justify-content: center;
  padding: var(--space-2xl);
  color: var(--color-text-tertiary);
}

.notes-grid {
  display: grid;
  grid-template-columns: 1fr;
  gap: var(--space-md);
}

@media (min-width: 600px) {
  .notes-grid {
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  }
}
</style>
