<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import IconTrash from '../../components/icons/IconTrash.vue';

const tags = ref<Array<{id: number, name: string}>>([]);
const isLoading = ref(true);
const newTagName = ref('');
const editingTagId = ref<number | null>(null);
const editTagName = ref('');

async function loadTags() {
  isLoading.value = true;
  try {
    tags.value = await invoke('get_tags');
  } catch (err) {
    console.error('Failed to load tags:', err);
  } finally {
    isLoading.value = false;
  }
}

async function createTag() {
  if (!newTagName.value.trim()) return;
  
  try {
    await invoke('create_tag', { name: newTagName.value.trim() });
    newTagName.value = '';
    await loadTags();
  } catch (err) {
    alert('创建失败: ' + err);
  }
}

function startEdit(tag: {id: number, name: string}) {
  editingTagId.value = tag.id;
  editTagName.value = tag.name;
}

function cancelEdit() {
  editingTagId.value = null;
  editTagName.value = '';
}

async function saveEdit() {
  if (!editTagName.value.trim() || !editingTagId.value) return;
  
  try {
    await invoke('update_tag', { 
      id: editingTagId.value, 
      name: editTagName.value.trim() 
    });
    editingTagId.value = null;
    await loadTags();
  } catch (err) {
    alert('更新失败: ' + err);
  }
}

async function deleteTag(id: number) {
  if (!confirm('确定要删除这个标签吗？关于该标签的关联也会被删除')) return;
  
  try {
    await invoke('delete_tag', { id });
    await loadTags();
  } catch (err) {
    alert('删除失败: ' + err);
  }
}

onMounted(() => {
  loadTags();
});
</script>

<template>
  <div class="page-container">
    <div class="create-section">
      <input 
        v-model="newTagName" 
        placeholder="输入新标签名称..." 
        @keyup.enter="createTag"
      />
      <button class="btn-primary" @click="createTag" :disabled="!newTagName.trim()">
        添加
      </button>
    </div>

    <div v-if="isLoading" class="loading-state">
      加载中...
    </div>
    
    <div v-else-if="tags.length === 0" class="empty-state">
      <p>还没有标签</p>
    </div>
    
    <div v-else class="tags-list">
      <div v-for="tag in tags" :key="tag.id" class="tag-item">
        <template v-if="editingTagId === tag.id">
          <input 
            v-model="editTagName" 
            class="edit-input"
            @keyup.enter="saveEdit"
            @keyup.esc="cancelEdit"
            autofocus
          />
          <button class="btn-primary btn-sm" @click="saveEdit">保存</button>
          <button class="btn-secondary btn-sm" @click="cancelEdit">取消</button>
        </template>
        <template v-else>
          <span class="tag-name" @click="startEdit(tag)">{{ tag.name }}</span>
          <div class="tag-actions">
            <button class="icon-btn danger" @click="deleteTag(tag.id)" title="删除标签">
              <IconTrash :size="16" />
            </button>
          </div>
        </template>
      </div>
    </div>
  </div>
</template>

<style scoped>
.page-container {
  padding: var(--space-lg);
  max-width: 600px;
  margin: 0 auto;
}

.create-section {
  display: flex;
  gap: var(--space-md);
  margin-bottom: var(--space-xl);
}
.create-section input {
  flex: 1;
}

.tags-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-sm);
}

.tag-item {
  display: flex;
  align-items: center;
  gap: var(--space-md);
  padding: var(--space-sm) var(--space-md);
  background: var(--color-bg);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  transition: all var(--transition-fast);
}

.tag-item:hover {
  border-color: var(--color-primary-light);
}

.tag-name {
  flex: 1;
  font-weight: 500;
  cursor: pointer;
}
.tag-name:hover {
  color: var(--color-primary);
}

.tag-actions {
  display: flex;
  opacity: 0;
  transition: opacity var(--transition-fast);
}

.tag-item:hover .tag-actions {
  opacity: 1;
}

/* Show actions on mobile devices automatically */
@media (max-width: 768px) {
  .tag-actions {
    opacity: 1;
  }
}

.icon-btn {
  padding: var(--space-xs);
  color: var(--color-text-tertiary);
  border-radius: var(--radius-sm);
}
.icon-btn:hover {
  background: var(--color-surface);
  color: var(--color-text);
}
.icon-btn.danger:hover {
  background: var(--color-danger-bg);
  color: var(--color-danger);
}

.edit-input {
  flex: 1;
  padding: 4px 8px;
}
.btn-sm {
  padding: 4px 12px;
  font-size: var(--font-xs);
}
</style>
