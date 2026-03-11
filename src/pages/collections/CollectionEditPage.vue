<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import LogicBuilder from '../../components/collections/LogicBuilder.vue';

const route = useRoute();
const router = useRouter();

const isNew = route.name === 'CollectionCreate';
const collectionId = !isNew ? parseInt(route.params.id as string) : null;

const name = ref('');
// Default to empty AND node
const rule = ref<any>({ type: 'and', children: [] });

const availableTags = ref<Array<{id: number, name: string}>>([]);
const isSaving = ref(false);

async function loadData() {
  try {
    availableTags.value = await invoke('get_tags');
    
    if (!isNew && collectionId) {
      const collections: any[] = await invoke('get_collections');
      const collection = collections.find(c => c.id === collectionId);
      if (collection) {
        name.value = collection.name;
        rule.value = JSON.parse(collection.rule_json);
      }
    }
  } catch (err) {
    console.error('Failed to load data:', err);
  }
}

async function save() {
  if (!name.value.trim()) {
    alert('请输入合集名称');
    return;
  }

  isSaving.value = true;
  try {
    const ruleJson = JSON.stringify(rule.value);
    
    if (isNew) {
      await invoke('create_collection', {
        name: name.value.trim(),
        ruleJson
      });
    } else {
      await invoke('update_collection', {
        id: collectionId,
        name: name.value.trim(),
        ruleJson
      });
    }
    router.back();
  } catch (err) {
    console.error('Failed to save collection:', err);
    alert('保存失败: ' + err);
  } finally {
    isSaving.value = false;
  }
}

onMounted(() => {
  loadData();
});
</script>

<template>
  <div class="edit-container">
    <div class="actions-bar">
      <button class="btn-primary" @click="save" :disabled="isSaving || availableTags.length === 0">
        {{ isSaving ? '保存中...' : '保存' }}
      </button>
    </div>

    <input 
      v-model="name" 
      class="name-input" 
      placeholder="输入合集名称..." 
      autofocus
    />
    
    <div v-if="availableTags.length === 0" class="empty-state">
      <p>还没有任何标签，无法创建合集规则。请先去添加标签。</p>
    </div>
    <div class="rule-builder-section" v-else>
      <h3 class="section-title">匹配规则</h3>
      <p class="section-desc">通过组合标签逻辑来动态筛选笔记</p>
      <LogicBuilder 
        v-model:node="rule" 
        :availableTags="availableTags" 
        :isRoot="true" 
      />
    </div>
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
}

.name-input {
  font-size: var(--font-2xl);
  font-weight: 700;
  border: none;
  border-bottom: 2px solid transparent;
  padding: var(--space-sm) 0;
  border-radius: 0;
  background: transparent;
  color: var(--color-text);
}
.name-input:focus {
  border-bottom-color: var(--color-primary);
  box-shadow: none;
}

.rule-builder-section {
  flex: 1;
  overflow-y: auto;
  padding: var(--space-md) 0;
}

.section-title {
  font-size: var(--font-lg);
  font-weight: 600;
  margin: 0 0 var(--space-xs);
  color: var(--color-text);
}

.section-desc {
  font-size: var(--font-sm);
  color: var(--color-text-secondary);
  margin: 0 0 var(--space-lg);
}
</style>
