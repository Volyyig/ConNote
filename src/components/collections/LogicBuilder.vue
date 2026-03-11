<script lang="ts">
export default {
  name: 'LogicBuilder'
}
</script>

<script setup lang="ts">
interface NodeRule {
  type: string;
  tag_id?: number;
  children?: NodeRule[];
  child?: NodeRule;
}

const props = defineProps<{
  node: NodeRule;
  availableTags: Array<{id: number, name: string}>;
  isRoot?: boolean;
}>();

const emit = defineEmits<{
  (e: 'update:node', value: NodeRule): void
}>();

function updateNode(newNode: NodeRule) {
  emit('update:node', newNode);
}

function changeType(newType: string) {
  if (newType === 'tag') {
    updateNode({ type: 'tag', tag_id: props.availableTags[0]?.id || 0 });
  } else if (newType === 'not') {
    updateNode({ type: 'not', child: { type: 'tag', tag_id: props.availableTags[0]?.id || 0 } });
  } else {
    updateNode({ type: newType, children: [] });
  }
}

function updateTagId(e: Event) {
  const target = e.target as HTMLSelectElement;
  updateNode({ ...props.node, tag_id: parseInt(target.value) });
}

function updateChild(index: number, newChild: NodeRule) {
  if (!props.node.children) return;
  const newChildren = [...props.node.children];
  newChildren[index] = newChild;
  updateNode({ ...props.node, children: newChildren });
}

function removeChild(index: number) {
  if (!props.node.children) return;
  const newChildren = [...props.node.children];
  newChildren.splice(index, 1);
  updateNode({ ...props.node, children: newChildren });
}

function addChild() {
  if (!props.node.children) return;
  const newChildren = [...props.node.children, { type: 'tag', tag_id: props.availableTags[0]?.id || 0 }];
  updateNode({ ...props.node, children: newChildren });
}

function updateNotChild(newChild: NodeRule) {
  updateNode({ ...props.node, child: newChild });
}
</script>

<template>
    <div class="logic-node" :class="{ 'is-root': isRoot }">
      <div class="node-header">
        <select class="type-select" :value="node.type" @change="changeType(($event.target as HTMLSelectElement).value)">
          <option value="and">与 (AND)</option>
          <option value="or">或 (OR)</option>
          <option value="not">非 (NOT)</option>
          <option value="tag">标签 (TAG)</option>
        </select>

        <template v-if="node.type === 'tag'">
          <select class="tag-select" :value="node.tag_id" @change="updateTagId">
            <option v-for="tag in availableTags" :key="tag.id" :value="tag.id">
              {{ tag.name }}
            </option>
          </select>
        </template>
      </div>

      <!-- Children for AND / OR -->
      <div v-if="node.type === 'and' || node.type === 'or'" class="node-children">
        <div v-for="(child, idx) in node.children" :key="idx" class="child-wrapper">
          <LogicBuilder :node="child" :availableTags="availableTags" @update:node="updateChild(idx, $event)" />
          <button class="remove-btn" @click="removeChild(idx)" title="移除此条件">×</button>
        </div>
        <button class="add-btn btn-secondary btn-sm" @click="addChild">+ 添加条件</button>
      </div>

      <!-- Child for NOT -->
    <div v-if="node.type === 'not' && node.child" class="node-children">
      <LogicBuilder 
        :node="node.child" 
        :availableTags="availableTags" 
        @update:node="updateNotChild"
      />
    </div>
    </div>
  </template>

  <style scoped>
  .logic-node {
    padding: var(--space-md);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    background: var(--color-bg);
    margin-bottom: var(--space-sm);
  }

  .logic-node.is-root {
    border-color: var(--color-primary-light);
    background: var(--color-surface);
  }

  .node-header {
    display: flex;
    gap: var(--space-sm);
    align-items: center;
    margin-bottom: var(--space-sm);
  }

  .type-select,
  .tag-select {
    padding: 4px 8px;
    border-radius: var(--radius-sm);
    border: 1px solid var(--color-border);
    background: var(--color-bg);
    font-size: var(--font-sm);
    color: var(--color-text);
  }

  .node-children {
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
    padding-left: var(--space-xl);
    border-left: 2px dashed var(--color-border-light);
    margin-left: var(--space-sm);
  }

  .child-wrapper {
    display: flex;
    align-items: flex-start;
    gap: var(--space-sm);
  }

  .child-wrapper>.logic-node {
    flex: 1;
    margin-bottom: 0;
  }

  .remove-btn {
    width: 28px;
    height: 28px;
    border-radius: var(--radius-sm);
    background: var(--color-danger-bg);
    color: var(--color-danger);
    font-size: var(--font-lg);
    display: flex;
    align-items: center;
    justify-content: center;
    margin-top: var(--space-sm);
    cursor: pointer;
    border: none;
  }

  .remove-btn:hover {
    background: #FEE2E2;
  }

  .add-btn {
    align-self: flex-start;
    margin-top: var(--space-sm);
  }

  .btn-sm {
    padding: 4px 12px;
    font-size: var(--font-xs);
  }
</style>
