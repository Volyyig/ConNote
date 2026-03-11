<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{
  note: {
    id: number;
    title: string;
    content: string;
    updated_at: string;
    tags: Array<{ id: number; name: string }>;
  };
}>();

const previewContent = computed(() => {
  if (!props.note.content) return '';
  const plain = props.note.content.substring(0, 100);
  return plain.length < props.note.content.length ? plain + '...' : plain;
});
</script>

<template>
  <div class="note-card" @click="$emit('click', note.id)">
    <h3 class="note-title">{{ note.title || '无标题' }}</h3>
    <p class="note-preview">{{ previewContent }}</p>
    
    <div class="note-footer">
      <div class="note-tags" v-if="note.tags && note.tags.length > 0">
        <span class="tag-chip" v-for="tag in note.tags" :key="tag.id">
          {{ tag.name }}
        </span>
      </div>
      <span class="note-date">{{ note.updated_at.split(' ')[0] }}</span>
    </div>
  </div>
</template>

<style scoped>
.note-card {
  padding: var(--space-lg);
  background: var(--color-bg);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-sm);
  cursor: pointer;
  transition: all var(--transition-fast);
  display: flex;
  flex-direction: column;
  gap: var(--space-sm);
}

.note-card:hover {
  border-color: var(--color-primary-light);
  box-shadow: var(--shadow-md);
  transform: translateY(-2px);
}

.note-title {
  font-size: var(--font-lg);
  font-weight: 600;
  color: var(--color-text);
  margin: 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.note-preview {
  font-size: var(--font-sm);
  color: var(--color-text-secondary);
  line-height: 1.5;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
  margin: 0;
}

.note-footer {
  display: flex;
  justify-content: space-between;
  align-items: flex-end;
  margin-top: var(--space-sm);
  gap: var(--space-md);
}

.note-tags {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-xs);
  flex: 1;
}

.note-date {
  font-size: var(--font-xs);
  color: var(--color-text-tertiary);
  white-space: nowrap;
}
</style>
