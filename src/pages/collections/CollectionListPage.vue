<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import IconPlus from '../../components/icons/IconPlus.vue';
import IconEdit from '../../components/icons/IconEdit.vue';
import IconTrash from '../../components/icons/IconTrash.vue';

const router = useRouter();
const collections = ref<Array<any>>([]);
const isLoading = ref(true);

async function loadCollections() {
  isLoading.value = true;
  try {
    collections.value = await invoke('get_collections');
  } catch (err) {
    console.error('Failed to load collections:', err);
  } finally {
    isLoading.value = false;
  }
}

function createCollection() {
  router.push('/collections/new');
}

function editCollection(id: number) {
  router.push(`/collections/${id}/edit`);
}

function viewResults(id: number) {
  router.push(`/collections/${id}/results`);
}

async function deleteCollection(id: number) {
  if (!confirm('确定要删除这个合集吗？该操作不会删除合集中的笔记。')) return;
  try {
    await invoke('delete_collection', { id });
    await loadCollections();
  } catch (err) {
    alert('删除失败: ' + err);
  }
}

onMounted(() => {
  loadCollections();
});
</script>

<template>
  <div class="page-container">
    <div v-if="isLoading" class="loading-state">加载中...</div>
    
    <div v-else-if="collections.length === 0" class="empty-state">
      <IconPlus :size="48" />
      <p>还没有合集，点击右下角按钮新建</p>
    </div>
    
    <div v-else class="collections-list">
      <div 
        v-for="col in collections" 
        :key="col.id" 
        class="collection-card"
        @click="viewResults(col.id)"
      >
        <div class="collection-content">
          <h3 class="collection-title">{{ col.name }}</h3>
          <p class="collection-rule">规则已设定</p>
        </div>
        <div class="collection-actions" @click.stop>
          <button class="icon-btn" @click="editCollection(col.id)" title="编辑合集">
            <IconEdit :size="18" />
          </button>
          <button class="icon-btn danger" @click="deleteCollection(col.id)" title="删除合集">
            <IconTrash :size="18" />
          </button>
        </div>
      </div>
    </div>

    <button class="fab" @click="createCollection" title="新建合集">
      <IconPlus :size="24" />
    </button>
  </div>
</template>

<style scoped>
.page-container {
  padding: var(--space-lg);
  height: 100%;
}

.collections-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-md);
  padding-bottom: 80px;
}

.collection-card {
  display: flex;
  align-items: center;
  padding: var(--space-lg);
  background: var(--color-bg);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-sm);
  cursor: pointer;
  transition: all var(--transition-fast);
}

.collection-card:hover {
  border-color: var(--color-primary-light);
  box-shadow: var(--shadow-md);
  transform: translateY(-2px);
}

.collection-content {
  flex: 1;
}

.collection-title {
  font-size: var(--font-lg);
  font-weight: 600;
  margin: 0 0 var(--space-xs);
  color: var(--color-text);
}

.collection-rule {
  font-size: var(--font-sm);
  color: var(--color-text-secondary);
  margin: 0;
}

.collection-actions {
  display: flex;
  gap: var(--space-sm);
  opacity: 0;
  transition: opacity var(--transition-fast);
}

.collection-card:hover .collection-actions {
  opacity: 1;
}

@media (max-width: 768px) {
  .collection-actions {
    opacity: 1;
  }
}

.icon-btn {
  padding: var(--space-sm);
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
</style>
