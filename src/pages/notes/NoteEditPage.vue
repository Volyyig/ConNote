<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import IconTrash from '../../components/icons/IconTrash.vue';

const route = useRoute();
const router = useRouter();

const isNew = route.name === 'NoteCreate';
const noteId = !isNew ? parseInt(route.params.id as string) : null;

const title = ref('');
const content = ref('');
const availableTags = ref<Array<{id: number, name: string}>>([]);
const selectedTagIds = ref<number[]>([]);

const isSaving = ref(false);

async function loadData() {
  try {
    // Load all tags first
    availableTags.value = await invoke('get_tags');
    
    // Load note if editing
    if (!isNew && noteId) {
      const note: any = await invoke('get_note', { id: noteId });
      title.value = note.title;
      content.value = note.content;
      selectedTagIds.value = note.tags.map((t: any) => t.id);
    }
  } catch (err) {
    console.error('Failed to load data:', err);
  }
}

async function save() {
  isSaving.value = true;
  try {
    if (isNew) {
      await invoke('create_note', {
        title: title.value,
        content: content.value,
        tagIds: selectedTagIds.value
      });
    } else {
      await invoke('update_note', {
        id: noteId,
        title: title.value,
        content: content.value,
        tagIds: selectedTagIds.value
      });
    }
    router.back();
  } catch (err) {
    console.error('Failed to save note:', err);
    alert('保存失败: ' + err);
  } finally {
    isSaving.value = false;
  }
}

async function deleteNote() {
  if (!confirm('确定要删除这条笔记吗？')) return;
  
  try {
    await invoke('delete_note', { id: noteId });
    router.back();
  } catch (err) {
    console.error('Failed to delete note:', err);
    alert('删除失败: ' + err);
  }
}

function toggleTag(tagId: number) {
  const index = selectedTagIds.value.indexOf(tagId);
  if (index >= 0) {
    selectedTagIds.value.splice(index, 1);
  } else {
    selectedTagIds.value.push(tagId);
  }
}

onMounted(() => {
  loadData();
});
</script>

<template>
  <div class="edit-container">
    <div class="actions-bar">
      <button class="btn-primary" @click="save" :disabled="isSaving">
        {{ isSaving ? '保存中...' : '保存' }}
      </button>
      <button v-if="!isNew" class="btn-danger icon-btn" @click="deleteNote" title="删除笔记">
        <IconTrash :size="18" />
      </button>
    </div>

    <input 
      v-model="title" 
      class="title-input" 
      placeholder="输入标题..." 
      autofocus
    />
    
    <div class="tags-section" v-if="availableTags.length > 0">
      <span class="tags-label">标签:</span>
      <div class="tags-list">
        <button 
          v-for="tag in availableTags" 
          :key="tag.id"
          class="tag-toggle"
          :class="{ active: selectedTagIds.includes(tag.id) }"
          @click="toggleTag(tag.id)"
        >
          {{ tag.name }}
        </button>
      </div>
    </div>

    <textarea 
      v-model="content" 
      class="content-input" 
      placeholder="开始撰写笔记..."
    ></textarea>
  </div>
</template>

<style scoped>
.edit-container {
  display: flex;
  flex-direction: column;
  height: 100%;
  padding: var(--space-lg);
  gap: var(--space-md);
  max-width: 800px;
  margin: 0 auto;
}

.actions-bar {
  display: flex;
  justify-content: flex-end;
  gap: var(--space-md);
}

.icon-btn {
  padding: var(--space-sm);
}

.title-input {
  font-size: var(--font-2xl);
  font-weight: 700;
  border: none;
  border-bottom: 2px solid transparent;
  padding: var(--space-sm) 0;
  border-radius: 0;
  background: transparent;
  color: var(--color-text);
}
.title-input:focus {
  border-bottom-color: var(--color-primary);
  box-shadow: none;
}

.tags-section {
  display: flex;
  align-items: flex-start;
  gap: var(--space-md);
  padding: var(--space-sm) 0;
}

.tags-label {
  font-size: var(--font-sm);
  color: var(--color-text-tertiary);
  padding-top: 4px;
}

.tags-list {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-xs);
}

.tag-toggle {
  padding: 4px 12px;
  border-radius: var(--radius-full);
  font-size: var(--font-xs);
  font-weight: 500;
  border: 1px solid var(--color-border);
  color: var(--color-text-secondary);
  transition: all var(--transition-fast);
  background: transparent;
}

.tag-toggle:hover {
  background: var(--color-surface);
}

.tag-toggle.active {
  background: var(--color-primary);
  border-color: var(--color-primary);
  color: white;
}

.content-input {
  flex: 1;
  border: none;
  resize: none;
  font-size: var(--font-base);
  line-height: 1.8;
  padding: var(--space-sm) 0;
  background: transparent;
  color: var(--color-text);
}
.content-input:focus {
  box-shadow: none;
}
</style>
