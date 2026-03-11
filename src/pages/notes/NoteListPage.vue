<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import NoteCard from '../../components/notes/NoteCard.vue';
import IconPlus from '../../components/icons/IconPlus.vue';

const router = useRouter();
const notes = ref<Array<any>>([]);
const isLoading = ref(true);

async function loadNotes() {
  isLoading.value = true;
  try {
    notes.value = await invoke('get_notes');
  } catch (err) {
    console.error('Failed to load notes:', err);
  } finally {
    isLoading.value = false;
  }
}

function createNote() {
  router.push('/notes/new');
}

function openNote(id: number) {
  router.push(`/notes/${id}`);
}

onMounted(() => {
  loadNotes();
});
</script>

<template>
  <div class="page-container">
    <div v-if="isLoading" class="loading-state">
      加载中...
    </div>
    
    <div v-else-if="notes.length === 0" class="empty-state">
      <IconPlus :size="48" />
      <p>还没有笔记，点击右下角按钮新建</p>
    </div>
    
    <div v-else class="notes-grid">
      <NoteCard 
        v-for="note in notes" 
        :key="note.id" 
        :note="note"
        @click="openNote"
      />
    </div>

    <button class="fab" @click="createNote" title="新建笔记">
      <IconPlus :size="24" />
    </button>
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
  padding-bottom: 80px; /* space for fab */
}

@media (min-width: 600px) {
  .notes-grid {
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  }
}
</style>
